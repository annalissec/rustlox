use std::collections::HashMap;
use crate::object::Object;
use crate::token::Token;
use crate::error::LoxError;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new()
        }
    }

    pub fn new_enclosing(&self, enclosing: Environment) {
        self.enclosing = Some(Rc::new(enclosing));
    }

    pub fn define(&self, name: String, value: Object) {
        self.values.clone().insert(name, value);
        return;
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        if let Some(object) = self.values.get(&name.lexeme) {
            return Ok(object.clone());
        } 
        // if !assert_eq!(self.enclosing, None) {
        //     return Ok(self.enclosing.as_ref().unwrap().borrow().get(name)?);
        // }
        else {
            return Err(LoxError::runtime_error(&name, String::from(format!("Undefined variable '{}'.", name.lexeme))));
        }                                           
    }
    
    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxError>{
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return Ok(());
        }
        // if !assert_eq!(self.enclosing, None) {
        //     self.enclosing.as_ref().unwrap().borrow().assign(name, value)?;
        //     return Ok(());
        // } 
        else {
            Err(LoxError::runtime_error(name, String::from(format!("Undefined variable '{}'.", &name.lexeme))))
        }
        
    }
}