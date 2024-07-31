use std::collections::HashMap;

use crate::error::Error;
use crate::error::Error::RuntimeError;
use crate::interpreter::Value;
use crate::token::Token;

pub struct Environment {
    pub values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value, Error> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError(Option::from(format!("Undefined variable '{}'.", name.lexeme)))),
        }
    }

    pub(crate) fn assign(&mut self, name: &Token, value: Value) -> Result<Value, Error> {
        match self.values.contains_key(&name.lexeme) {
            true => {
                self.values.insert(name.lexeme.clone(), value.clone());
                Ok(value)
            }
            false => {
                Err(RuntimeError(Option::from(format!("Undefined variable '{}'.", name.lexeme))))
            }
        }
    }
}