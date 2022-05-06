
use std::fmt;
use std::rc::Rc;

use crate::loxfunction::*;
use crate::nativefunction::*;


#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(String),
    Number(f64),
    Bool(bool),
    Func(Rc<LoxFunction>),
    Native(Rc<LoxNative>),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Number(x) => write!(f, "{x}"),
            Object::String(x) => write!(f, "{x}"),
            Object::Bool(x) => {
                if *x {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Object::Nil => write!(f, "nil"),
            Object::Func(function) => write!(f, "{function}"),
            Object::Native(function) => write!(f, "{function}"),
        }
    }
}