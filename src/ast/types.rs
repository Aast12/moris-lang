use crate::ast;

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
