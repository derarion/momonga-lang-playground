use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::*;
use crate::data::*;
use crate::env::*;
use crate::error::EvalError;

type EvalStmtResult<'a> = Result<Option<Rc<RefCell<Value<'a>>>>, JumpStmt<'a>>;
type EvalExprResult<'a> = Result<Rc<RefCell<Value<'a>>>, JumpStmt<'a>>;

#[derive(Debug, PartialEq)]
pub enum JumpStmt<'a> {
    Continue,
    Break,
    Return(Rc<RefCell<Value<'a>>>),
    Error(EvalError),
}

const MAX_ABS_INT: u64 = i64::MIN.unsigned_abs(); // TODO: Reconsider how to handle value overflow

pub fn eval<'a>(
    program: &'a Program,
    env: Rc<RefCell<Env<'a>>>,
) -> Result<Option<Rc<RefCell<Value<'a>>>>, EvalError> {
    match eval_block_stmt(program, env) {
        Ok(val) => Ok(val),
        Err(JumpStmt::Error(eval_error)) => Err(eval_error),
        _ => unreachable!(),
    }
}

fn eval_block_stmt<'a>(block_stmt: &'a BlockStmt, env: Rc<RefCell<Env<'a>>>) -> EvalStmtResult<'a> {
    let mut result = Ok(None);

    for stmt in block_stmt {
        result = match stmt {
            Stmt::BlockStmt(block_stmt) => {
                let env_block = Rc::new(RefCell::new(Env::new(Some(Rc::clone(&env)))));
                eval_block_stmt(block_stmt, Rc::clone(&env_block))
            },
            Stmt::FuncDecl(func_decl) => eval_func_decl(func_decl, Rc::clone(&env)),
            Stmt::IfStmt(if_stmt) => eval_if_stmt(if_stmt, Rc::clone(&env)),
            Stmt::ForStmt(for_stmt) => eval_for_stmt(for_stmt, Rc::clone(&env)),
            Stmt::WhileStmt(_while_stmt) => todo!(),
            Stmt::VarStmt(var_stmt) => eval_var_stmt(var_stmt, Rc::clone(&env)),
            Stmt::ExprStmt(expr_stmt) => eval_expr_stmt(expr_stmt, Rc::clone(&env)),
            Stmt::ContinueStmt => Err(JumpStmt::Continue),
            Stmt::BreakStmt => Err(JumpStmt::Break),
            Stmt::ReturnStmt(return_stmt) => {
                let ReturnStmt { expr } = return_stmt;
                match expr {
                    Some(expr) => Err(JumpStmt::Return(eval_expr(expr, Rc::clone(&env))?)),
                    None => Err(JumpStmt::Return(Rc::new(RefCell::new(Value::None)))),
                }
            }
        };

        if let Err(JumpStmt::Continue)
        | Err(JumpStmt::Break)
        | Err(JumpStmt::Return(_))
        | Err(JumpStmt::Error(_)) = &result
        {
            return result;
        };
    }

    result
}

fn eval_func_decl<'a>(func_decl: &'a FuncDecl, env: Rc<RefCell<Env<'a>>>) -> EvalStmtResult<'a> {
    let FuncDecl {
        ident_func,
        ident_param,
        block,
    } = func_decl;
    let Ident { name } = ident_func;

    env.borrow_mut().set(
        name,
        Rc::new(RefCell::new(Value::Func {
            params: ident_param,
            block,
        })),
    );

    Ok(None)
}

fn eval_if_stmt<'a>(if_stmt: &'a IfStmt, env: Rc<RefCell<Env<'a>>>) -> EvalStmtResult<'a> {
    let IfStmt {
        condition,
        block,
        else_clause,
    } = if_stmt;

    let env_block = Rc::new(RefCell::new(Env::new(Some(Rc::clone(&env)))));

    match *eval_expr(condition, Rc::clone(&env))?.borrow() {
        Value::Bool(bool) => {
            if bool {
                eval_block_stmt(block, env_block)
            } else if let Some(if_stmt_else_clause) = else_clause {
                eval_if_stmt_else_clause(if_stmt_else_clause, env_block)
            } else {
                Ok(None)
            }
        }
        _ => {
            Err(JumpStmt::Error(EvalError::Type)) // Condition type must be bool
        }
    }
}

