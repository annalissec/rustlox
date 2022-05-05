use super::expr::*;
use super::object::Object;
use super::error::*;

#[derive(Debug, Clone)]
pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() {
    }
    
    pub fn print(&self, expr: Expr) -> Result<String, LoxError> {
        return expr.accept(self);
    }
    fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> Result<String, LoxError> {
        let mut builder = String::from("(");
        builder.push_str(&name);

        for expr in exprs {
            builder.push_str(" ");
            builder.push_str(&expr.accept(self)?);
        }
        builder.push_str(")");

        Ok(String::from(builder))
    }

}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError> {
        return self.parenthesize(expr.operator.lexeme.to_owned(), vec![&expr.left, &expr.right]);
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxError> {
        return self.parenthesize("group".to_owned(), vec![&expr.expression]);
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxError> {
        match expr.value {
            Some(Object::Nil) => Ok(String::from("nil")),
            _ => Ok(format!("{:?}", expr.value))
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxError> {
        return self.parenthesize(expr.operator.lexeme.to_owned(), vec![&expr.right]);
    }
}