use crate::{ast::expressions::constant::Const, memory::resolver::MemAddress};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeTupleStruct};
use std::collections::HashMap;

use super::{function::FunctionEntry, quadruples::Quadruple};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramMeta {
    pub quadruples: Vec<Quadruple>,
    pub constant_table: HashMap<MemAddress, String>,
    pub procedure_table: HashMap<String, FunctionEntry>,
}

// impl Serialize for Quadruple {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut ts = serializer.serialize_tuple_struct("Quadruple", 4)?;
//         ts.serialize_field(&self.0);
//         ts.serialize_field(&self.1);
//         ts.serialize_field(&self.2);
//         ts.serialize_field(&self.3);
//         ts.end()
//     }
// }
