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
pub enum Expression {
    Const(Const),
    Op(Operation),
    Access(Access),
    Id(Id),
    Call(Call),
}

impl<'m> Expression {
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

impl<'m> node::Node<'m> for Expression {
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

    fn reduce(&self) -> String {
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
pub enum Index {
    Simple(Box<Expression>),
    Range(Box<Expression>, Box<Expression>),
}

pub trait ExpressionT<'m>: ast::node::Node<'m> {}

impl<'m> ast::node::Node<'m> for Index {}

#[cfg(test)]
mod tests {
    use crate::ast::{
        node::Node,
        quadruples::Manager,
        types::{DataType, Operator},
    };

    use super::{constant::Const, operation::Operation, Expression};

    fn build_int<'m>() -> Box<Expression> {
        Box::new(Expression::Const(Const::new("5", DataType::Int)))
    }

    fn build_float<'m>() -> Box<Expression> {
        Box::new(Expression::Const(Const::new("5.0", DataType::Float)))
    }

    fn build_string<'m>() -> Box<Expression> {
        Box::new(Expression::Const(Const::new("str", DataType::String)))
    }

    // fn expect_fail(in_str: &str) {
    //     let parser = get_parser();
    //     assert!(std::panic::catch_unwind(|| parser.parse(in_str).unwrap()).is_err());
    // }

    #[test]
    fn test_compatible_types() {
        let expr = Expression::Op(Operation::new(build_int(), Operator::Add, build_float()));

        assert_eq!(expr.data_type(), DataType::Float);
    }

    #[test]
    #[should_panic]
    fn test_incompatible_types() {
        let expr = Expression::Op(Operation::new(build_string(), Operator::Add, build_float()));

        expr.data_type();
    }
}
