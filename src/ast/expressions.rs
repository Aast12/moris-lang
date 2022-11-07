use crate::{ast, memory::types::DataType};

use self::{
    call::Call,
    constant::Const,
    id::{Access, Id},
    operation::Operation,
};

use super::{
    node,
    types::{self},
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
    Not(Box<Expression>),
    Negative(Box<Expression>),
}

impl Expression {
    // TODO: optimize data type resolution
    pub fn data_type(&self) -> DataType {
        match &self {
            Expression::Const(constant) => constant.dtype.clone(),
            Expression::Op(operation) => operation.data_type(),
            Expression::Access(access) => access.id.data_type(),
            Expression::Id(id) => id.data_type(),
            Expression::Call(call) => call.data_type(),
            Expression::Not(expr) => expr.data_type(),
            Expression::Negative(expr) => expr.data_type(),
        }
    }
}

impl node::Node for Expression {
    fn generate(&mut self) -> () {
        match self {
            Expression::Const(constant) => constant.generate(),
            Expression::Op(operation) => operation.generate(),
            Expression::Access(access) => access.generate(),
            Expression::Id(id) => id.generate(),
            Expression::Call(call) => call.generate(),
            Expression::Not(_) => todo!(),
            Expression::Negative(_) => todo!(),
        }
    }

    fn reduce(&self) -> String {
        match self {
            Expression::Const(constant) => constant.reduce(),
            Expression::Op(operation) => operation.reduce(),
            Expression::Access(access) => access.reduce(),
            Expression::Id(id) => id.reduce(),
            Expression::Call(call) => call.reduce(),
            Expression::Not(_) => todo!(),
            Expression::Negative(_) => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum Index {
    Simple(Box<Expression>),
    Range(Box<Expression>, Box<Expression>),
}

impl ast::node::Node for Index {}

#[cfg(test)]
mod tests {
    use crate::{ast::types::Operator, memory::types::DataType};

    use super::{constant::Const, operation::Operation, Expression};

    fn build_int() -> Box<Expression> {
        Box::new(Expression::Const(Const::new("5", DataType::Int)))
    }

    fn build_float() -> Box<Expression> {
        Box::new(Expression::Const(Const::new("5.0", DataType::Float)))
    }

    fn build_string() -> Box<Expression> {
        Box::new(Expression::Const(Const::new("str", DataType::String)))
    }

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
