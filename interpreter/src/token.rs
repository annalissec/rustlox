use crate::tokentype::TokenType;
use crate::object::Object;
use std::fmt;

pub struct Token {
    t_type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

// TODO: toString() ? https://github.com/Harmful-Alchemist/rilox/blob/main/src/token.rs
#[derive(Debug)]
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
            .field("type", &self.token_type)
            .field("lexeme", &self.lexeme)
            .field("literal", &self.literal)
            .finish()
    }
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
