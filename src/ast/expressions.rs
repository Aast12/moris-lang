use crate::ast;

use super::types;
pub mod constant;
pub mod id;
pub mod operation;

#[derive(Debug)]
pub enum Expr {
    Const(constant::TypeConst),
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

pub trait Expression<'m>: ast::node::Node<'m> {}

// impl<'m> ast::node::Node<'m> for dyn Expression { }