fn eval_if_stmt_else_clause<'a>(
    if_stmt_else_clause: &'a IfStmtElseClause,
    env: Rc<RefCell<Env<'a>>>,
) -> EvalStmtResult<'a> {
    match if_stmt_else_clause {
        IfStmtElseClause::IfStmtBlock(stmts) => eval_block_stmt(stmts, env),
        IfStmtElseClause::IfStmt(if_stmt) => eval_if_stmt(if_stmt, env),
    }
}

fn eval_for_stmt<'a>(for_stmt: &'a ForStmt, env: Rc<RefCell<Env<'a>>>) -> EvalStmtResult<'a> {
    let ForStmt {
        init,
        cond,
        afterthought,
        block,
    } = for_stmt;
    let env_block = Rc::new(RefCell::new(Env::new(Some(Rc::clone(&env)))));

    match init {
        Some(ForStmtInit::Var(var_stmt)) => eval_var_stmt(var_stmt, Rc::clone(&env_block))?,
        Some(ForStmtInit::Expr(expr_stmt)) => eval_expr_stmt(expr_stmt, env)?,
        _ => todo!(), // TODO: Define how to handle this case
    };

    let mut result = Ok(None);
    loop {
        let cond = match cond {
            Some(cond) => match *eval_expr(cond, Rc::clone(&env_block))?.borrow() {
                Value::Bool(bool) => bool,
                _ => todo!(), // TODO: Define how to handle this case
            },
            None => false,
        };

        if !cond {
            break;
        }

        result = match eval_block_stmt(block, Rc::clone(&env_block)) {
            Err(JumpStmt::Continue) => {
                eval_for_stmt_afterthought(afterthought, Rc::clone(&env_block))?;
                result = Ok(None);
                continue;
            }
            Err(JumpStmt::Break) => {
                return Ok(None);
            }
            default => default,
        };

        eval_for_stmt_afterthought(afterthought, Rc::clone(&env_block))?;
    }
    result
}

fn eval_for_stmt_afterthought<'a>(
    for_stmt_afterthought: &'a Option<Expr>,
    env: Rc<RefCell<Env<'a>>>,
) -> EvalStmtResult<'a> {
    if let Some(for_stmt_afterthought) = for_stmt_afterthought {
        eval_expr_stmt(for_stmt_afterthought, Rc::clone(&env))?;
    };
    Ok(None)
}

fn eval_var_stmt<'a>(var_stmt: &'a VarStmt, env: Rc<RefCell<Env<'a>>>) -> EvalStmtResult<'a> {
    let VarStmt {
        ident: Ident { name },
        expr,
    } = var_stmt;
    let value = if let Some(expr) = expr {
        eval_expr(expr, Rc::clone(&env))?
    } else {
        Rc::new(RefCell::new(Value::None))
    };
    env.borrow_mut().set_var(name, value);

    Ok(None)
}

fn eval_expr_stmt<'a>(expr_stmt: &'a ExprStmt, env: Rc<RefCell<Env<'a>>>) -> EvalStmtResult<'a> {
    Ok(Some(eval_expr(expr_stmt, env)?))
}

