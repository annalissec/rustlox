use super::expr::{Expr, Acceptor, Visitor};
use super::token::Token;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() {
    }
    
    pub fn print(&mut self, expr: Expr) -> String {
        return expr.accept(self);
    }
    fn parenthesize(&mut self, name: String, exprs: Vec<&Expr>) -> String {
        let mut builder = String::from("(");
        builder.push_str(&name);

        for expr in exprs {
            builder.push_str(" ");
            builder.push_str(&expr.accept(self).to_owned());
        }
        builder.push_str(")");

        return String::from(builder);
    }

}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        return self.parenthesize(operator.lexeme.to_owned(), vec![left, right]);
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
        return self.parenthesize("group".to_owned(), vec![expression]);
    }

    fn visit_literal_expr(&mut self, value: &Object) -> String {
        match value {
            Object::Nil => return String::from("nil"),
            //TODO: is this to_string() for an object?
            _ => value.to_string()
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
        return self.parenthesize(operator.lexeme.to_owned(), vec![right]);
    }
}