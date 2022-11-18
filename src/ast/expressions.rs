use crate::{
    ast,
    codegen::{manager::GlobalManager, quadruples::Quadruple},
    memory::{resolver::MemoryResolver, types::DataType},
};

use self::{
    call::Call,
    constant::Const,
    id::{Access, Id},
    operation::Operation,
};

use super::{
    node,
    types::{self, Operator},
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
            Expression::Not(not) => not.generate(),
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
            Expression::Not(not) => {
                let mut to_negate = not.reduce();
                let expr_type =
                    MemoryResolver::get_type_from_address(to_negate.parse().unwrap()).unwrap();

                if DataType::equivalent(expr_type, &DataType::Bool).is_err() {
                    panic!("Expression can't be casted to boolean");
                }
                if *expr_type != DataType::Bool {
                    to_negate = GlobalManager::emit_cast(&DataType::Bool, &to_negate);
                }

                let dest = GlobalManager::new_temp(&DataType::Bool).to_string();
                GlobalManager::emit(Quadruple::unary(
                    Operator::Not,
                    to_negate.as_str(),
                    dest.as_str(),
                ));

                dest
            }
            Expression::Negative(expr) => {
                let addr = expr.reduce();
                let expr_dt = expr.data_type();
                let new_addr = GlobalManager::new_temp(&expr_dt).to_string();

                GlobalManager::emit(Quadruple::unary(Operator::Neg, &addr, &new_addr));

                new_addr
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Index {
    Simple(Box<Expression>),
    Range(Box<Expression>, Box<Expression>),
}

impl ast::node::Node for Index {
    fn reduce(&self) -> String {
        match self {
            Self::Simple(idx) => idx.reduce(),
            Self::Range(_, _) => panic!("Range not supported"), // TODO
        }
    }
}

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
