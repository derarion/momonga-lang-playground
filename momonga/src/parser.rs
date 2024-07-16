use pest::iterators::Pair;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::*;
use crate::error::ParseError;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(ASSIGN, Right))
            .op(Op::infix(OR, Left))
            .op(Op::infix(AND, Left))
            .op(Op::infix(EQ, Left) | Op::infix(NOT_EQ, Left) | Op::infix(GT, Left) | Op::infix(GE, Left) | Op::infix(LT, Left) | Op::infix(LE, Left))
            .op(Op::infix(ADD, Left) | Op::infix(SUBTRACT, Left))
            .op(Op::infix(MULTIPLY, Left) | Op::infix(DIVIDE, Left) | Op::infix(MODULO, Left))
            .op(Op::prefix(POS) | Op::prefix(NEG) | Op::prefix(NOT))
            .op(Op::postfix(INDEX) | Op::postfix(CALL) )
    };
}

#[derive(Parser)]
#[grammar = "momonga.pest"]
pub struct PestMomongaParser;

pub fn parse(source: &str) -> Result<Program, ParseError> {
    match PestMomongaParser::parse(Rule::program, source) {
        Ok(mut pairs) => {
            let program_pair = pairs.next().unwrap();
            let mut ast_builder = AstBuilder::new();
            Ok(ast_builder.program(program_pair)?)
        }
        Err(_e) => Err(ParseError::PestParser),
    }
}

enum AstBuildFlow {
    Value,
    Break,
    Continue,
    Return,
}
struct AstBuilder {
    flow: AstBuildFlow,
}

impl AstBuilder {
    fn new() -> Self {
        Self {
            flow: AstBuildFlow::Value,
        }
    }

    pub fn program(&mut self, program_pair: Pair<Rule>) -> Result<Program, ParseError> {
        let mut program = Vec::new();
        for unknown_pair in program_pair.into_inner() {
            match unknown_pair.as_rule() {
                Rule::stmt => {
                    let stmt = self.stmt(unknown_pair)?;
                    if let AstBuildFlow::Continue = self.flow {
                        return Err(ParseError::BuildAst);
                    };
                    if let AstBuildFlow::Break = self.flow {
                        return Err(ParseError::BuildAst);
                    };
                    if let AstBuildFlow::Return = self.flow {
                        return Err(ParseError::BuildAst);
                    };
                    program.push(stmt);
                }
                Rule::EOI => break,
                _ => unreachable!(),
            };
        }
        Ok(program)
    }

    fn stmt(&mut self, stmt_pair: Pair<Rule>) -> Result<Stmt, ParseError> {
        let unknown_pair = stmt_pair.into_inner().next().unwrap();
        match unknown_pair.as_rule() {
            Rule::block_stmt => Ok(Stmt::BlockStmt(self.block_stmt(unknown_pair)?)),
            Rule::func_decl => Ok(Stmt::FuncDecl(self.func_decl(unknown_pair)?)),
            Rule::if_stmt => Ok(Stmt::IfStmt(self.if_stmt(unknown_pair)?)),
            Rule::for_stmt => Ok(Stmt::ForStmt(self.for_stmt(unknown_pair)?)),
            Rule::while_stmt => Ok(Stmt::WhileStmt(self.while_stmt(unknown_pair)?)),
            Rule::var_stmt => Ok(Stmt::VarStmt(self.var_stmt(unknown_pair)?)),
            Rule::expr => Ok(Stmt::ExprStmt(self.expr(unknown_pair)?)),
            Rule::continue_stmt => {
                self.flow = AstBuildFlow::Continue;
                Ok(Stmt::ContinueStmt)
            }
            Rule::break_stmt => {
                self.flow = AstBuildFlow::Break;
                Ok(Stmt::BreakStmt)
            }
            Rule::return_stmt => {
                self.flow = AstBuildFlow::Return;
                Ok(Stmt::ReturnStmt(self.return_stmt(unknown_pair)?))
            }
            _ => unreachable!(),
        }
    }

    fn block_stmt(&mut self, block_stmt_pair: Pair<Rule>) -> Result<BlockStmt, ParseError> {
        let mut block_stmt = vec![];
        for stmt_pair in block_stmt_pair.into_inner() {
            let stmt = self.stmt(stmt_pair)?;

            // Skip building ASTs after these control flow statements
            if let Stmt::ContinueStmt | Stmt::BreakStmt | Stmt::ReturnStmt(_) = stmt {
                block_stmt.push(stmt);
                break;
            }
            block_stmt.push(stmt)
        }
        Ok(block_stmt)
    }

