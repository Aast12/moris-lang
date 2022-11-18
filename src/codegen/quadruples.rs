use core::panic;
use std::fmt::{Debug, Error, Formatter};

use serde::{Deserialize, Serialize};

use crate::{ast::types::Operator, memory::types::DataType};

use super::manager::MANAGER;

#[derive(Serialize, Deserialize, Clone)]
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

    pub fn param(param_address: &str, index: usize) -> Quadruple {
        Quadruple::new("param", param_address, "", index.to_string().as_str())
    }

    pub fn era(id: &str) -> Quadruple {
        Quadruple::new("era", "", "", id)
    }

    pub fn type_cast(data_type: &DataType, origin: &str, dest: &str) -> Quadruple {
        Quadruple::new(format!("{:#?}", data_type).as_str(), origin, "", dest)
    }

    pub fn unary(operator: Operator, left: &str, dest: &str) -> Quadruple {
        let op = operator.to_string();
        Quadruple::new(op, left, "", dest)
    }

    pub fn operation(operator: Operator, left: &str, right: &str, dest: &str) -> Quadruple {
        let op = operator.to_string();
        Quadruple::new(op, left, right, dest)
    }

    pub fn verify(value: &str, bound: &str) -> Quadruple {
        Quadruple::new("ver", value, "", bound)
    }

    pub fn go_sub(id: &str) -> Quadruple {
        Quadruple::new("gosub", "", "", id)
    }

    pub fn goto_false(check: &str, position: usize) -> Quadruple {
        Quadruple::new("gotoFalse", check, "", position.to_string().as_str())
    }

    pub fn goto(position: usize) -> Quadruple {
        Quadruple::new("goto", "", "", position.to_string().as_str())
    }

    pub fn new_empty() -> Quadruple {
        Quadruple(String::new(), String::new(), String::new(), String::new())
    }

    pub fn new_return(id: &str) -> Quadruple {
        Quadruple(
            String::from("return"),
            String::new(),
            String::new(),
            String::from(id),
        )
    }

    pub fn void_return() -> Quadruple {
        Quadruple::new("voidReturn", "", "", "")
    }

    pub fn end_func() -> Quadruple {
        Self::new_coded("endFunc")
    }

    pub fn end_program() -> Quadruple {
        Self::new_coded("endProgram")
    }

    fn new_coded(key: &str) -> Quadruple {
        Quadruple(
            String::from(key),
            String::new(),
            String::new(),
            String::new(),
        )
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

pub struct QuadrupleHold {
    pub position: usize,
    pub released: bool,
}

impl QuadrupleHold {
    pub fn new() -> QuadrupleHold {
        let position: usize;
        if let Ok(mut manager) = MANAGER.try_lock() {
            position = manager.get_next_id();
            manager.emit(Quadruple::new_empty());
        } else {
            panic!("Manager lock could not be acquired!");
        }

        QuadrupleHold {
            position,
            released: false,
        }
    }

    pub fn release(&mut self, value: Quadruple) {
        if let Ok(mut manager) = MANAGER.try_lock() {
            manager.update_instruction(self.position, value);
            self.released = true;
        } else {
            panic!("Manager lock could not be acquired!");
        }
    }
}
