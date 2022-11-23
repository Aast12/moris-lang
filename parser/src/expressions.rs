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

#[derive(Debug, Clone)]
pub enum Index {
    Simple(Box<Expression>),
}