    fn func_decl(&mut self, func_decl_pair: Pair<Rule>) -> Result<FuncDecl, ParseError> {
        let mut func_decl_inner = func_decl_pair.into_inner();
        let ident_func = self.ident(func_decl_inner.next().unwrap())?;

        let mut ident_param = vec![];
        loop {
            let unknown_pair = func_decl_inner.next().unwrap();
            match unknown_pair.as_rule() {
                Rule::IDENT => {
                    ident_param.push(self.ident(unknown_pair)?);
                }
                Rule::block_stmt => {
                    let block = self.func_block_stmt(unknown_pair)?;
                    return Ok(FuncDecl {
                        ident_func,
                        ident_param,
                        block,
                    });
                }
                _ => unreachable!(),
            }
        }
    }

    fn func_block_stmt(
        &mut self,
        func_block_stmt_pair: Pair<Rule>,
    ) -> Result<BlockStmt, ParseError> {
        let block_stmt = self.block_stmt(func_block_stmt_pair);
        if let AstBuildFlow::Return = self.flow {
            self.flow = AstBuildFlow::Value;
        };
        block_stmt
    }

    fn if_stmt(&mut self, if_pair: Pair<Rule>) -> Result<IfStmt, ParseError> {
        let mut if_pair_inner = if_pair.into_inner();
        let condition = if_pair_inner.next().map(|p| self.expr(p)).unwrap()?;
        let block = self.block_stmt(if_pair_inner.next().unwrap())?;
        match if_pair_inner.next() {
            Some(if_stmt_else_clause_pair) => {
                let unknown_pair = if_stmt_else_clause_pair.into_inner().next().unwrap();
                match unknown_pair.as_rule() {
                    Rule::block_stmt => {
                        let mut block_stmt = vec![];
                        for stmt_pair in unknown_pair.into_inner() {
                            block_stmt.push(self.stmt(stmt_pair)?)
                        }
                        Ok(IfStmt {
                            condition,
                            block,
                            else_clause: Some(IfStmtElseClause::IfStmtBlock(block_stmt)),
                        })
                    }
                    Rule::if_stmt => Ok(IfStmt {
                        condition,
                        block,
                        else_clause: Some(IfStmtElseClause::IfStmt(Box::new(
                            self.if_stmt(unknown_pair)?,
                        ))),
                    }),
                    _ => unreachable!(),
                }
            }
            None => Ok(IfStmt {
                condition,
                block,
                else_clause: None,
            }),
        }
    }

    fn for_stmt(&mut self, for_stmt_pair: Pair<Rule>) -> Result<ForStmt, ParseError> {
        // TODO: Refactor
        let mut for_stmt_inner = for_stmt_pair.into_inner();
        let unknown_pair = for_stmt_inner.next().unwrap();
        match unknown_pair.as_rule() {
            Rule::for_stmt_init => {
                let init = Some(self.for_stmt_init(unknown_pair)?);
                let unknown_pair = for_stmt_inner.next().unwrap();
                match unknown_pair.as_rule() {
                    Rule::for_stmt_cond => {
                        let cond = Some(self.expr(unknown_pair)?);
                        let unknown_pair = for_stmt_inner.next().unwrap();
                        match unknown_pair.as_rule() {
                            Rule::for_stmt_afterthought => {
                                let afterthought = Some(self.expr(unknown_pair)?);
                                let block =
                                    self.block_stmt_of_loop(for_stmt_inner.next().unwrap())?;
                                Ok(ForStmt {
                                    init,
                                    cond,
                                    afterthought,
                                    block,
                                })
                            }
                            Rule::block_stmt => Ok(ForStmt {
                                init,
                                cond,
                                afterthought: None,
                                block: self.block_stmt_of_loop(unknown_pair)?,
                            }),
                            _ => unreachable!(),
                        }
                    }
                    Rule::for_stmt_afterthought => {
                        let afterthought = Some(self.expr(unknown_pair)?);
                        let block = self.block_stmt_of_loop(for_stmt_inner.next().unwrap())?;
                        Ok(ForStmt {
                            init,
                            cond: None,
                            afterthought,
                            block,
                        })
                    }
                    Rule::block_stmt => Ok(ForStmt {
                        init,
                        cond: None,
                        afterthought: None,
                        block: self.block_stmt_of_loop(unknown_pair)?,
                    }),
                    _ => unreachable!(),
                }
            }
            Rule::for_stmt_cond => {
                let cond = Some(self.expr(unknown_pair)?);
                let unknown_pair = for_stmt_inner.next().unwrap();
                match unknown_pair.as_rule() {
                    Rule::for_stmt_afterthought => {
                        let afterthought = Some(self.expr(unknown_pair)?);
                        let block = self.block_stmt_of_loop(for_stmt_inner.next().unwrap())?;
                        Ok(ForStmt {
                            init: None,
                            cond,
                            afterthought,
                            block,
                        })
                    }
                    Rule::block_stmt => Ok(ForStmt {
                        init: None,
                        cond,
                        afterthought: None,
                        block: self.block_stmt_of_loop(unknown_pair)?,
                    }),
                    _ => unreachable!(),
                }
            }
            Rule::for_stmt_afterthought => {
                let afterthought = Some(self.expr(unknown_pair)?);
                let block = self.block_stmt_of_loop(for_stmt_inner.next().unwrap())?;
                Ok(ForStmt {
                    init: None,
                    cond: None,
                    afterthought,
                    block,
                })
            }
            Rule::block_stmt => Ok(ForStmt {
                init: None,
                cond: None,
                afterthought: None,
                block: self.block_stmt_of_loop(unknown_pair)?,
            }),
            _ => unreachable!(),
        }
    }

