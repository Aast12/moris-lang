use std::borrow::{Borrow, BorrowMut};

// use crate::{moris_lang::environ::Environment, symbols::SymbolTable};
use crate::env::Environment;

use super::{temp::Temp, types::DataType};
// use crate::parser::
#[derive(Debug)]
pub struct Manager {
    temp_counter: i32,
    instruction_counter: i32,
    quadruples: Vec<Quadruple>,
    pub env: Environment,
}

impl<'m> Manager {
    pub fn new() -> Self {
        Manager {
            temp_counter: 0,
            instruction_counter: 0,
            quadruples: vec![],
            env: Environment::new(),
        }
    }

    pub fn get_env(&mut self) -> &mut Environment {
        return self.env.borrow_mut()
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
