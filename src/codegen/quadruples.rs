use core::panic;
use std::fmt::{Debug, Error, Formatter};

use serde::{Deserialize, Serialize};

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

    pub fn jump_check(instruction: &str, check: &str, position: usize) -> Quadruple {
        Quadruple::new(instruction, check, "", position.to_string().as_str())
    }

    pub fn jump(instruction: &str, position: usize) -> Quadruple {
        Quadruple::new(instruction, "", "", position.to_string().as_str())
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

    pub fn new_coded(key: &str) -> Quadruple {
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
