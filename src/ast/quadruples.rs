use crate::symbols::SymbolTable;

use super::{temp::Temp, types::DataType};
// use crate::parser::
#[derive(Debug)]
pub struct Manager<'m> {
    temp_counter: i32,
    instruction_counter: i32,
    quadruples: Vec<Quadruple>,
    symbols: SymbolTable<'m>,
}

impl<'m> Manager<'m> {
    pub fn new() -> Self {
        Manager {
            temp_counter: 0,
            instruction_counter: 0,
            quadruples: vec![],
            symbols: SymbolTable::new(),
        }
    }

    pub fn new_temp(&mut self, data_type: DataType) -> Temp {
        self.temp_counter += 1;
        return Temp::new(self.temp_counter - 1, data_type);
    }

    pub fn emit(&mut self, quadruple: Quadruple) {
        self.quadruples.push(quadruple);
        self.instruction_counter += 1;
    }
}

#[derive(Debug)]
pub struct Quadruple(String, String, String, String);
