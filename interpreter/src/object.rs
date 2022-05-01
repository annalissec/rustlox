
use std::fmt;


#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f64),
    Bool(bool),
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
        }
    }
}