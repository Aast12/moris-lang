use memory::{resolver::MemAddress, types::DataType};
use serde::{Deserialize, Serialize};

use parser::{functions::FunctionSignature};

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
        func: &FunctionSignature,
    ) -> FunctionEntry {
        FunctionEntry {
            id: func.id.to_owned(),
            procedure_address: address,
            return_type: func.data_type.to_owned(),
            params: params_mapped,
            return_address,
        }
    }
}
