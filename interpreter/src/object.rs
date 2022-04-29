
use std::fmt;

#[derive(Debug)]
pub enum Object {
    String(String),
    Number(f64),
    Bool(bool),
    None,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Number(x) => write!(f, "{x}"),
            Object::String(s) => write!(f, "{x}"),
            Object::Bool(x) => {
                if *x {
                    write!(f, "true");
                } else {
                    write!(f, "false");
                }
            }
            Object::None => write!(f, "nil"),
        }
    }
}