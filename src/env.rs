use core::panic;
use std::collections::HashMap;

use crate::{
    ast::functions::{FunctionParam, FunctionSignature},
    memory::{
        resolver::{MemAddress, MemoryScope},
        types::DataType,
        virtual_allocator::VirtualAllocator,
    },
};

#[derive(Debug)]
pub enum SymbolType {
    Variable,
}

#[derive(Debug)]
pub struct SymbolEntry {
    pub id: String,
    pub address: MemAddress,
    pub symbol_type: SymbolType,
    pub data_type: DataType,
    pub dimension: usize,
    pub shape: Vec<i32>,
}

#[derive(Debug)]
pub struct EnvEntry {
    is_global: bool,
    pub env_id: String,
    pub return_type: Option<DataType>,
    pub symbols: HashMap<String, SymbolEntry>,
}

#[derive(Debug)]
pub struct Environment {
    pub current_env: String,
    pub entries: HashMap<String, EnvEntry>,
    pub current_scope: MemoryScope,
    pub allocator: VirtualAllocator,
}

impl Environment {
    pub fn new() -> Environment {
        return Environment {
            current_env: String::from("global"),
            entries: HashMap::from([(
                String::from("global"),
                EnvEntry {
                    is_global: true,
                    env_id: String::from("global"),
                    return_type: None,
                    symbols: HashMap::new(),
                },
            )]),
            allocator: VirtualAllocator::new(),
            current_scope: MemoryScope::Global,
        };
    }

    fn current_env_mut(&mut self) -> &mut EnvEntry {
        if let Some(env) = self.entries.get_mut(&self.current_env) {
            return env;
        }
        panic!("Current environment does not exist!");
    }

    fn current_env(&self) -> &EnvEntry {
        if let Some(env) = self.entries.get(&self.current_env) {
            return env;
        }
        panic!("Current environment does not exist!");
    }

    pub fn drop_env(&mut self, id: &String) {
        if self.current_env == *id {
            panic!("Cannot drop current env {}", id)
        }

        self.entries.remove(id);
    }

    pub fn switch(&mut self, id: &String) {
        if let Some(_) = self.entries.get(id) {
            self.current_env = id.clone();
            if id == "global" {
                self.current_scope = MemoryScope::Global;
            } else {
                self.current_scope = MemoryScope::Local;
            }
        } else {
            panic!("Environment {} does not exist!", id);
        }
    }

    pub fn from_function(&mut self, func: &FunctionSignature, switch: bool) {
        let id = &func.id;
        if let Some(_) = self.entries.get(id) {
            panic!("Environment {} already exist!", id);
        }

        if switch {
            self.current_env = id.clone();
        }

        self.current_scope = MemoryScope::Local;
        self.allocator.reset_locals();

        self.entries
            .insert(id.clone(), EnvEntry::from_func(func, &mut self.allocator));
    }

    pub fn add_var(&mut self, id: &String, data_type: &DataType) {
        let address = self
            .allocator
            .assign_location(&self.current_scope, data_type);

        self.current_env_mut()
            .add(SymbolEntry::new_var(id.clone(), data_type.clone(), address));
    }

    pub fn del_var(&mut self, id: &String) {
        self.current_env_mut().delete(id);
    }

    pub fn get_var(&self, id: &String) -> Option<&SymbolEntry> {
        if let Some(symbol) = self.current_env().get(id) {
            return Some(symbol);
        } else {
            if !self.current_env().is_global {
                return self.entries.get("global").unwrap().get(id);
            }
        }

        return None;
    }
}

impl EnvEntry {
    pub fn new(env_id: String, return_type: Option<DataType>) -> EnvEntry {
        EnvEntry {
            is_global: false,
            env_id,
            return_type,
            symbols: HashMap::new(),
        }
    }

    pub fn from_func(func: &FunctionSignature, allocator: &mut VirtualAllocator) -> EnvEntry {
        let mut symbols: HashMap<String, SymbolEntry> = HashMap::new();

        for FunctionParam(id, data_type) in func.params.iter() {
            let key = id.clone();
            let val = SymbolEntry::new_var(
                id.clone(),
                data_type.clone(),
                allocator.assign_location(&MemoryScope::Local, &data_type),
            );

            symbols.insert(key, val);
        }

        EnvEntry {
            is_global: false,
            env_id: func.id.clone(),
            return_type: Some(func.data_type.clone()),
            symbols,
        }
    }

    pub fn add(&mut self, symbol: SymbolEntry) {
        let id = symbol.id.clone();
        if let Some(_) = &self.symbols.insert(id.clone(), symbol) {
            panic!("{} was already defined!", id.clone());
        }
    }

    pub fn delete(&mut self, symbol_id: &String) {
        self.symbols.remove(symbol_id);
    }

    pub fn get(&self, id: &String) -> Option<&SymbolEntry> {
        return self.symbols.get(id);
    }
}

impl SymbolEntry {
    pub fn new_var(id: String, data_type: DataType, address: MemAddress) -> SymbolEntry {
        SymbolEntry {
            id,
            symbol_type: SymbolType::Variable,
            data_type,
            dimension: 0,
            shape: vec![],
            address,
        }
    }

    pub fn new_vec(
        id: String,
        data_type: DataType,
        shape: Vec<i32>,
        address: MemAddress,
    ) -> SymbolEntry {
        SymbolEntry {
            id,
            symbol_type: SymbolType::Variable,
            data_type,
            dimension: shape.len(),
            shape,
            address,
        }
    }
}
