use crate::expr::*;
use crate::object::Object;
use crate::error::LoxError;
use crate::tokentype::TokenType::*;


pub struct Interpreter {

}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError>{
        return expr.accept(self)
    }
    fn is_truthy(&self, object: &Object) -> bool {
        if *object == Object::Nil {
            return false;
        } 
        match object {
            Object::Bool(b) => {
                return *b;
            }
            _ => {}
        }
        return true;
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError>{
        Ok(expr.value.clone().unwrap())
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        Ok(self.evaluate(&expr.expression)?)
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let mut right = self.evaluate(&expr.right)?;

        match expr.operator.t_type {
            MINUS => match right {
                Object::Number(n) => return Ok(Object::Number(-n)),
                _ => return Ok(Object::Nil),
            }
            BANG => {
                if self.is_truthy(&right) {
                    Ok(Object::Bool(false))
                } else { Ok(Object::Bool(true))}
            }
        }
        Err(LoxError::null()) //unreachable
    }
}