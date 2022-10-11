use crate::ast;

use super::{
    expressions::Expression, node::Node, quadruples::Manager, statements::Block, Dimension,
};

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
    Function(Box<DataType>),
}

#[derive(Debug)]
pub struct Variable<'m> {
    manager: Option<&'m Manager<'m>>,
    pub id: String,
    pub data_type: DataType,
    pub dimension: ast::Dimension<'m>,
    pub value: Option<Box<Expression<'m>>>,
}

impl<'m> Variable<'m> {
    pub fn new(
        id: String,
        data_type: DataType,
        dimension: Dimension<'m>,
        value: Option<Box<Expression<'m>>>,
    ) -> Variable<'m> {
        Variable {
            manager: None,
            id,
            data_type,
            dimension,
            value,
        }
    }
}

impl<'m> Node<'m> for Variable<'m> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager<'m>) -> () {
        self.manager = Some(&manager);
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
