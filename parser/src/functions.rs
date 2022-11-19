use memory::types::DataType;

use crate::{statements::Block, types::Variable, Dimension};

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub id: String,
    pub data_type: DataType,
    pub params: Vec<FunctionParam>,
    pub is_native: bool,
}

#[derive(Debug, Clone)]
pub struct FunctionParam(pub Variable);

impl FunctionParam {
    pub fn new_scalar(id: &str, data_type: DataType) -> FunctionParam {
        FunctionParam(Variable{
            id: id.to_string(),
            data_type,
            dimension: Dimension::new_scalar(),
            value: None
        })
    }
}

#[derive(Debug)]
pub struct Function {
    pub signature: FunctionSignature,
    pub block: Block,
}

impl Function {
    pub fn new(signature: FunctionSignature, block: Block) -> Function {
        Function { signature, block }
    }
}