fn eval_expr<'a>(expr: &'a Expr, env: Rc<RefCell<Env<'a>>>) -> EvalExprResult<'a> {
    match expr {
        Expr::Literal(literal) => eval_literal(literal, env),
        Expr::Ident(ident) => {
            let val = eval_ident(ident, env)?;
            if let Value::Func { .. } = *val.borrow() {
                return Err(JumpStmt::Error(EvalError::InvalidExpression)); // Function is not an expression
            };
            Ok(val)
        }
        Expr::PrefixOp { kind, rhs } => {
            match kind {
                PrefixOpKind::Pos => {
                    match *eval_expr(rhs, env)?.borrow() {
                        Value::Int(int) => Ok(Rc::new(RefCell::new(Value::Int(int)))),
                        _ => Err(JumpStmt::Error(EvalError::Type)), // Incorrect operand
                    }
                }
                PrefixOpKind::Neg => {
                    if let Expr::Literal(Literal::Int(int)) = **rhs {
                        if int == MAX_ABS_INT {
                            return Ok(Rc::new(RefCell::new(Value::Int(i64::MIN))));
                        };
                    };
                    match *eval_expr(rhs, env)?.borrow() {
                        Value::Int(int) => {
                            if int == i64::MIN {
                                return Err(JumpStmt::Error(EvalError::OutOfRange));
                                // Attempt to nagate i64 min
                            }
                            Ok(Rc::new(RefCell::new(Value::Int(-int))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)), // Incorrect operand
                    }
                }
                PrefixOpKind::Not => {
                    match *eval_expr(rhs, env)?.borrow() {
                        Value::Bool(bool) => Ok(Rc::new(RefCell::new(Value::Bool(!bool)))),
                        _ => Err(JumpStmt::Error(EvalError::Type)), // Incorrect operand
                    }
                }
            }
        }
        Expr::InfixOp { kind, lhs, rhs } => {
            match kind {
                InfixOpKind::Add => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Int(lhs + rhs))))
                        }
                        (Value::String(lhs), Value::String(rhs)) => {
                            let mut lhs = lhs.borrow_mut();
                            let rhs = rhs.borrow();
                            lhs.push_str(&rhs);
                            Ok(Rc::new(RefCell::new(Value::String(RefCell::new(
                                lhs.clone(),
                            )))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::Subtract => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Int(lhs - rhs))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::Multiply => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Int(lhs * rhs))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::Divide => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            if *rhs == 0 {
                                return Err(JumpStmt::Error(EvalError::ZeroDivision));
                            }
                            Ok(Rc::new(RefCell::new(Value::Int(lhs / rhs))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::Modulo => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            if *rhs == 0 {
                                return Err(JumpStmt::Error(EvalError::ZeroDivision));
                            }
                            Ok(Rc::new(RefCell::new(Value::Int(lhs % rhs))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::Eq => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Bool(lhs), Value::Bool(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs == rhs))))
                        }
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs == rhs))))
                        }
                        (Value::String(lhs), Value::String(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs == rhs))))
                        }
                        (Value::Array(lhs), Value::Array(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs == rhs))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::NotEq => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Bool(lhs), Value::Bool(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs != rhs))))
                        }
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs != rhs))))
                        }
                        (Value::String(lhs), Value::String(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs != rhs))))
                        }
                        (Value::Array(lhs), Value::Array(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs != rhs))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::Gt => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs > rhs))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::Ge => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs >= rhs))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::Lt => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs < rhs))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::Le => {
                    match (
                        &*eval_expr(lhs, Rc::clone(&env))?.borrow(),
                        &*eval_expr(rhs, Rc::clone(&env))?.borrow(),
                    ) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            Ok(Rc::new(RefCell::new(Value::Bool(lhs <= rhs))))
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    }
                }
                InfixOpKind::And => match &*eval_expr(lhs, Rc::clone(&env))?.borrow() {
                    Value::Bool(false) => Ok(Rc::new(RefCell::new(Value::Bool(false)))), // FIXME: false && "foo" returns false, but it should return type error
                    Value::Bool(true) => match &*eval_expr(rhs, Rc::clone(&env))?.borrow() {
                        Value::Bool(false) => Ok(Rc::new(RefCell::new(Value::Bool(false)))),
                        Value::Bool(true) => Ok(Rc::new(RefCell::new(Value::Bool(true)))),
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    },
                    _ => Err(JumpStmt::Error(EvalError::Type)),
                },
                InfixOpKind::Or => match &*eval_expr(lhs, Rc::clone(&env))?.borrow() {
                    Value::Bool(true) => Ok(Rc::new(RefCell::new(Value::Bool(true)))), // FIXME: true || "foo" returns true, but it should return type error
                    Value::Bool(false) => match *eval_expr(rhs, Rc::clone(&env))?.borrow() {
                        Value::Bool(true) => Ok(Rc::new(RefCell::new(Value::Bool(true)))),
                        Value::Bool(false) => Ok(Rc::new(RefCell::new(Value::Bool(false)))),
                        _ => Err(JumpStmt::Error(EvalError::Type)),
                    },
                    _ => Err(JumpStmt::Error(EvalError::Type)),
                },
                InfixOpKind::Assign => {
                    match **lhs {
                        Expr::Ident(Ident { ref name }) => {
                            let value = eval_expr(rhs, Rc::clone(&env))?;
                            if let Err(err) = env.borrow_mut().set_assign(name, Rc::clone(&value)) {
                                return Err(JumpStmt::Error(err));
                            };
                            Ok(value)
                        }
                        // Expr::PostfixOp { kind: _, lhs: _ } => todo!(), // TODO: Assign value to an element of array directly
                        _ => Err(JumpStmt::Error(EvalError::Type)), // TODO: Generate syntax error before reaching here
                    }
                }
            }
        }
        Expr::PostfixOp { kind, lhs } => {
            match kind {
                PostfixOpKind::Index(index_expr) => {
                    let index = match *eval_expr(index_expr, Rc::clone(&env))?.borrow() {
                        Value::Int(idx) => {
                            if idx < 0 {
                                return Err(JumpStmt::Error(EvalError::Index)); // Index must be a non-negative Integer value
                            }
                            idx as usize
                        }
                        _ => return Err(JumpStmt::Error(EvalError::Type)), // Index must be Integer type
                    };
                    match **lhs {
                        Expr::Literal(Literal::Array(ref exprs)) => match exprs.get(index) {
                            Some(expr) => eval_expr(expr, Rc::clone(&env)),
                            None => Err(JumpStmt::Error(EvalError::Index)), // Index out of range
                        },
                        ref expr => {
                            if let Value::Array(Array(ref vals)) =
                                *eval_expr(expr, Rc::clone(&env))?.borrow()
                            {
                                if let Some(val) = vals.borrow().get(index) {
                                    Ok(Rc::clone(val))
                                } else {
                                    Err(JumpStmt::Error(EvalError::Index)) // Index out of range
                                }
                            } else {
                                Err(JumpStmt::Error(EvalError::Type)) // Operand is not subscriptable
                            }
                        }
                    }
                }
                PostfixOpKind::Call(args) => {
                    let Expr::Ident(ref ident) = **lhs else {
                        return Err(JumpStmt::Error(EvalError::Type)); // Operand is not callable
                    };
                    let res = match *eval_ident(ident, Rc::clone(&env))?.borrow() {
                        Value::Func { params, block } => {
                            let evaluated_args = args
                                .iter()
                                .map(|arg| eval_expr(arg, Rc::clone(&env)))
                                .collect::<Result<Vec<Rc<RefCell<Value<'a>>>>, JumpStmt>>()?;
                            let env_block = Rc::new(RefCell::new(Env::new(Some(Rc::clone(&env)))));

                            for (ident, val) in params.iter().zip(evaluated_args.iter()) {
                                env_block.borrow_mut().set_var(&ident.name, Rc::clone(val))
                            }

                            match eval_block_stmt(block, env_block) {
                                Ok(_) => Ok(Rc::new(RefCell::new(Value::None))),
                                Err(JumpStmt::Return(val)) => Ok(val),
                                Err(default) => Err(default),
                            }
                        }
                        Value::Builtin(args_cnt, builin_func) => {
                            // Incorrect number of arguments
                            if args.len() as i64 != args_cnt {
                                return Err(JumpStmt::Error(EvalError::Argument));
                            };

                            let mut evaluated_args = vec![];
                            for arg in args {
                                evaluated_args.push(eval_expr(arg, Rc::clone(&env))?)
                            }
                            match builin_func(evaluated_args) {
                                Ok(val) => Ok(val),
                                Err(eval_error) => Err(JumpStmt::Error(eval_error)),
                            }
                        }
                        _ => Err(JumpStmt::Error(EvalError::Type)), // Operand is not callable
                    };
                    res
                }
            }
        }
    }
}

