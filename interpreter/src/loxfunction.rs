use crate::object::*;
use crate::token::*;
use crate::stmt::*;
use crate::loxcallable::*;
use crate::interpreter::*;
use crate::environment::*;
use crate::error::*;


use std::rc::Rc;
use std::fmt;
use std::time::SystemTime;



pub struct LoxFunction {
    name: Token,
    params: Rc<Vec<Token>>,
    body: Rc<Vec<Rc<Stmt>>>
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name.t_type == other.name.t_type
            && Rc::ptr_eq(&self.params, &other.params)
            && Rc::ptr_eq(&self.body, &other.body)
            // && Rc::ptr_eq(&self.closure, &other.closure)
    }
}

impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{self}")
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}

impl LoxFunction{
    pub fn new(declaration: &FunctionStmt) -> Self {
        Self {
            name: declaration.name,
            params: Rc::clone(&declaration.params),
            body: Rc::clone(&declaration.body)
        }
    }
    fn to_string(&self) -> String {
        String::from(self.name.lexeme)
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxError> {
        let mut e = Environment::new_enclosing(Rc::clone(&interpreter.globals));

        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            e.define(&param.lexeme, arg.clone())
        }

        interpreter.execute_block(&self.body, e)?;
        Ok(Object::Nil)
    }
    fn arity(&self) -> usize{
        self.params.len()
    }
}

pub struct NativeClock;

impl LoxCallable for NativeClock {
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<Object>) ->  Result<Object, LoxError>{
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Ok(Object::Number(n.as_millis() as f64)),
            Err(e) => Err(LoxError::null()) //TODO: make error for clock
        }
    }

    fn arity(&self) -> usize {
        0
    }
}