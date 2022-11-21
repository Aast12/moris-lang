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
    // pub fn data_type(&self, manager: &mut Manager) -> DataType {
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
