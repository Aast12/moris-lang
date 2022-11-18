use serde::{Serialize, Deserialize};

use crate::{
    ast::functions::Function,
    memory::{resolver::MemAddress, types::DataType},
};

pub type ParamAddress = (MemAddress, DataType, Option<MemAddress>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionEntry {
    pub id: String,
    pub return_type: DataType,
    pub params: Vec<ParamAddress>,
    pub procedure_address: usize,
    pub return_address: Option<MemAddress>,
}

impl FunctionEntry {
    pub fn new(
        address: usize,
        return_address: Option<MemAddress>,
        params_mapped: Vec<ParamAddress>,
        fn_definition: &Function,
    ) -> FunctionEntry {
        FunctionEntry {
            id: fn_definition.signature.id.to_owned(),
            procedure_address: address,
            return_type: fn_definition.signature.data_type.to_owned(),
            params: params_mapped,
            return_address,
        }
    }
}
