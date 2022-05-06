use crate::token::Token;
use crate::tokentype::TokenType::*;
// use crate::object::Object;
// use crate::expr::Expr;

#[derive(Clone, Debug)]
pub enum LoxError {
    Error { line: usize, message: String },
    ParseError {token: Token, message: String},
    RuntimeError {operator: Token, message: String},
    BreakError,
    ContinueError,
    Null
}

impl LoxError {
    pub fn break_error() -> LoxError{
        LoxError::BreakError
    }
    pub fn continue_error() -> LoxError{
        LoxError::ContinueError
    }
    pub fn error(line: usize, message: String) -> LoxError{
        let mut err = LoxError::Error {
            line, message
        };
        err.report(String::from(""));
        //Lox.had_error = true;
        err
    }

    pub fn parse_error(token: &Token, message: String) -> LoxError {
        let mut err = LoxError::ParseError {
            token: token.clone(),
            message: message
        };

        err.report(String::from(""));

        err
    }

    pub fn runtime_error(operator: &Token, message: String) -> LoxError {
        let mut err = LoxError::RuntimeError {
            operator: operator.clone(),message: message
        };

        err.report(String::from(""));

        err
    }
    
    pub fn null() -> LoxError {
        // error pretty much already handled ?
        LoxError::Null
    }
    

    pub fn report(&mut self, err: String) {
        match self {
            LoxError::Error {line, message} => {
                //self.print_error(*line, err, *message);
                eprintln!("[line {0}] Error{1}: {2}", line, err, message);
            }
            LoxError::ParseError {token, message} => {
                if token.t_type == EOF {
                    //self.print_error(token.line, String::from(" at end"), message.to_string())
                    eprintln!("[line {0}] Error{1}: {2}", token.line, " at end", message);
                } else {
                    //self.print_error(token.line, String::from(" at '") + &token.lexeme + &String::from("'"), message.to_string())
                    eprintln!("[line {0}] Error{1}: {2}", token.line, String::from(" at '") + &token.lexeme + &String::from("'"), message);
                }
            }
            LoxError::RuntimeError {operator, message} => {
                eprintln!("{} [line {}]", message, operator.line)
            }
            LoxError::ContinueError => {
            }
            LoxError::BreakError => {
            }
            LoxError::Null => {
                panic!("IDK bro")
            }
        }

    }

    // fn print_error(&mut self, line: usize, whr: String, message: String) {
    //     eprintln!("[line {0}] Error{1}: {2}", line, whr, message);
    // }
}

    