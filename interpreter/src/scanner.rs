use crate::token::*;
use crate::tokentype::TokenType;
//TODO: do you need to use TokenType before each one?
use crate::object::*;
use crate::error::*;

#[derive(Clone)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start:0,
            current:0,
            line:1,

        }
    }

    fn keyword(&self, check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::AND),
            "else" => Some(TokenType::ELSE),
            "false" => Some(TokenType::FALSE),
            "for" => Some(TokenType::FOR),
            "fun" => Some(TokenType::FUN),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::NIL),
            "or" => Some(TokenType::OR),
            "print" => Some(TokenType::PRINT),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "var" => Some(TokenType::VAR),
            "while" => Some(TokenType::WHILE),
            "break" => Some(TokenType::BREAK),
            _ => None,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {

        let mut had_error: Option<LoxError> = None;
        
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {},
                Err(e) =>
                {had_error = Some(e);}
            }
        }

        self.tokens.push(Token{
            t_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: None,
            line: self.line,

        });
        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.len();
    }

    //TODO: fix error return 
    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let sec_char = self.is_match('=');
                self.add_token(if sec_char {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                })
            }
            '=' => {
                let sec_char = self.is_match('=');
                self.add_token(if sec_char {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                })
            }
            '<' => {
                let sec_char = self.is_match('=');
                self.add_token(if sec_char {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                })
            }
            '>' => {
                let sec_char = self.is_match('=');
                self.add_token(if sec_char {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                })
            }
            '/' => {
                let sec_char = self.is_match('/');
                if sec_char {
                    let mut next = self.peek();
                    while next != '\n' && !self.is_at_end() {
                        self.advance();
                        next = self.peek();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line = self.line + 1;
            }
            '"' => self.string(),
            c => {
                if self.is_digit(c) {
                    self.number();
                } 
                else if self.is_alpha(c) {
                    self.identifier();
                } 
                else {
                    LoxError::error(self.line, String::from("Unexpected character."));
                }
            }
            
        }
        Ok(())
    }

    fn identifier(&mut self) {
        let mut peek_var = self.peek();
        while self.is_alpha_numeric(peek_var) {
            self.advance();
            peek_var = self.peek();
        }

        let text = &self.source[self.start..self.current];
        if let Some(t_type) = self.keyword(text) {
            self.add_token(t_type);
        } else {
            self.add_token(TokenType::IDENTIFIER);
        }
    }

    pub fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        } 

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
        }

        while self.peek().is_ascii_digit() {
            self.advance();
        } 

        let s = &self.source[self.start..self.current];

        self.add_token_fr(TokenType::NUMBER, Some(Object::Number(s.parse::<f64>().unwrap())));
    }

    pub fn string(&mut self) {
        let mut peek_var = self.peek();
        while peek_var != '"' && !self.is_at_end() {
            if peek_var == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
            peek_var = self.peek();
        }

        if self.is_at_end() {
            LoxError::error(self.line, String::from("Unterminated string."));
        }

        self.advance();

        let value = String::from(&self.source[self.start+1..self.current-1]);
        self.add_token_fr(TokenType::STRING, Some(Object::String(value)));

    }

    pub fn is_match(&mut self, expected: char) -> bool{
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current = self.current + 1;
        return true;
    }

    pub fn peek(&mut self ) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    pub fn peek_next(&mut self) -> char{
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    pub fn is_digit(&mut self, c: char) -> bool {
        return c.is_ascii_digit();
    }
    
    pub fn is_alpha(&mut self, c: char) -> bool {
        return c.is_alphabetic() || c == '_';
    }

    pub fn is_alpha_numeric(&mut self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    pub fn advance(&mut self) -> char {
        let result = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        result
    }

    pub fn add_token(&mut self, t_type: TokenType) {
        self.add_token_fr(t_type, None)
    }

    pub fn add_token_fr(&mut self, t_type: TokenType, literal: Option<Object>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token{
            t_type,
            lexeme: String::from(text),
            literal,
            line: self.line
        })
    }
}