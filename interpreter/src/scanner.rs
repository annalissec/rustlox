use crate::token::*;
use crate::tokentype::TokenType;
use crate::object::*;
use crate::lox::Lox;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start:0,
            current:0,
            line:1,

        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token{
            t_type: TokenType.EOF,
            lexeme: "",
            literal: None,
            line: self.line,

        })
        Ok(&self.tokens)
    }

    pub fn scan_token(&mut self) {
        let mut c = self.advance();

        match c {
            '(' => self.add_token(TokenType.LEFT_PAREN),
            ')' => self.add_token(TokenType.RIGHT_PAREN),
            '{' => self.add_token(TokenType.LEFT_BRACE),
            '}' => self.add_token(TokenType.RIGHT_BRACE),
            ',' => self.add_token(TokenType.COMMA),
            '.' => self.add_token(TokenType.DOT),
            '-' => self.add_token(TokenType.MINUS),
            '+' => self.add_token(TokenType.PLUS),
            ';' => self.add_token(TokenType.SEMICOLON),
            '*' => self.add_token(TokenType.STAR),
            '!' => {
                let sec_char = self.is_match('=');
                self.add_token(if sec_char {
                    TokenType.BANG_EQUAL
                } else {
                    TokenType.BANG
                })
            }
            '=' => {
                let sec_char = self.is_match('=');
                self.add_token(if sec_char {
                    TokenType.EQUAL_EQUAL
                } else {
                    TokenType.EQUAL
                })
            }
            '<' => {
                let sec_char = self.is_match('=');
                self.add_token(if sec_char {
                    TokenType.LESS_EQUAL
                } else {
                    TokenType.LESS
                })
            }
            '>' => {
                let sec_char = self.is_match('=');
                self.add_token(if sec_char {
                    TokenType.GREATER_EQUAL
                } else {
                    TokenType.GREATER
                })
            }

            _ => 
                Lox.error(self.line, "Unexpected character.");
            
        }
    }

    pub fn is_match(&mut self, expected: char) {
        if (self.is_at_end()) {
            return false;
        }
        if (self.source.chars().nth(self.current).unwrap() != expected) {
            return false;
        }
        self.current = self.current + 1;
        return true;
    }

    pub fn is_at_end(&mut self) -> Result<bool> {
        Ok(self.current >= self.source.len())
    }

    pub fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        Ok(result)
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