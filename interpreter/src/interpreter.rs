use crate::expr::*;
use crate::object::Object;


pub struct Interpreter {

}

// impl ExprVisitor<Object> for Interpreter {
//     fn visit_literal_expr(&mut self, expr: LiteralExpr) -> Object{
//         return expr.value;
//     }
// }