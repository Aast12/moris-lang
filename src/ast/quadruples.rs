use core::panic;
use lazy_static::lazy_static;
// 1.4.0
use std::{
    collections::{HashMap, LinkedList},
    fmt::{Debug, Error, Formatter},
    hash::Hash,
    sync::{Mutex, MutexGuard},
};

// use crate::{moris_lang::environ::Environment, symbols::SymbolTable};
use crate::{
    env::Environment,
    semantics::{ExitStatement, SemanticContext, SemanticRules},
};

use super::{temp::Temp, types::DataType};

lazy_static! {
    pub static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::new());
}

#[derive(Debug)]
pub struct Manager {
    pub env: Environment,
    instruction_counter: i32,
    pub quadruples: Vec<Quadruple>,
    pub unresolved: HashMap<ExitStatement, Vec<usize>>,
    temp_counter: i32,
}

impl<'m> Manager {
    pub fn new() -> Self {
        Manager {
            temp_counter: 0,
            instruction_counter: 0,
            quadruples: vec![],
            env: Environment::new(),
            unresolved: HashMap::new(),
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

pub struct GlobalManager {}

impl GlobalManager {
    pub fn get() -> MutexGuard<'static, Manager> {
        if let Ok(manager) = MANAGER.try_lock() {
            manager
        } else {
            panic!("Manager lock could not be acquired!");
        }
    }

    pub fn prepare_exit_stmt(stmt_type: &ExitStatement) {
        let mut instance = Self::get();
        let stmt_position = instance.get_next_id();
        instance.emit(Quadruple::new_empty());

        if let Some(context) = instance.unresolved.get_mut(&stmt_type) {
            context.push(stmt_position);
        } else {
            instance.unresolved.insert(*stmt_type, vec![stmt_position]);
        }
    }

    pub fn resolve_context(stmt_type: &ExitStatement, quadruple: Quadruple) {
        let mut instance = Self::get();

        let unresolved = instance
            .unresolved
            .get(stmt_type)
            .unwrap_or(&vec![])
            .clone();

        for ref_quadruple in unresolved {
            if let Some(to_update) = instance.quadruples.get_mut(ref_quadruple) {
                quadruple.clone_into(to_update);
            }
        }

        drop(instance);
        Self::clean_exit_stmt(stmt_type);
    }

    pub fn clean_exit_stmt(stmt_type: &ExitStatement) {
        let mut instance = Self::get();
        instance.unresolved.insert(*stmt_type, vec![]);
    }

    pub fn emit(quadruple: Quadruple) {
        Self::get().emit(quadruple);
    }

    pub fn get_next_pos() -> usize {
        Self::get().get_next_id()
    }
}

#[derive(Clone)]
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

        QuadrupleHold { position }
    }

    pub fn release(&mut self, value: Quadruple) {
        if let Ok(mut manager) = MANAGER.try_lock() {
            manager.update_instruction(self.position, value);
        } else {
            panic!("Manager lock could not be acquired!");
        }
    }
}
