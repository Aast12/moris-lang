use crate::ast;

use self::{
    call::Call,
    constant::Const,
    id::{Access, Id},
    operation::Operation,
};

use super::{
    node,
    types::{self, DataType},
};
pub mod call;
pub mod constant;
pub mod id;
pub mod operation;

#[derive(Debug)]
pub enum Expression<'m> {
    Const(Const<'m>),
    Op(Operation<'m>),
    Access(Access<'m>),
    Id(Id<'m>),
    Call(Call<'m>),
}

impl<'m> Expression<'m> {
    fn data_type(&self) -> DataType {
        match &self {
            Expression::Const(constant) => constant.dtype.clone(),
            Expression::Op(operation) => operation.data_type(),
            Expression::Access(access) => access.id.data_type(),
            Expression::Id(id) => id.data_type(),
            Expression::Call(call) => call.data_type(),
        }
    }
}

impl<'m> node::Node<'m> for Expression<'m> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager<'m>) -> () {
        match self {
            Expression::Const(constant) => constant.set_manager(manager),
            Expression::Op(operation) => operation.set_manager(manager),
            Expression::Access(access) => access.set_manager(manager),
            Expression::Id(id) => id.set_manager(manager),
            Expression::Call(call) => call.set_manager(manager),
            _ => todo!(),
        }
    }

    fn generate(&mut self) -> () {
        match self {
            Expression::Const(constant) => constant.generate(),
            Expression::Op(operation) => operation.generate(),
            Expression::Access(access) => access.generate(),
            Expression::Id(id) => id.generate(),
            Expression::Call(call) => call.generate(),
            _ => todo!(),
        }
    }

    fn reduce(&self) -> &dyn node::Leaf {
        match self {
            Expression::Const(constant) => constant.reduce(),
            Expression::Op(operation) => operation.reduce(),
            Expression::Access(access) => access.reduce(),
            Expression::Id(id) => id.reduce(),
            Expression::Call(call) => call.reduce(),
            _ => todo!(),
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
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager<'m>) -> () {
        match self {
            Self::Simple(expr) => expr.set_manager(manager),
            Self::Range(start_expr, end_expr) => {
                start_expr.set_manager(manager);
                end_expr.set_manager(manager);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        node::Node,
        quadruples::Manager,
        types::{DataType, Operator},
    };

    use super::{constant::Const, operation::Operation, Expression};

    fn build_int<'m>() -> Box<Expression<'m>> {
        Box::new(Expression::Const(Const::new("5", DataType::Int)))
    }

    fn build_float<'m>() -> Box<Expression<'m>> {
        Box::new(Expression::Const(Const::new("5.0", DataType::Float)))
    }

    fn build_string<'m>() -> Box<Expression<'m>> {
        Box::new(Expression::Const(Const::new("str", DataType::String)))
    }

    // fn expect_fail(in_str: &str) {
    //     let parser = get_parser();
    //     assert!(std::panic::catch_unwind(|| parser.parse(in_str).unwrap()).is_err());
    // }

    #[test]
    fn test_compatible_types() {
        let manager = Manager::new();

        let mut expr = Expression::Op(Operation::new(build_int(), Operator::Add, build_float()));
        expr.set_manager(&manager);

        assert_eq!(expr.data_type(), DataType::Float);
    }

    #[test]
    #[should_panic]
    fn test_incompatible_types() {
        let manager = Manager::new();

        let mut expr = Expression::Op(Operation::new(build_string(), Operator::Add, build_float()));
        expr.set_manager(&manager);

        expr.data_type();
    }
}
