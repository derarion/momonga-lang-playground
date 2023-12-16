use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    data::{new_builtins, Value},
    error::EvalError,
};

pub type Store<'a> = HashMap<&'a str, Rc<RefCell<Value<'a>>>>;

#[derive(Debug)]
pub struct Env<'a> {
    store: Store<'a>,
    outer: Option<Rc<RefCell<Env<'a>>>>,
}

impl<'a> Env<'a> {
    pub fn new(outer: Option<Rc<RefCell<Env<'a>>>>) -> Self {
        Self {
            store: HashMap::new(),
            outer,
        }
    }
    pub fn new_with_builtins() -> Self {
        Self {
            store: new_builtins(),
            outer: None,
        }
    }

    pub fn set(&mut self, name: &'a str, value: Rc<RefCell<Value<'a>>>) {
        self.store.insert(name, value);
    }

    pub fn set_var(&mut self, name: &'a str, value: Rc<RefCell<Value<'a>>>) {
        self.store.insert(name, value);
    }

    pub fn set_assign(
        &mut self,
        name: &'a str,
        value: Rc<RefCell<Value<'a>>>,
    ) -> Result<(), EvalError> {
        match self.store.get(name) {
            Some(_) => {
                self.store.insert(name, value);
                Ok(())
            }
            None => {
                match &self.outer {
                    Some(outer) => {
                        outer.borrow_mut().set_assign(name, value)?;
                        Ok(())
                    }
                    None => Err(EvalError::Name), // `name` is not defined
                }
            }
        }
    }

    pub fn get(&self, name: &str) -> Result<Rc<RefCell<Value<'a>>>, EvalError> {
        match self.store.get(name) {
            Some(val) => Ok(Rc::clone(val)),
            None => match &self.outer {
                Some(env) => env.borrow().get(name),
                None => Err(EvalError::Name),
            },
        }
    }
}
