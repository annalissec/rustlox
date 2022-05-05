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
    fn is_equal (&self, a: Object, b: Object) -> bool {
        if a == Object::Nil && b == Object::Nil {return true;}
        if a == Object::Nil {return false;}
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
        let right = self.evaluate(&expr.right)?;

        match expr.operator.t_type {
            MINUS => match right {
                Object::Number(n) => return Ok(Object::Number(-n)),
                _ => return Ok(Object::Nil),
            }
            BANG => {
                return Ok(Object::Bool(!self.is_truthy(&right)))
            }
            _ => {
                Err(LoxError::null()) //unreachable after implementing all matches hopefully
            }
        }
    }
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError>{
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.t_type {
            MINUS => {
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Number(left - right))
                    }
                }
                return Err(LoxError::null()) //TODO: change error message?
            }
            SLASH => {
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Number(left / right))
                    }
                }
                return Err(LoxError::null()) //TODO: change error message?
            }
            STAR => {
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Number(left * right))
                    }
                }
                return Err(LoxError::null()) //TODO: change error message?
            }
            PLUS => {
                match (left.clone(), right.clone()) {
                    (Object::Number(x), Object::Number(y)) => {
                        Ok(Object::Number(x+y))
                        // if let Object::Number(left) = left {
                        //     if let Object::Number(right)= right {
                        //         return Ok(Object::Number(left + right))
                        //     }
                        // }
                        // return Err(LoxError::null()) //TODO: change error message?
                    },
                    (Object::String(x), Object::String(y)) => {
                        Ok(Object::String(x+&y))
                        // if let Object::String(left) = left {
                        //     if let Object::String(right)= right {
                        //         return Ok(Object::String(left + right))
                        //     }
                        // }
                        // return Err(LoxError::null()) //TODO: change error message?
                    },
                    _ => {
                        return Err(LoxError::interp_error(&left, &right, String::from("Cannot add varying types: ")))
                    }
                }
            }
            GREATER => {
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Bool(left > right))
                    }
                }
                return Err(LoxError::null()) //TODO: change error message?
            },
            GREATER_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Bool(left >= right))
                    }
                }
                return Err(LoxError::null()) //TODO: change error message?
            },
            LESS => {
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Bool(left < right))
                    }
                }
                return Err(LoxError::null()) //TODO: change error message?
            },
            LESS_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Bool(left <= right))
                    }
                }
                return Err(LoxError::null()) //TODO: change error message?
            },
            BANG_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Bool(!self.is_equal(left, right)))
                    }
                }
                return Err(LoxError::null()) //TODO: change error message?
            },
            EQUAL_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Bool(self.is_equal(left.clone(), right.clone())))
                    }
                }
                return Err(LoxError::null()) //TODO: change error message?
            },
            _ => {
                Err(LoxError::Null)
            }

            
        }
    }
}