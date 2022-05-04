use super::token::*;
use super::object::Object;


//TODO: https://github.com/sasurau4/lox-rust/blob/master/interpreter/src/expr.rs
pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Object) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
}

pub trait Acceptor<T> {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Option<Object>,
    },
}

impl<T> Acceptor<T> for Expr {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(&value.clone().unwrap()),
        }
    }
}