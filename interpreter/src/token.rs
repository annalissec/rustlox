use crate::tokentype::*;
use crate::object::Object;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(
        t_type: TokenType, 
        lexeme: String,
        literal: Option<Object>,
        line: usize,
    ) -> Self {
            Token {
                // TODO: not sure if this is correct struct initialization
                t_type, 
                lexeme,
                literal,
                line,
            }
        }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.t_type, self.lexeme, if let Some(literal) = &self.literal {
            literal
        }else {
            &Object::Nil
        })
    }
}
