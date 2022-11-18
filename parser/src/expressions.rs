use self::{
    call::Call,
    constant::Const,
    id::{Access, Id},
    operation::Operation,
};

pub mod call;
pub mod constant;
pub mod id;
pub mod operation;

#[derive(Debug, Clone)]
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
    // pub fn data_type(&self) -> DataType {
    //     match &self {
    //         Expression::Const(constant) => constant.dtype.clone(),
    //         Expression::Op(operation) => operation.data_type(),
    //         Expression::Access(access) => access.id.data_type(),
    //         Expression::Id(id) => id.data_type(),
    //         Expression::Call(call) => call.data_type(),
    //         Expression::Not(expr) => expr.data_type(),
    //         Expression::Negative(expr) => expr.data_type(),
    //     }
    // }
}

#[derive(Debug, Clone)]
pub enum Index {
    Simple(Box<Expression>),
    Range(Box<Expression>, Box<Expression>),
}

#[cfg(test)]
mod tests {
    use crate::ast::types::Operator;
    use memory::types::DataType;

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