fn eval_literal<'a>(literal: &'a Literal, env: Rc<RefCell<Env<'a>>>) -> EvalExprResult<'a> {
    match literal {
        Literal::Bool(bool) => Ok(Rc::new(RefCell::new(Value::Bool(*bool)))),
        Literal::Int(int) => {
            if *int >= MAX_ABS_INT {
                return Err(JumpStmt::Error(EvalError::OutOfRange));
            }
            Ok(Rc::new(RefCell::new(Value::Int(*int as i64))))
        }
        Literal::String(string) => Ok(Rc::new(RefCell::new(Value::String(RefCell::new(
            string.clone(),
        ))))),
        Literal::Array(exprs) => {
            let mut vals = vec![];
            for expr in exprs {
                vals.push(eval_expr(expr, Rc::clone(&env))?);
            }
            Ok(Rc::new(RefCell::new(Value::Array(Array(RefCell::new(
                vals,
            ))))))
        }
        Literal::None => Ok(Rc::new(RefCell::new(Value::None))),
    }
}

fn eval_ident<'a>(ident: &'a Ident, env: Rc<RefCell<Env<'a>>>) -> EvalExprResult<'a> {
    match env.borrow().get(&ident.name) {
        Ok(val) => Ok(val),
        Err(eval_error) => Err(JumpStmt::Error(eval_error)),
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::*;
    #[test]
    fn stmt_is_evaluated_correctly() {
        // Program
        assert_eq!(
            eval(
                // Empty program
                &vec![],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(None)
        );
        // BlockStmt
        assert_eq!(
            eval(
                // {}
                &vec![Stmt::BlockStmt(vec![])],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(None)
        );
        // FuncDecl
        assert_eq!(
            eval(
                // func foo() {}
                &vec![Stmt::FuncDecl(FuncDecl {
                    ident_func: Ident {
                        name: "foo".to_string(),
                    },
                    ident_param: vec![],
                    block: vec![],
                })],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(None)
        );
        // IfStmt
        assert_eq!(
            eval(
                // if (true) {}
                &vec![Stmt::IfStmt(IfStmt {
                    condition: Expr::literal_bool(true),
                    block: vec![Stmt::IfStmt(IfStmt {
                        condition: Expr::literal_bool(true),
                        block: vec![],
                        else_clause: None,
                    })],
                    else_clause: None,
                })],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(None)
        );
        assert_eq!(
            eval(
                // if (true) { none; }
                &vec![Stmt::IfStmt(IfStmt {
                    condition: Expr::literal_bool(true),
                    block: vec![Stmt::IfStmt(IfStmt {
                        condition: Expr::literal_bool(true),
                        block: vec![Stmt::ExprStmt(Expr::literal_none())],
                        else_clause: None,
                    })],
                    else_clause: None,
                })],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::None))))
        );
        // ForStmt
        assert_eq!(
            eval(
                // for (var i = 0; i < 10; i = i + 1) {}
                &vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Var(VarStmt {
                        ident: Ident {
                            name: "i".to_string(),
                        },
                        expr: Some(Expr::literal_int(1)),
                    })),
                    cond: Some(Expr::InfixOp {
                        kind: InfixOpKind::Lt,
                        lhs: Box::new(Expr::Ident(Ident {
                            name: "i".to_string()
                        })),
                        rhs: Box::new(Expr::literal_int(10))
                    }),
                    afterthought: Some(Expr::InfixOp {
                        kind: InfixOpKind::Assign,
                        lhs: Box::new(Expr::Ident(Ident {
                            name: "i".to_string()
                        })),
                        rhs: Box::new(Expr::InfixOp {
                            kind: InfixOpKind::Add,
                            lhs: Box::new(Expr::Ident(Ident {
                                name: "i".to_string()
                            })),
                            rhs: Box::new(Expr::Literal(Literal::Int(1)))
                        })
                    }),
                    block: vec![],
                })],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(None)
        );
        assert_eq!(
            eval(
                // for (var i = 0; i < 10; i = i + 1) { i; }
                &vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Var(VarStmt {
                        ident: Ident {
                            name: "i".to_string(),
                        },
                        expr: Some(Expr::literal_int(1)),
                    })),
                    cond: Some(Expr::InfixOp {
                        kind: InfixOpKind::Lt,
                        lhs: Box::new(Expr::Ident(Ident {
                            name: "i".to_string()
                        })),
                        rhs: Box::new(Expr::literal_int(10))
                    }),
                    afterthought: Some(Expr::InfixOp {
                        kind: InfixOpKind::Assign,
                        lhs: Box::new(Expr::Ident(Ident {
                            name: "i".to_string()
                        })),
                        rhs: Box::new(Expr::InfixOp {
                            kind: InfixOpKind::Add,
                            lhs: Box::new(Expr::Ident(Ident {
                                name: "i".to_string()
                            })),
                            rhs: Box::new(Expr::Literal(Literal::Int(1)))
                        })
                    }),
                    block: vec![Stmt::ExprStmt(Expr::Ident(Ident {
                        name: "i".to_string()
                    }))],
                })],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::Int(9)))))
        );
        // VarStmt
        assert_eq!(
            eval(
                // var x;
                &vec![Stmt::VarStmt(VarStmt {
                    ident: Ident {
                        name: "x".to_string(),
                    },
                    expr: None,
                })],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(None)
        );
        assert_eq!(
            eval(
                // var x = 1;
                &vec![Stmt::VarStmt(VarStmt {
                    ident: Ident {
                        name: "x".to_string(),
                    },
                    expr: Some(Expr::literal_int(1)),
                })],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(None)
        );
    }

    #[test]
    fn ident_expression_is_evaluated_correctly() {
        assert_eq!(
            eval(
                // var x; // No initialization
                // x;
                &vec![
                    Stmt::VarStmt(VarStmt {
                        ident: Ident {
                            name: "x".to_string(),
                        },
                        expr: None,
                    }),
                    Stmt::ExprStmt(Expr::Ident(Ident {
                        name: "x".to_string()
                    }))
                ],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::None))))
        );
        assert_eq!(
            eval(
                // var x = 1;
                // x;
                &vec![
                    Stmt::VarStmt(VarStmt {
                        ident: Ident {
                            name: "x".to_string(),
                        },
                        expr: Some(Expr::literal_int(1)),
                    }),
                    Stmt::ExprStmt(Expr::Ident(Ident {
                        name: "x".to_string()
                    }))
                ],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::Int(1)))))
        );
    }

    #[test]
    fn literal_ast_is_evaluated_correctly() {
        // Boolean
        assert_eq!(
            eval(
                &vec![Stmt::ExprStmt(Expr::literal_bool(true))], // true
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::Bool(true)))))
        );
        assert_eq!(
            eval(
                &vec![Stmt::ExprStmt(Expr::literal_bool(false))], // false
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::Bool(false)))))
        );
        // Integer
        assert_eq!(
            eval(
                &vec![Stmt::ExprStmt(Expr::literal_int(0))], // 0
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::Int(0)))))
        );
        assert_eq!(
            eval(
                &vec![Stmt::ExprStmt(Expr::literal_int(1))], // 1
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::Int(1)))))
        );
        assert_eq!(
            eval(
                &vec![Stmt::ExprStmt(Expr::literal_int(9223372036854775807))], // 9223372036854775807
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::Int(std::i64::MAX)))))
        );
        // String
        assert_eq!(
            eval(
                &vec![Stmt::ExprStmt(Expr::literal_string("foo".to_string()))], // "foo"
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::String(RefCell::new(
                "foo".to_string()
            ))))))
        );
        // Array
        assert_eq!(
            eval(
                &vec![Stmt::ExprStmt(Expr::literal_array(vec![]))], // []
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::Array(Array(
                RefCell::new(vec![])
            ))))))
        );
        assert_eq!(
            eval(
                &vec![Stmt::ExprStmt(Expr::literal_array(vec![
                    // [1, 2, 3]
                    Expr::literal_int(1),
                    Expr::literal_int(2),
                    Expr::literal_int(3),
                ]))],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::Array(Array(
                RefCell::new(vec![
                    Rc::new(RefCell::new(Value::Int(1))),
                    Rc::new(RefCell::new(Value::Int(2))),
                    Rc::new(RefCell::new(Value::Int(3))),
                ])
            ))))))
        );
        // None
        assert_eq!(
            eval(
                &vec![Stmt::ExprStmt(Expr::literal_none())],
                Rc::new(RefCell::new(Env::new_with_builtins()))
            ),
            Ok(Some(Rc::new(RefCell::new(Value::None))))
        );
    }
}
