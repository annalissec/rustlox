use crate::token::*;
use crate::tokentype::TokenType::*;
use crate::expr::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn expression(&mut self) {
        return self.equality();
    }

    pub fn equality(&mut self) -> Expr {
        let expr = self.comparison();

        while self.match(BANG_EQUAL, EQUAL_EQUAL) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary{left: Box::new(expr), operator: operator, right: Box::new(right)};
        }
        expr
    }
//TODO: start here
    pub fn match()
}