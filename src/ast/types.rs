use crate::ast;

use super::{expressions::Expression, statements::Block, node::Node};

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

#[derive(Debug, Clone, PartialEq)]
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

impl<'m> Node<'m> for Variable<'m> {
    fn set_manager(&mut self, _: &'m ast::quadruples::Manager) -> () {

    }

    fn generate(&mut self) -> () {
        
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub id: String,
    pub data_type: DataType,
    pub params: Vec<FunctionParam>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParam(pub String, pub DataType);

#[derive(Debug)]
pub struct Function<'m> {
    pub signature: FunctionSignature,
    pub block: Block<'m>,
}
