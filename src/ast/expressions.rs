use crate::ast;

use self::{constant::Const, operation::Operation, id::{Access, Id}, call::Call};

use super::{types, node};
pub mod constant;
pub mod id;
pub mod operation;
pub mod call;

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
            _ => todo!()
        }
    }

    fn generate(&self) -> () {
        match self {
            Expression::Const(constant) => constant.generate(), 
            Expression::Op(operation) => operation.generate(), 
            Expression::Access(access) => access.generate(), 
            Expression::Id(id) => id.generate(), 
            Expression::Call(call) => call.generate(), 
            _ => todo!()
        }
    }

    fn reduce(&self) -> &dyn node::Leaf {
        match self {
            Expression::Const(constant) => constant.reduce(),
            Expression::Op(operation) => operation.reduce(),
            Expression::Access(access) => access.reduce(),
            Expression::Id(id) => id.reduce(),
            Expression::Call(call) => call.reduce(),
            _ => todo!()
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