    fn for_stmt_init(&self, for_stmt_init_pair: Pair<Rule>) -> Result<ForStmtInit, ParseError> {
        let mut for_stmt_init_inner = for_stmt_init_pair.into_inner();
        let unknown_pair = for_stmt_init_inner.next().unwrap();
        if let Rule::IDENT = unknown_pair.as_rule() {
            let ident = self.ident(unknown_pair)?;
            let expr = Some(self.expr(for_stmt_init_inner.next().unwrap())?);
            Ok(ForStmtInit::Var(VarStmt { ident, expr }))
        } else {
            let expr = self.expr(unknown_pair)?;
            Ok(ForStmtInit::Expr(expr))
        }
    }

    fn block_stmt_of_loop(&mut self, block_stmt_pair: Pair<Rule>) -> Result<BlockStmt, ParseError> {
        let block_stmt = self.block_stmt(block_stmt_pair)?;
        if let AstBuildFlow::Continue | AstBuildFlow::Break = self.flow {
            self.flow = AstBuildFlow::Value;
        };
        Ok(block_stmt)
    }

    fn while_stmt(&mut self, while_stmt_pair: Pair<Rule>) -> Result<WhileStmt, ParseError> {
        let mut while_stmt_inner = while_stmt_pair.into_inner();
        let cond = while_stmt_inner.next().map(|p| self.expr(p)).unwrap()?;
        let block = while_stmt_inner.next().map(|p| self.block_stmt_of_loop(p)).unwrap()?;
        Ok(WhileStmt {
            cond,
            block
        })
    }

    fn var_stmt(&self, pair: Pair<Rule>) -> Result<VarStmt, ParseError> {
        let mut var_stmt_inner = pair.into_inner();
        let ident_pair = var_stmt_inner.next().unwrap();
        let expr = if let Some(expr_pair) = var_stmt_inner.next() {
            Some(self.expr(expr_pair)?)
        } else {
            None
        };
        Ok(VarStmt {
            ident: self.ident(ident_pair)?,
            expr,
        })
    }

    fn return_stmt(&self, return_stmt_pair: Pair<Rule>) -> Result<ReturnStmt, ParseError> {
        let mut return_stmt_inner = return_stmt_pair.into_inner();
        let expr = if let Some(expr_pair) = return_stmt_inner.next() {
            Some(self.expr(expr_pair)?)
        } else {
            None
        };
        Ok(ReturnStmt { expr })
    }

