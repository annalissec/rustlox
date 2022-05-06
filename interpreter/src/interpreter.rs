#![allow(non_snake_case)]

use crate::expr::*;
use crate::object::Object;
use crate::error::LoxError;
use crate::tokentype::TokenType::*;
use crate::token::Token;
use crate::stmt::*;
use crate::environment::Environment;
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Iterator;

#[derive(Clone, Debug)]
pub struct Interpreter {
    environment: RefCell<Rc<RefCell<Environment>>>
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: RefCell::new(Rc::new(RefCell::new(Environment::new()))),   
        }
    }
    pub fn interpret(&self, statements: Vec<Rc<Stmt>>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?;
            // match self.execute(&statement)? {
            //     LoxError => { 
            //         return Err(LoxError::null()); //TODO: fix error handling
            //     }
            //     _=> {return Ok(());}
            // }
        }
        Ok(())
    }

    fn evaluate(&self, expr: Rc<Expr>) -> Result<Object, LoxError>{
        return expr.clone().accept(self)
    }

    fn execute(&self, stmt: Rc<Stmt>) -> Result<(), LoxError>{
        stmt.accept(self)?;
        Ok(())
    }

    fn execute_block(&self, statements: &Rc<Vec<Rc<Stmt>>>, 
        environment: Environment // environment: RefCell<Rc<RefCell<Environment>>>
    ) -> Result<(), LoxError> {

        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));

        let result = statements.iter().try_for_each(|statement| self.execute(statement.clone()));

        self.environment.replace(previous);

        result
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
    fn is_equal(&self, a: Object, b: Object) -> bool {
        if a == Object::Nil && b == Object::Nil {return true;}
        if a == Object::Nil {return false;}

        match (a, b) {
            (Object::Bool(a), Object::Bool(b)) => return a == b,
            (Object::String(a), Object::String(b)) => return a == b,
            (Object::Number(a), Object::Number(b)) => return a == b,
            _ => return false,
        }
    }
    fn check_number_operand(&self, operator: Token, operand: &Object) -> Result<(), LoxError> {
        match operand {
            Object::Number(_) => {Ok(())}
            _ => {Err(LoxError::runtime_error(&operator, String::from("Operand must be a number.")))} 
        }
    }
    fn check_number_operands(&self, operator: Token, left: &Object, right: &Object) -> Result<(), LoxError> {
        match (left, right) {
            (Object::Number(_), Object::Number(_)) => {
                Ok(())
            }
            _=> Err(LoxError::runtime_error(&operator, String::from("Operands must be numbers.")))
        }
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError>{
        Ok(expr.value.clone().unwrap())
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        Ok(self.evaluate(expr.expression.clone())?)
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(expr.right.clone())?;

        match expr.operator.t_type {
            MINUS => {
                self.check_number_operand(expr.operator.clone(), &right)?; 
                match right {  
                    Object::Number(n) => return Ok(Object::Number(-n)),
                    _ => return Ok(Object::Nil),
                }
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
        let left = self.evaluate(expr.left.clone())?;
        let right = self.evaluate(expr.right.clone())?;

        match expr.operator.t_type {
            MINUS => {
                self.check_number_operands(expr.operator.clone(), &left, &right)?;
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Number(left - right))
                    }
                }
                return Err(LoxError::null())
            }
            SLASH => {
                self.check_number_operands(expr.operator.clone(), &left, &right)?;
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Number(left / right))
                    }
                }
                return Err(LoxError::null())
            }
            STAR => {
                self.check_number_operands(expr.operator.clone(), &left, &right)?;
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Number(left * right))
                    }
                }
                return Err(LoxError::null())
            }
            PLUS => {
                match (left.clone(), right.clone()) {
                    (Object::Number(x), Object::Number(y)) => {
                        Ok(Object::Number(x+y))
                    },
                    (Object::String(x), Object::String(y)) => {
                        Ok(Object::String(x+&y))
                    },
                    _ => {
                        return Err(LoxError::runtime_error(&expr.operator, String::from("Operands must be two numbers or two strings.")))
                    }
                }
            }
            GREATER => {
                self.check_number_operands(expr.operator.clone(), &left, &right)?;
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Bool(left > right))
                    }
                }
                return Err(LoxError::null())
            },
            GREATER_EQUAL => {
                self.check_number_operands(expr.operator.clone(), &left, &right)?;
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Bool(left >= right))
                    }
                }
                return Err(LoxError::null())
            },
            LESS => {
                self.check_number_operands(expr.operator.clone(), &left, &right)?;
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Bool(left < right))
                    }
                }
                return Err(LoxError::null())
            },
            LESS_EQUAL => {
                self.check_number_operands(expr.operator.clone(), &left, &right)?;
                if let Object::Number(left) = left {
                    if let Object::Number(right)= right {
                        return Ok(Object::Bool(left <= right))
                    }
                }
                return Err(LoxError::null())
            },
            BANG_EQUAL => {
                return Ok(Object::Bool(!self.is_equal(left, right)))
            },
            EQUAL_EQUAL => {
                return Ok(Object::Bool(self.is_equal(left, right)))
            },
            _ => {
                Err(LoxError::Null)
            }

            
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, LoxError> {
        Ok(self.environment.borrow().borrow_mut().get(&expr.name)?)
    }

    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Object, LoxError> {
        let value = self.evaluate(expr.value.clone());

        self.environment.borrow().borrow_mut().assign(&expr.name.clone(), value.clone()?)?;

        return Ok(value?);
    }
}   

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(stmt.expression.clone())?;
        return Ok(());
    }
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxError>{
        let value = self.evaluate(stmt.expression.clone())?;
        println!("{:?}", value);
        return Ok(());
    }
    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxError>{
        let value = if let Some(initializer) = stmt.initializer.clone() {
            self.evaluate(initializer)?
        } else {
            Object::Nil
        };
    
    
        self.environment.borrow().borrow_mut().define(stmt.name.to_string(), value);
        Ok(())
    }

    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxError>{ //enclosing: Rc<RefCell<Environment>>
        self.execute_block(&stmt.statements, Environment::new_enclosing(self.environment.borrow().clone()))? ;
        return Ok(());
    }
}