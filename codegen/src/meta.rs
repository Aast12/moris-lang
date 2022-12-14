use memory::resolver::MemAddress;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{symbols::FunctionEntry, quadruples::Quadruple};

/// Structure to dump the needed program's data for its 
/// execution.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramMeta {
    pub quadruples: Vec<Quadruple>,
    pub constant_table: HashMap<MemAddress, String>,
    pub procedure_table: HashMap<String, FunctionEntry>,
}

impl ProgramMeta {
    pub fn get_func(&self, id: &String) -> &FunctionEntry {
        self.procedure_table.get(id).unwrap()
    }
}