    fn expr(&self, expr_pair: Pair<Rule>) -> Result<Expr, ParseError> {
        PRATT_PARSER
            .map_primary(|primary_pair| match primary_pair.as_rule() {
                Rule::literal => Ok(Expr::Literal(self.literal(primary_pair)?)),
                Rule::IDENT => Ok(Expr::Ident(self.ident(primary_pair)?)),
                Rule::expr => Ok(self.expr(primary_pair)?),
                rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
            })
            .map_postfix(|lhs, postfix_pair| {
                let postfix = match postfix_pair.as_rule() {
                    Rule::INDEX => PostfixOpKind::Index(Box::new(
                        self.expr(postfix_pair.into_inner().next().unwrap())?,
                    )),
                    Rule::CALL => {
                        let mut exprs = vec![];
                        for expr_pair in postfix_pair.into_inner() {
                            exprs.push(self.expr(expr_pair)?)
                        }
                        PostfixOpKind::Call(exprs)
                    }
                    _ => unreachable!(),
                };
                Ok(Expr::PostfixOp {
                    kind: postfix,
                    lhs: Box::new(lhs?),
                })
            })
            .map_prefix(|prefix_pair, rhs| {
                let prefix = match prefix_pair.as_rule() {
                    Rule::POS => PrefixOpKind::Pos,
                    Rule::NEG => PrefixOpKind::Neg,
                    Rule::NOT => PrefixOpKind::Not,
                    _rule => unreachable!(),
                };
                Ok(Expr::PrefixOp {
                    kind: prefix,
                    rhs: Box::new(rhs?),
                })
            })
            .map_infix(|lhs, infix_pair, rhs| {
                let infix = match infix_pair.as_rule() {
                    Rule::ADD => InfixOpKind::Add,
                    Rule::SUBTRACT => InfixOpKind::Subtract,
                    Rule::MULTIPLY => InfixOpKind::Multiply,
                    Rule::DIVIDE => InfixOpKind::Divide,
                    Rule::MODULO => InfixOpKind::Modulo,
                    Rule::EQ => InfixOpKind::Eq,
                    Rule::NOT_EQ => InfixOpKind::NotEq,
                    Rule::GE => InfixOpKind::Ge,
                    Rule::GT => InfixOpKind::Gt,
                    Rule::LE => InfixOpKind::Le,
                    Rule::LT => InfixOpKind::Lt,
                    Rule::AND => InfixOpKind::And,
                    Rule::OR => InfixOpKind::Or,
                    Rule::ASSIGN => InfixOpKind::Assign,
                    _ => unreachable!(),
                };
                Ok(Expr::InfixOp {
                    kind: infix,
                    lhs: Box::new(lhs?),
                    rhs: Box::new(rhs?),
                })
            })
            .parse(expr_pair.into_inner())
    }

    fn literal(&self, literal_pair: Pair<Rule>) -> Result<Literal, ParseError> {
        let unknown_pair = literal_pair.into_inner().next().unwrap();
        match unknown_pair.as_rule() {
            Rule::BOOL_LITERAL => Ok(Literal::Bool(unknown_pair.as_str().parse().unwrap())),
            Rule::INT_LITERAL => Ok(Literal::Int(unknown_pair.as_str().parse().unwrap())), // FIXME: Handle overflow
            Rule::STRING_LITERAL => Ok(Literal::String(unknown_pair.as_str().parse().unwrap())),
            Rule::ARRAY_LITERAL => {
                let mut exprs = vec![];
                for expr_pair in unknown_pair.into_inner() {
                    exprs.push(self.expr(expr_pair)?);
                }
                Ok(Literal::Array(exprs))
            }
            Rule::NONE_LITERAL => Ok(Literal::None),
            _ => unreachable!(),
        }
    }

