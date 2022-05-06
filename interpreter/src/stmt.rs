use crate::error::*;
use crate::expr::*;
use crate::token::*;
use std::rc::Rc;

pub enum Stmt {
    Block(Rc<BlockStmt>),
    Break(Rc<BreakStmt>),
    Continue(Rc<ContinueStmt>),
    Expression(Rc<ExpressionStmt>),
    Function(Rc<FunctionStmt>),
    If(Rc<IfStmt>),
    Print(Rc<PrintStmt>),
    Var(Rc<VarStmt>),
    While(Rc<WhileStmt>),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block(v) => v.accept(stmt_visitor),
            Stmt::Break(v) => v.accept(stmt_visitor),
            Stmt::Continue(v) => v.accept(stmt_visitor),
            Stmt::Expression(v) => v.accept(stmt_visitor),
            Stmt::Function(v) => v.accept(stmt_visitor),
            Stmt::If(v) => v.accept(stmt_visitor),
            Stmt::Print(v) => v.accept(stmt_visitor),
            Stmt::Var(v) => v.accept(stmt_visitor),
            Stmt::While(v) => v.accept(stmt_visitor),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<T, LoxError>;
    fn visit_break_stmt(&self, stmt: &BreakStmt) -> Result<T, LoxError>;
    fn visit_continue_stmt(&self, stmt: &ContinueStmt) -> Result<T, LoxError>;
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<T, LoxError>;
    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> Result<T, LoxError>;
    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<T, LoxError>;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<T, LoxError>;
    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<T, LoxError>;
    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<T, LoxError>;
}

pub struct BlockStmt {
    pub statements: Rc<Vec<Rc<Stmt>>>,
}

pub struct BreakStmt {
    pub token: Token,
}

pub struct ContinueStmt {
    pub token: Token,
}

pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

pub struct FunctionStmt {
    pub name: Token,
    pub params: Rc<Vec<Token>>,
    pub body: Rc<Vec<Rc<Stmt>>>,
}

pub struct IfStmt {
    pub condition: Rc<Expr>,
    pub then_branch: Rc<Stmt>,
    pub else_branch: Option<Rc<Stmt>>,
}

pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Rc<Expr>>,
}

pub struct WhileStmt {
    pub condition: Rc<Expr>,
    pub body: Rc<Stmt>,
    pub is_for_loop: bool,
}

impl BlockStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_block_stmt(self)
    }
}

impl BreakStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_break_stmt(self)
    }
}

impl ContinueStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_continue_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl FunctionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_function_stmt(self)
    }
}

impl IfStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_if_stmt(self)
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

impl WhileStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_while_stmt(self)
    }
}

