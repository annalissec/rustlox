use crate::error::*;
use crate::expr::*;
use crate::token::*;
use std::rc::Rc;

pub enum Stmt {
    Block(Rc<BlockStmt>),
    Expression(Rc<ExpressionStmt>),
    Print(Rc<PrintStmt>),
    Var(Rc<VarStmt>),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block(v) => v.accept(stmt_visitor),
            Stmt::Expression(v) => v.accept(stmt_visitor),
            Stmt::Print(v) => v.accept(stmt_visitor),
            Stmt::Var(v) => v.accept(stmt_visitor),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, expr: &BlockStmt) -> Result<T, LoxError>;
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T, LoxError>;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<T, LoxError>;
    fn visit_var_stmt(&self, expr: &VarStmt) -> Result<T, LoxError>;
}

pub struct BlockStmt {
    pub statements: Rc<Vec<Rc<Stmt>>>,
}

pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Rc<Expr>>,
}

impl BlockStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_block_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_print_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_var_stmt(self)
    }
}

