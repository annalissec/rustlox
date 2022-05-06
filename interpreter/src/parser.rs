#![allow(non_snake_case)]

use crate::tokentype::TokenType::*;
use crate::tokentype::TokenType;
use crate::expr::Expr;
use crate::object::Object;
use crate::error::LoxError;
use crate::expr::*;
use crate::stmt::Stmt;
use crate::stmt::*;
use crate::token::Token;


use std::rc::Rc;


#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
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

    pub fn parse(&mut self) -> Result<Vec<Rc<Stmt>>, LoxError>{
        let mut statements : Vec<Rc<Stmt>>= Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        if self.had_error {
            Err(LoxError::null())
        } else {
            return Ok(statements);
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError>{
        Ok(self.assignment()?)
    }

    fn declaration(&mut self) -> Result<Rc<Stmt>, LoxError>{
        let result = 
        if self.is_match(&[VAR]) {
            return Ok(self. var_declaration()?)
        }
        else {
            Ok(self.statement()?)
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    fn statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        
        if self.is_match(&[LEFT_BRACE]) {
            return Ok(Rc::new(Stmt::Block(Rc::new(BlockStmt{statements: Rc::new(self.block()?)}))));
        }
        if self.is_match(&[PRINT]) {
            return Ok(Rc::new(self.print_statement()?));
        }
        if self.is_match(&[IF]) {
            return Ok(Rc::new(self.if_statement()?));
        }
        if self.is_match(&[WHILE]) {
            return Ok(Rc::new(self.while_statement()?));
        }
        if self.is_match(&[FOR]) {
            return Ok(self.for_statement()?);
        }

        // if self.is_match(&[BREAK]) {
        //     return Ok(Rc::new(self.break_statement()));
        // }
        // if self.is_match(&[CONTINUE]) {
        //     return Ok(Rc::new(self.continue_statement()?));
        // }
         
        return Ok(self.expression_statement()?);
    }

    // fn break_statement(&mut self) -> Result<Stmt, LoxError> {
        
    // }

    fn for_statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        self.consume(LEFT_PAREN, String::from("Expect '(' after 'for'."))?;

        let initializer = 
            if self.is_match(&[SEMICOLON]) {
                None
            } else if self.is_match(&[VAR]) {
                Some(self.var_declaration()?)
            } else {
                Some(self.expression_statement()?)
            };
        
        let mut condition = 
            if !self.check(SEMICOLON) {
                Some(self.expression()?)
            } else {
                None
            };

        self.consume(SEMICOLON, String::from("Expect ';' after loop condition."))?;

        let increment = 
            if !self.check(RIGHT_PAREN) {
                Some(self.expression()?)
            } else {
                None
            };
        
        self.consume(RIGHT_PAREN, String::from("Expect ')' after for clauses."))?;
        let mut body = self.statement()?;

        
        match increment {
            None => {},
            Some(inc) => {
                body = Rc::new(Stmt::Block(Rc::new(BlockStmt{
                    statements: Rc::new(vec!(body, Rc::new(Stmt::Expression(Rc::new(ExpressionStmt{
                        expression: Rc::new(inc)
                    })))))
                })))
            }
        }

        match condition {
            None => {
                condition = Some(Expr::Literal(Rc::new(LiteralExpr{
                    value: Some(Object::Bool(false))
                })))
            }
            _ => {}
        }

        body = Rc::new(Stmt::While(Rc::new(WhileStmt{
            condition: Rc::new(condition.unwrap()),
            body
        })));

        match initializer {
            None => {},
            Some(initializer) => {
                body = Rc::new(Stmt::Block(Rc::new(BlockStmt{
                    statements: Rc::new(vec!(initializer, body))
                })))
            }
        }

        Ok(body.clone())
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError>{
        self.consume(LEFT_PAREN, String::from("Expect '(' after 'if'."))?;
        let condition = self.expression();

        self.consume(RIGHT_PAREN, String::from("Expect ')' after if condition."))?;

        let then_branch = self.statement();
        let else_branch = 
            if self.is_match(&[ELSE]) {
                Some(self.statement()?)
            }
            else {
                None
            };

        Ok(Stmt::If(Rc::new(IfStmt{
            condition: Rc::new(condition?), 
            then_branch: then_branch?, 
            else_branch: else_branch})))
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression();

        self.consume(SEMICOLON, String::from("Expect ';' after value."))?;

        return Ok(Stmt::Print(Rc::new(PrintStmt {expression: Rc::new(value?)})));
    }
    
    fn while_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(LEFT_PAREN, String::from("Expect '(' after 'while'."))?;
        let condition = self.expression()?;
        self.consume(RIGHT_PAREN, String::from("Expect ')' after condition."))?;
        let body = self.statement()?;

        return Ok(Stmt::While(Rc::new(WhileStmt{
            condition: Rc::new(condition),
            body
        })));

    }

    fn var_declaration(&mut self) -> Result<Rc<Stmt>, LoxError>{
        let name = self.consume(IDENTIFIER, String::from("Expect variable name."));
        
        let initializer = if self.is_match(&[EQUAL]) { Some(self.expression()) } else {None};

        self.consume(SEMICOLON, String::from("Expect ';' after variable declaration."))?;

        Ok(Rc::new(Stmt::Var(Rc::new(VarStmt{name: name?, initializer: Some(Rc::new(
            match initializer {
                None => {Expr::Literal(Rc::new(LiteralExpr{
                    value: Some(Object::Nil)
                }))}
                Some(initializer) => {
                    initializer?
                }
            }
        ))}))))
        
    }

    fn expression_statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        let expr = self.expression();

        self.consume(SEMICOLON, String::from("Expect ';' after expression."))?;
        
        return Ok(Rc::new(Stmt::Expression(Rc::new(ExpressionStmt {expression: Rc::new(expr?)}))));
    }

    fn block(&mut self) -> Result<Vec<Rc<Stmt>>, LoxError> {
        let mut statements = Vec::new();

        while !self.check(RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(RIGHT_BRACE, String::from("Expect '}' after block."))?;
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.or()?;
        
        if self.is_match(&[EQUAL]) {
            let equals = self.previous();
            let value = self.assignment();

            if let Expr::Variable(expr) = expr {
                return Ok(Expr::Assign(Rc::new(AssignExpr{name: expr.name.clone(),  value: Rc::new(value?)}))) 
            } else {
                //TODO: might be wrong
                return Err(LoxError::error(equals.line, String::from("Invalid assignment target.")))
            }
        }
        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.and()?;

        while self.is_match(&[OR]) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Expr::Logical(Rc::new(LogicalExpr{
                left: Rc::new(expr),
                operator,
                right: Rc::new(right)
            })
        )}
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;

        while self.is_match(&[AND]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Rc::new(LogicalExpr{
                left: Rc::new(expr),
                operator,
                right: Rc::new(right)
            }) 
        )}
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.is_match(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right?)}));
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.is_match(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right?)}));
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.is_match(&[MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right?)}));
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.is_match(&[SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right?)}));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Ok(Expr::Unary(Rc::new(UnaryExpr {operator: operator, right: Rc::new(right?)})));
        }
            Ok(self.primary()?)
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
        if self.is_match(&[IDENTIFIER]) {
           return Ok(Expr::Variable(Rc::new(VariableExpr{name: self.previous()})));
        }

        if self.is_match(&[LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(RIGHT_PAREN, String::from("Expect ')' after expression."))?;
            return Ok(Expr::Grouping(Rc::new(GroupingExpr {expression: Rc::new(expr?)})));
        }

        let peek_var = self.peek();
        Err(LoxError::parse_error(&peek_var, String::from("Expect expression.")))
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for t_type in types {
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

    fn check(&self, t_type: TokenType) -> bool {
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

    fn is_at_end(&self) -> bool {
        return self.peek().t_type == EOF;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&self) -> Token {
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