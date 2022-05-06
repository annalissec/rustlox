use std::fmt;
use std::rc::Rc;
use std::time::SystemTime;
use core::fmt::{Debug, Display};

use crate::interpreter::*;
use crate::object::*;
use crate::error::*;

#[derive(Clone)]
pub struct Callable {
    pub function: Rc<dyn LoxCallable>,
    pub arity: usize
}

impl Debug for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<callable>")
    }
}


impl Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Callable>")
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.function, &other.function)
    }
}


pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) ->  Result<Object, LoxError>;
    fn arity(&self) -> usize;
}

impl LoxCallable for Callable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) ->  Result<Object, LoxError> {
        self.function.call(interpreter, arguments)
    }
    fn arity(&self) -> usize {
        self.arity
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