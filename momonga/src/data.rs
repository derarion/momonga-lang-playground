use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::env::Store;
use crate::error::EvalError;
use crate::{emit_print_event, PrintEvent};

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
    Bool(bool),
    Int(i64),
    String(RefCell<String>),
    Array(Array<'a>),
    None,
    Func {
        params: &'a Vec<crate::ast::Ident>,
        block: &'a crate::ast::BlockStmt,
    },
    Builtin(
        i64,                                  // Number of arguments
        fn(BuiltinArgs<'a>) -> BuiltinReturn, // Function itself
    ),
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::String(s) => write!(f, "{}", s.borrow()),
            Value::Array(Array(vals)) => {
                let mut res = String::new();
                for (i, val) in vals.borrow().iter().enumerate() {
                    if i < 1 {
                        res.push_str(&format!("{}", val.borrow()));
                    } else {
                        // TODO: Format String with single quotes, like formatting Array value ["foo"] to ['foo']
                        res.push_str(&format!(", {}", val.borrow()))
                    }
                }
                write!(f, "[{}]", res)
            }
            Value::None => write!(f, "none"),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array<'a>(pub RefCell<Vec<Rc<RefCell<Value<'a>>>>>);

pub fn new_builtins<'a>() -> Store<'a> {
    let mut builtins = HashMap::new();
    builtins.insert("len", Rc::new(RefCell::new(Value::Builtin(1, momonga_len))));
    builtins.insert(
        "push",
        Rc::new(RefCell::new(Value::Builtin(2, momonga_push))),
    );
    builtins.insert("pop", Rc::new(RefCell::new(Value::Builtin(1, momonga_pop))));
    builtins.insert(
        "print",
        Rc::new(RefCell::new(Value::Builtin(1, momonga_print))),
    );
    builtins
}

type BuiltinArgs<'a> = Vec<Rc<RefCell<Value<'a>>>>;
type BuiltinReturn<'a> = Result<Rc<RefCell<Value<'a>>>, EvalError>;

pub fn momonga_len(args: BuiltinArgs) -> BuiltinReturn {
    match *args[0].borrow() {
        Value::String(ref string) => Ok(Rc::new(RefCell::new(Value::Int(
            string.borrow().len() as i64
        )))),
        Value::Array(Array(ref vals)) => Ok(Rc::new(RefCell::new(Value::Int(
            vals.borrow().len() as i64
        )))),
        _ => Err(EvalError::Type),
    }
}

pub fn momonga_push(args: BuiltinArgs) -> BuiltinReturn {
    match *args[0].borrow() {
        Value::Array(Array(ref vals)) => {
            vals.borrow_mut().push(Rc::clone(&args[1]));
            Ok(Rc::new(RefCell::new(Value::Array(Array(vals.clone())))))
        }
        _ => Err(EvalError::Type),
    }
}

pub fn momonga_pop(args: BuiltinArgs) -> BuiltinReturn {
    match *args[0].borrow() {
        Value::Array(Array(ref vals)) => match vals.borrow_mut().pop() {
            Some(val) => Ok(Rc::new(RefCell::new(val.borrow_mut().clone()))),
            None => Err(EvalError::Index), // pop() from empty array
        },
        _ => Err(EvalError::Type),
    }
}

pub fn momonga_print(args: BuiltinArgs) -> BuiltinReturn {
    emit_print_event(PrintEvent::Stdout, &(*args[0].borrow()).to_string());
    Ok(Rc::new(RefCell::new(Value::None)))
}
