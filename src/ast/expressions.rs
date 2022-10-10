use crate::ast;

use self::{constant::Const, operation::Operation, id::{Access, Id}, call::Call};

use super::{types, node};
pub mod constant;
pub mod id;
pub mod operation;
pub mod call;

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
pub enum Expression<'m> {
    Const(Const<'m>),
    Op(Operation<'m>),
    Access(Access<'m>),
    Id(Id<'m>),
    Call(Call<'m>)
}

impl<'m> node::Node<'m> for Expression<'m> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager) -> () {
        match self {
            Expression::Const(constant) => constant.set_manager(manager), 
            Expression::Op(operation) => operation.set_manager(manager),
            Expression::Access(access) => access.set_manager(manager),
            Expression::Id(id) => id.set_manager(manager),
            Expression::Call(call) => call.set_manager(manager),
        }
    }
}

#[derive(Debug)]
pub enum Index<'m> {
    Simple(Box<Expression<'m>>),
    Range(Box<Expression<'m>>, Box<Expression<'m>>),
}

pub trait ExpressionT<'m>: ast::node::Node<'m> {}

impl<'m> ast::node::Node<'m> for Index<'m> {
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
