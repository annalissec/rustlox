use crate::token::*;
use crate::tokentype::TokenType::*;
use crate::tokentype::TokenType;
use crate::expr::Expr;
use crate::object::Object;
use crate::error::LoxError;
use crate::expr::*;
use crate::stmt::Stmt;
use crate::stmt::*;

use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
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

        return Ok(statements);
    }

    fn expression(&mut self) -> Result<Expr, LoxError>{
        Ok(self.assignment()?)
    }

    fn declaration(&mut self) -> Result<Rc<Stmt>, LoxError>{
        match self.is_match(&[VAR]) {
            LoxError => {
                self.synchronize();
            }
            _=> {
                return Ok(Rc::new(self.var_declaration()?));
            }
        }
        return Ok(self.statement()?);
    }

    fn statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        if self.is_match(&[PRINT]) {
            return Ok(Rc::new(self.print_statement()?));
        }
        if self.is_match(&[LEFT_BRACE]) {
            return Ok(Rc::new(Stmt::Block(Rc::new(BlockStmt{statements: Rc::new(self.block()?)}))));
        }
         
        return Ok(Rc::new(self.expression_statement()?));
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression();

        self.consume(SEMICOLON, String::from("Expect ';' after value."))?;

        return Ok(Stmt::Print(Rc::new(PrintStmt {expression: Rc::new(value?)})));
    }

    fn var_declaration(&mut self) -> Result<Rc<Stmt>, LoxError>{
        let name = self.consume(IDENTIFIER, String::from("Expect variable name."));
        
        let initializer = if self.is_match(&[EQUAL]) { Some(self.expression()) } else {None};

        self.consume(SEMICOLON, String::from("Expect ';' after variable declaration."));

        Ok(Rc::new(Stmt::Var(Rc::new(VarStmt{name: name?, initializer: Some(Rc::new(initializer.unwrap()?))}))))
        
    }

    fn expression_statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        let expr = self.expression();

        self.consume(SEMICOLON, String::from("Expect ';' after expression."))?;
        
        return Ok(Rc::new(Stmt::Expression(Rc::new(ExpressionStmt {expression: Rc::new(expr?)}))));
    }

    fn block(&self) -> Result<Vec<Rc<Stmt>>, LoxError> {
        let mut statements = Vec::new();

        while !self.check(RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(RIGHT_BRACE, String::from("Expect '}' after block."));
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality();
        
        if self.is_match(&[EQUAL]) {
            let equals = self.previous();
            let value = self.assignment();

            if let Expr::Variable(expr) = expr {
                return Ok(Expr::Assign(Rc::new(AssignExpr{name: expr.name,  value: Rc::new(value?)}))) 
            } else {
                return Err(LoxError::error(equals.line, String::from("Invalid assignment target.")))
            }
        }
        return Ok(expr)
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.is_match(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right)}));
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.is_match(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right)}));
        }
        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.is_match(&[MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right)}));
        }

        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.is_match(&[SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Rc::new(BinaryExpr {left: Rc::new(expr), operator: operator, right: Rc::new(right)}));
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.is_match(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(Rc::new(UnaryExpr {operator: operator, right: Rc::new(right)})); 
        }
        return self.primary().unwrap();
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
            return Ok(Expr::Grouping(Rc::new(GroupingExpr {expression: Rc::new(expr)})));
        }

        let peek_var = self.peek();
        Err(self.error(peek_var, String::from("Expect expression.")))
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