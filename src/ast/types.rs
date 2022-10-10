use crate::ast;

use super::{expressions::Expression, statements::Block};

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
    Pipe,
    ForwardPipe,
    And,
    Or,
    LessThan,
    GreaterThan,
    NotEq,
    Eq,
    Assign,
}

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
    DataFrame,
    Void,
    Function(Box<FunctionSignature>),
}

#[derive(Debug)]
pub struct Variable<'m> {
    pub id: String,
    pub data_type: DataType,
    pub dimension: ast::Dimension<'m>,
    pub value: Option<Box<Expression<'m>>>,
}

#[derive(Debug)]
pub struct FunctionSignature {
    pub id: String,
    pub data_type: DataType,
    pub params: Vec<FunctionParam>,
}


#[derive(Debug)]
pub struct FunctionParam(pub String, pub DataType);

#[derive(Debug)]
pub struct Function<'m> {
    pub signature: FunctionSignature,
    pub block: Block<'m>,
}
