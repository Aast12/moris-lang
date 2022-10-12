use super::{
    expressions::Expression, node::Node, quadruples::MANAGER, statements::Block, Dimension,
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

impl Operator {
    pub fn to_string(&self) -> &str {
        match self {
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Pipe => "|>",
            Operator::ForwardPipe => "|> fwd",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::LessThan => "<",
            Operator::GreaterThan => ">",
            Operator::NotEq => "!=",
            Operator::Eq => "==",
            Operator::Assign => "=",
        }
    }
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
pub struct Variable {
    pub id: String,
    pub data_type: DataType,
    pub dimension: Dimension,
    pub value: Option<Box<Expression>>,
}

impl<'m> Variable {
    pub fn new(
        id: String,
        data_type: DataType,
        dimension: Dimension,
        value: Option<Box<Expression>>,
    ) -> Variable {
        Variable {
            id,
            data_type,
            dimension,
            value,
        }
    }
}

impl<'m> Node<'m> for Variable {
    fn generate(&mut self) -> () {
        MANAGER
            .lock()
            .unwrap()
            .get_env()
            .add_var(self.id.clone(), self.data_type.clone());
    }

    fn reduce(&self) -> String {
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
pub struct Function {
    pub signature: FunctionSignature,
    pub block: Block,
}

impl<'m> Function {
    pub fn new(signature: FunctionSignature, block: Block) -> Function {
        Function { signature, block }
    }
}

impl<'m> Node<'m> for Function {
    fn generate(&mut self) -> () {
        {
            MANAGER.lock().unwrap().get_env().from_function(
                self.signature.id.clone(),
                self.signature.clone(),
                false,
            );
        }

        self.block.generate();
    }

    fn reduce(&self) -> String {
        todo!("Function reduce!");
    }
}
