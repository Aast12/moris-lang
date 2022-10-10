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
pub enum Index<T> {
    Simple(Box<T>),
    Range(Box<T>, Box<T>),
}

pub trait Expression<'m>: ast::node::Node<'m> {}

impl<'m, T: Expression<'m>> ast::node::Node<'m> for Index<T> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager) -> () {
        match self {
            Self::Simple(expr) => expr.set_manager(manager),
            Self::Range(start_expr, end_expr) => {
                start_expr.set_manager(manager);
                end_expr.set_manager(manager);
            }
        }
    }
}