    fn ident(&self, ident_pair: Pair<Rule>) -> Result<Ident, ParseError> {
        Ok(Ident {
            name: ident_pair.as_str().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::*;
    use crate::parser::*;

    #[test]
    fn generate_pest_parser_error() {
        let tests = [
            // Empty statement
            (
                r#"
            ;
            "#,
                Err(ParseError::PestParser),
            ),
            // Identifier starts with number
            (
                r#"
            var 0a;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var 1a;
            "#,
                Err(ParseError::PestParser),
            ),
            // Identifier uses reserved words
            (
                r#"
            var func = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var return = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var if = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var else = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var for = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var var = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var true = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var false = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var break = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var continue = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var none = 1;
            "#,
                Err(ParseError::PestParser),
            ),
            // Mismatched paretheses
            (
                r#"
            (1));
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            ((1);
            "#,
                Err(ParseError::PestParser),
            ),
            // Integer literal starts with 0
            (
                r#"
            00;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            01;
            "#,
                Err(ParseError::PestParser),
            ),
            // Array literal with empty elements
            (
                r#"
            [,];
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            [, 1];
            "#,
                Err(ParseError::PestParser),
            ),
            // Invalid if statement
            (
                r#"
            if {}
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            if(){}
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            if (true)
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            if (false) {
            } elseif (true) {
            } else {
            }
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            while(){}
            "#,
                Err(ParseError::PestParser),
            ),        ];

        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn block_stmt_ast_is_built_correctly() {
        let tests = [
            (r#""#, Ok(vec![])), // Empty program
            (
                r#"
            {}
            "#,
                Ok(vec![Stmt::BlockStmt(vec![])]),
            ),
            (
                r#"
            0;
            {
                1;
                {
                    2;
                }
            }
            "#,
                Ok(vec![
                    Stmt::ExprStmt(Expr::literal_int(0)),
                    Stmt::BlockStmt(vec![
                        Stmt::ExprStmt(Expr::literal_int(1)),
                        Stmt::BlockStmt(vec![Stmt::ExprStmt(Expr::literal_int(2))]),
                    ]),
                ]),
            ),
            (
                r#"
            {
                return;
            }
            "#,
                Err(ParseError::BuildAst),
            ),
            (
                r#"
            {
                break;
            }
            "#,
                Err(ParseError::BuildAst),
            ),
            (
                r#"
            {
                continue;
            }
            "#,
                Err(ParseError::BuildAst),
            ),
        ];

        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn func_decl_ast_is_built_correctly() {
        let tests = [
            (
                r#"
            func foo(){}
            "#,
                Ok(vec![Stmt::FuncDecl(FuncDecl {
                    ident_func: Ident {
                        name: "foo".to_string(),
                    },
                    ident_param: vec![],
                    block: vec![],
                })]),
            ),
            (
                r#"
            func foo(param1){}
            "#,
                Ok(vec![Stmt::FuncDecl(FuncDecl {
                    ident_func: Ident {
                        name: "foo".to_string(),
                    },
                    ident_param: vec![Ident {
                        name: "param1".to_string(),
                    }],
                    block: vec![],
                })]),
            ),
            (
                r#"
            func foo(param){
                123;
                return x;
            }
            "#,
                Ok(vec![Stmt::FuncDecl(FuncDecl {
                    ident_func: Ident {
                        name: "foo".to_string(),
                    },
                    ident_param: vec![Ident {
                        name: "param".to_string(),
                    }],
                    block: vec![
                        Stmt::ExprStmt(Expr::literal_int(123)),
                        Stmt::ReturnStmt(ReturnStmt {
                            expr: Some(Expr::Ident(Ident {
                                name: "x".to_string(),
                            })),
                        }),
                    ],
                })]),
            ),
            (
                r#"
            func foo(){
                return;
                // Following statements will be ignored by parser
                123;
            }
            "#,
                Ok(vec![Stmt::FuncDecl(FuncDecl {
                    ident_func: Ident {
                        name: "foo".to_string(),
                    },
                    ident_param: vec![],
                    block: vec![Stmt::ReturnStmt(ReturnStmt { expr: None })],
                })]),
            ),
            (
                r#"
            func foo(){ break; }
            "#,
                Err(ParseError::BuildAst),
            ),
            (
                r#"
            func foo(){ continue; }
            "#,
                Err(ParseError::BuildAst),
            ),
            (
                r#"
            return; // return outside of function
            "#,
                Err(ParseError::BuildAst),
            ),
        ];

        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn if_stmt_ast_is_built_correctly() {
        let tests = vec![
            (
                r#"
            if() {}
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            if(true) {
                if (true) {}
            }
            "#,
                Ok(vec![Stmt::IfStmt(IfStmt {
                    condition: Expr::literal_bool(true),
                    block: vec![Stmt::IfStmt(IfStmt {
                        condition: Expr::literal_bool(true),
                        block: vec![],
                        else_clause: None,
                    })],
                    else_clause: None,
                })]),
            ),
            (
                r#"
            if(true) {} else {}
            "#,
                Ok(vec![Stmt::IfStmt(IfStmt {
                    condition: Expr::literal_bool(true),
                    block: vec![],
                    else_clause: Some(IfStmtElseClause::IfStmtBlock(vec![])),
                })]),
            ),
            (
                r#"
            if (true) {
            } else if(false) {
            }
            "#,
                Ok(vec![Stmt::IfStmt(IfStmt {
                    condition: Expr::literal_bool(true),
                    block: vec![],
                    else_clause: Some(IfStmtElseClause::IfStmt(Box::new(IfStmt {
                        condition: Expr::literal_bool(false),
                        block: vec![],
                        else_clause: None,
                    }))),
                })]),
            ),
            (
                r#"
            if (true) {
            } else if(false) {
            } else if(false) {
            } else {
            }
            "#,
                Ok(vec![Stmt::IfStmt(IfStmt {
                    condition: Expr::literal_bool(true),
                    block: vec![],
                    else_clause: Some(IfStmtElseClause::IfStmt(Box::new(IfStmt {
                        condition: Expr::literal_bool(false),
                        block: vec![],
                        else_clause: Some(IfStmtElseClause::IfStmt(Box::new(IfStmt {
                            condition: Expr::literal_bool(false),
                            block: vec![],
                            else_clause: Some(IfStmtElseClause::IfStmtBlock(vec![])),
                        }))),
                    }))),
                })]),
            ),
            (
                r#"
            if(true) {
                return;
            }
            "#,
                Err(ParseError::BuildAst),
            ),
            (
                r#"
            if(true) {
                break; // break outside of loop
            }
            "#,
                Err(ParseError::BuildAst),
            ),
            (
                r#"
            if(true) {
                continue; // continue outside of loop
            }
            "#,
                Err(ParseError::BuildAst),
            ),
        ];

        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn for_stmt_ast_is_built_correctly() {
        let tests = vec![
            (
                r#"
            for(; ; ){}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: None,
                    afterthought: None,
                    block: vec![],
                })]),
            ),
            (
                r#"
            for(; ; ){1;}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: None,
                    afterthought: None,
                    block: vec![Stmt::ExprStmt(Expr::literal_int(1))],
                })]),
            ),
            (
                r#"
            for(; ; 1){}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: None,
                    afterthought: Some(Expr::literal_int(1)),
                    block: vec![],
                })]),
            ),
            (
                r#"
            for(; ; 1){1;}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: None,
                    afterthought: Some(Expr::literal_int(1)),
                    block: vec![Stmt::ExprStmt(Expr::literal_int(1))],
                })]),
            ),
            (
                r#"
            for(; 1; ){}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: Some(Expr::literal_int(1)),
                    afterthought: None,
                    block: vec![],
                })]),
            ),
            (
                r#"
            for(; 1; ){1;}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: Some(Expr::literal_int(1)),
                    afterthought: None,
                    block: vec![Stmt::ExprStmt(Expr::literal_int(1))],
                })]),
            ),
            (
                r#"
            for(; 1; 1){}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: Some(Expr::literal_int(1)),
                    afterthought: Some(Expr::literal_int(1)),
                    block: vec![],
                })]),
            ),
            (
                r#"
            for(; 1; 1){1;}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: Some(Expr::literal_int(1)),
                    afterthought: Some(Expr::literal_int(1)),
                    block: vec![Stmt::ExprStmt(Expr::literal_int(1))],
                })]),
            ),
            (
                r#"
            for(1; ; ){}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Expr(Expr::literal_int(1))),
                    cond: None,
                    afterthought: None,
                    block: vec![],
                })]),
            ),
            (
                r#"
            for(1; ; ){1;}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Expr(Expr::literal_int(1))),
                    cond: None,
                    afterthought: None,
                    block: vec![Stmt::ExprStmt(Expr::literal_int(1))],
                })]),
            ),
            (
                r#"
            for(1; ; 1){}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Expr(Expr::literal_int(1))),
                    cond: None,
                    afterthought: Some(Expr::literal_int(1)),
                    block: vec![],
                })]),
            ),
            (
                r#"
            for(1; ; 1){1;}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Expr(Expr::literal_int(1))),
                    cond: None,
                    afterthought: Some(Expr::literal_int(1)),
                    block: vec![Stmt::ExprStmt(Expr::literal_int(1))],
                })]),
            ),
            (
                r#"
            for(1; 1; ){}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Expr(Expr::literal_int(1))),
                    cond: Some(Expr::literal_int(1)),
                    afterthought: None,
                    block: vec![],
                })]),
            ),
            (
                r#"
            for(1; 1; ){1;}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Expr(Expr::literal_int(1))),
                    cond: Some(Expr::literal_int(1)),
                    afterthought: None,
                    block: vec![Stmt::ExprStmt(Expr::literal_int(1))],
                })]),
            ),
            (
                r#"
            for(1; 1; 1){}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Expr(Expr::literal_int(1))),
                    cond: Some(Expr::literal_int(1)),
                    afterthought: Some(Expr::literal_int(1)),
                    block: vec![],
                })]),
            ),
            (
                r#"
            for(1; 1; 1){1;}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Expr(Expr::literal_int(1))),
                    cond: Some(Expr::literal_int(1)),
                    afterthought: Some(Expr::literal_int(1)),
                    block: vec![Stmt::ExprStmt(Expr::literal_int(1))],
                })]),
            ),
            (
                r#"
            for(var i = 1; ; ){}
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: Some(ForStmtInit::Var(VarStmt {
                        ident: Ident {
                            name: "i".to_string(),
                        },
                        expr: Some(Expr::literal_int(1)),
                    })),
                    cond: None,
                    afterthought: None,
                    block: vec![],
                })]),
            ),
            (
                r#"
            for(;;){
                break;
                continue;  // ignored by parser
            }
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: None,
                    afterthought: None,
                    block: vec![Stmt::BreakStmt],
                })]),
            ),
            (
                r#"
            for(;;){
                continue;
                break;  // Ignored by parser
            }
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: None,
                    afterthought: None,
                    block: vec![Stmt::ContinueStmt],
                })]),
            ),
            (
                r#"
            for(;;){
                {
                    continue;
                    break;  // Ignored by parser

                }
            }
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: None,
                    afterthought: None,
                    block: vec![Stmt::BlockStmt(vec![Stmt::ContinueStmt])],
                })]),
            ),
            (
                r#"
            for(;;){
                {
                    break;
                    continue;  // ignored by parser
                }
            }
            "#,
                Ok(vec![Stmt::ForStmt(ForStmt {
                    init: None,
                    cond: None,
                    afterthought: None,
                    block: vec![Stmt::BlockStmt(vec![Stmt::BreakStmt])],
                })]),
            ),
            (
                r#"
            break; // break outside of loop
            "#,
                Err(ParseError::BuildAst),
            ),
            (
                r#"
            continue; // continue outside of loop
            "#,
                Err(ParseError::BuildAst),
            ),
            (
                r#"
            for(return;;){}
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            for(;return;){}
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            for(;;return){}
            "#,
                Err(ParseError::PestParser),
            ),
        ];

        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn while_stmt_ast_is_built_correctly() {
        let tests = [
            (
                r#"
            while (true) {}
            "#,
                Ok(vec![
                    Stmt::WhileStmt(WhileStmt {
                        cond: Expr::literal_bool(true),
                        block: vec![]
                    })
                ])
            ),
            (
                r#"
            while (true) {
                42;
            }
            "#,
                Ok(vec![
                    Stmt::WhileStmt(WhileStmt {
                        cond: Expr::literal_bool(true),
                        block: vec![
                            Stmt::ExprStmt(Expr::literal_int(42))
                        ]
                    })
                ])
            ),
            (
                r#"
            while (true) {
                break;
                continue; // Ignored by parser
            }
            "#,
                Ok(vec![
                    Stmt::WhileStmt(WhileStmt {
                        cond: Expr::literal_bool(true),
                        block: vec![
                            Stmt::BreakStmt
                        ]
                    })
                ])
            ),
            (
                r#"
            while (true) {
                continue;
                break; // Ignored by parser
            }
            "#,
                Ok(vec![
                    Stmt::WhileStmt(WhileStmt {
                        cond: Expr::literal_bool(true),
                        block: vec![
                            Stmt::ContinueStmt
                        ]
                    })
                ])
            ),
        ];

        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn var_stmt_ast_is_built_correctly() {
        let tests = [
            (
                r#"
            var x;
            "#,
                Ok(vec![Stmt::VarStmt(VarStmt {
                    ident: Ident {
                        name: "x".to_string(),
                    },
                    expr: None,
                })]),
            ),
            (
                r#"
            var x = none;
            "#,
                Ok(vec![Stmt::VarStmt(VarStmt {
                    ident: Ident {
                        name: "x".to_string(),
                    },
                    expr: Some(Expr::literal_none()),
                })]),
            ),
            (
                r#"
            var x = 1 + 2;
            "#,
                Ok(vec![Stmt::VarStmt(VarStmt {
                    ident: Ident {
                        name: "x".to_string(),
                    },
                    expr: Some(Expr::infix(
                        InfixOpKind::Add,
                        Expr::literal_int(1),
                        Expr::literal_int(2),
                    )),
                })]),
            ),
            // PestParser
            (
                r#"
            var 0;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var 1;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var 1a;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var func;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var func;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var return;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var if;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var else;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var for;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var var;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var true;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var false;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var break;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var continue;
            "#,
                Err(ParseError::PestParser),
            ),
            (
                r#"
            var none;
            "#,
                Err(ParseError::PestParser),
            ),
        ];
        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn literal_expr_stmt_ast_is_built_correctly() {
        let tests = [
            // Boolean
            (
                r#"
            true;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_bool(true))]),
            ),
            (
                r#"
            false;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_bool(false))]),
            ),
            // Integer
            (
                r#"
            0;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_int(0))]),
            ),
            (
                r#"
            1;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_int(1))]),
            ),
            (
                r#"
                9223372036854775807;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_int(9223372036854775807))]),
            ),
            // String
            (
                r#"
            "foo";
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_string(
                    "foo".to_string(),
                ))]),
            ),
            (
                r#"
            "\"";
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_string(
                    "\\\"".to_string(),
                ))]),
            ),
            (
                r#"
            "\\";
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_string(
                    "\\\\".to_string(),
                ))]),
            ),
            (
                r#"
            "\n";
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_string(
                    "\\n".to_string(),
                ))]),
            ),
            // Array
            (
                r#"
            [1, 2, 3];
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_array(vec![
                    Expr::literal_int(1),
                    Expr::literal_int(2),
                    Expr::literal_int(3),
                ]))]),
            ),
            (
                r#"
            [];
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_array(vec![]))]),
            ),
            // None
            (
                r#"
            none;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::literal_none())]),
            ),
        ];

        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn ident_expr_stmt_ast_is_built_correctly() {
        let tests = [(
            r#"
            foo;
            "#,
            Ok(vec![Stmt::ExprStmt(Expr::ident("foo"))]),
        )];

        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn prefix_op_expr_stmt_ast_is_built_correctly() {
        let tests = [
            // Pos
            (
                r#"
            +1;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::prefix(
                    PrefixOpKind::Pos,
                    Expr::literal_int(1),
                ))]),
            ),
            // Neg
            (
                r#"
            -1;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::prefix(
                    PrefixOpKind::Neg,
                    Expr::literal_int(1),
                ))]),
            ),
            // Not
            (
                r#"
            !true;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::prefix(
                    PrefixOpKind::Not,
                    Expr::literal_bool(true),
                ))]),
            ),
            (
                r#"
            !false;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::prefix(
                    PrefixOpKind::Not,
                    Expr::literal_bool(false),
                ))]),
            ),
        ];
        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn infix_op_expr_stmt_ast_is_built_correctly() {
        let tests = [
            (
                r#"
            2 + 3;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::infix(
                    InfixOpKind::Add,
                    Expr::literal_int(2),
                    Expr::literal_int(3),
                ))]),
            ),
            (
                r#"
            2 - 3;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::infix(
                    InfixOpKind::Subtract,
                    Expr::literal_int(2),
                    Expr::literal_int(3),
                ))]),
            ),
            (
                r#"
            2 * 3;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::infix(
                    InfixOpKind::Multiply,
                    Expr::literal_int(2),
                    Expr::literal_int(3),
                ))]),
            ),
            (
                r#"
            2 / 3;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::infix(
                    InfixOpKind::Divide,
                    Expr::literal_int(2),
                    Expr::literal_int(3),
                ))]),
            ),
            (
                r#"
            2 % 3;
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::infix(
                    InfixOpKind::Modulo,
                    Expr::literal_int(2),
                    Expr::literal_int(3),
                ))]),
            ),
        ];

        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }

    #[test]
    fn postfix_op_expr_stmt_ast_is_built_correctly() {
        let tests = [
            // Index
            (
                r#"
            [x, y][1];
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::PostfixOp {
                    kind: PostfixOpKind::Index(Box::new(Expr::literal_int(1))),
                    lhs: Box::new(Expr::literal_array(vec![
                        Expr::ident("x"),
                        Expr::ident("y"),
                    ])),
                })]),
            ),
            // Call
            (
                r#"
            foo(1, x);
            "#,
                Ok(vec![Stmt::ExprStmt(Expr::PostfixOp {
                    kind: PostfixOpKind::Call(vec![Expr::literal_int(1), Expr::ident("x")]),
                    lhs: Box::new(Expr::ident("foo")),
                })]),
            ),
        ];
        for (src, expected) in tests {
            assert_eq!(parse(src), expected, "Failed in test case: {}", src);
        }
    }
}
