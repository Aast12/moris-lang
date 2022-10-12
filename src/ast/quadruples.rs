use core::panic;
use lazy_static::lazy_static;
// 1.4.0
use std::{
    fmt::{Debug, Error, Formatter},
    sync::Mutex,
};

// use crate::{moris_lang::environ::Environment, symbols::SymbolTable};
use crate::env::Environment;

use super::{temp::Temp, types::DataType};

lazy_static! {
    pub static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::new());
}

#[derive(Debug)]
pub struct Manager {
    temp_counter: i32,
    instruction_counter: i32,
    pub quadruples: Vec<Quadruple>,
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
        return &mut self.env;
    }

    pub fn new_temp(&mut self, data_type: &DataType) -> Temp {
        self.temp_counter += 1;
        let tmp = Temp::new(self.temp_counter - 1, data_type.clone());

        return tmp;
    }

    pub fn emit(&mut self, quadruple: Quadruple) {
        self.quadruples.push(quadruple);
        self.instruction_counter += 1;
    }

    pub fn get_next_id(&self) -> usize {
        return self.quadruples.len();
    }

    pub fn update_instruction(&mut self, id: usize, quad: Quadruple) {
        if let Some(local) = self.quadruples.get_mut(id) {
            *local = quad;
        }
    }
}

pub struct Quadruple(pub String, pub String, pub String, pub String);

impl Quadruple {
    pub fn new(fst: &str, snd: &str, thrd: &str, fth: &str) -> Quadruple {
        Quadruple(
            String::from(fst),
            String::from(snd),
            String::from(thrd),
            String::from(fth),
        )
    }

    pub fn new_empty() -> Quadruple {
        Quadruple(String::new(), String::new(), String::new(), String::new())
    }

    pub fn update(&mut self, item: usize, value: String) {
        match item {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            3 => self.3 = value,
            _ => panic!("Index {} out of quadruple bounds.", item),
        }
    }
}

impl Debug for Quadruple {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        return write!(fmt, "{}\t{}\t{}\t{}\t", self.0, self.1, self.2, self.3);
    }
}
