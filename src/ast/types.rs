use crate::ast;

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

#[derive(Debug)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
    DataFrame,
    Void,
    Function(Box<ast::FunctionSignature>),
}
