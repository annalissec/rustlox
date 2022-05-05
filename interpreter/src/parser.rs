use crate::token::*;
use crate::tokentype::TokenType::*;
use crate::tokentype::TokenType;
use crate::expr::Expr;
use crate::object::Object;
use crate::error::LoxError;
use crate::expr::*;

use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            had_error: false
        }
    }

    pub fn parse(&mut self) -> Option<Expr>{

        // match self.expression() {
        //     Ok(expr) => Some(expr),
        //     Err(_) => None
        // }
        if self.had_error {
            return None;
        } else {
            return Some(self.expression());
        }
    }

    pub fn expression(&mut self) -> Expr{
        return self.equality();
    }

    pub fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.is_match(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right)}));
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.is_match(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right)}));
        }
        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.is_match(&[MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right)}));
        }

        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.is_match(&[SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right)}));
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.is_match(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(Rc::new(UnaryExpr {operator: operator, right: Rc::new(right)})); 
        }
        return self.primary().unwrap();
    }

    fn primary(&mut self) -> Result<Expr, LoxError>{
        if self.is_match(&[FALSE.clone()]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr{value: Some(Object::Bool(false))})));
        }
        if self.is_match(&[TRUE]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr{value: Some(Object::Bool(true))})));
        }
        if self.is_match(&[NIL]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr{value: Some(Object::Nil)})));
        }

        if self.is_match(&[NUMBER, STRING]) {
            //TODO: idk if clone changed anything
           return Ok(Expr::Literal(Rc::new(LiteralExpr{value: self.previous().literal.clone()})));
        }

        if self.is_match(&[LEFT_PAREN]) {
            let mut expr = self.expression();
            self.consume(RIGHT_PAREN, String::from("Expect ')' after expression."));
            return Ok(Expr::Grouping(Rc::new(GroupingExpr {expression: Rc::new(expr)})));
        }

        let peek_var = self.peek();
        Err(self.error(peek_var, String::from("Expect expression.")))
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for t_type in types {
            //TODO: v bad solution, brain dead
            if self.check(t_type.clone()) {
                self.advance();
                return true;
            }

        }
        return false;
    }

    fn consume(&mut self, t_type: TokenType, message: String) -> Result<Token, LoxError> {
        if self.check(t_type) {
            Ok(self.advance())
        } else {
            let peek_var = self.peek();
            Err(self.error(peek_var, message))
        }
    }

    fn check(&mut self, t_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            return self.peek().t_type == t_type;
        }
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek().t_type == EOF;
    }

    fn peek(&mut self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current-1].clone();
    }

    fn error(&mut self, token: Token, message: String) -> LoxError{
        self.had_error = true;
        return LoxError::parse_error(&token, message);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().t_type == SEMICOLON {
                return;
            }

            match self.peek().t_type {
                CLASS => {return;}
                FUN => {return;}
                FOR => {return;}
                VAR => {return;}
                IF => {return;}
                WHILE => {return;}
                PRINT => {return;}
                RETURN => {return;}
                _ => {}
            }

            self.advance();
        }
    }
}