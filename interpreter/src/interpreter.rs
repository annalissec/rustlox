use crate::expr::*;
use crate::object::Object;


pub struct Interpreter {

}

impl Visitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, expr: Expr::Literal) -> Object{
        return expr.value;
    }
}