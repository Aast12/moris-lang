use crate::ast;
pub mod id;

#[derive(Debug)]
pub enum Expr {
    Const(ast::TypeConst),
    Op(Box<Expr>, ast::types::Operator, Box<Expr>),
    ParenthOp(Box<Expr>),
    Var(ast::VarRef),
    FunctionCall(String, Vec<Box<Expr>>),
    Error,
}

#[derive(Debug)]
pub enum Index {
    Simple(Box<ast::Expr>),
    Range(Box<ast::Expr>, Box<ast::Expr>),
}

pub struct Operation {

}

