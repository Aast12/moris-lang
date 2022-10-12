use core::panic;
use std::collections::HashMap;

use crate::ast::types::{DataType, FunctionSignature};

#[derive(Debug)]
pub enum SymbolType {
    Variable,
}

#[derive(Debug)]
pub struct SymbolEntry {
    pub id: String,
    pub symbol_type: SymbolType,
    pub data_type: DataType,
    pub dimension: usize,
    pub shape: Vec<i32>,
}

#[derive(Debug)]
pub struct EnvEntry {
    pub env_id: String,
    pub return_type: Option<DataType>,
    pub symbols: HashMap<String, SymbolEntry>,
}

#[derive(Debug)]
pub struct Environment {
    pub current_env: String,
    pub entries: HashMap<String, EnvEntry>,
}

impl Environment {
    pub fn new() -> Environment {
        return Environment {
            current_env: String::from("global"),
            entries: HashMap::from([(
                String::from("global"),
                EnvEntry {
                    env_id: String::from("global"),
                    return_type: None,
                    symbols: HashMap::new(),
                },
            )]),
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

    pub fn switch(&mut self, id: String) {
        if let Some(_) = self.entries.get(&id) {
            self.current_env = id;
        } else {
            panic!("Environment {} does not exist!", id);
        }
    }

    pub fn from_function(&mut self, id: String, func: FunctionSignature, switch: bool) {
        if let Some(_) = self.entries.get(&id) {
            panic!("Environment {} already exist!", id);
        }
        self.entries.insert(id.clone(), EnvEntry::from_func(func));

        if switch {
            self.current_env = id;
        }
    }

    pub fn add_var(&mut self, id: String, data_type: DataType) {
        self.current_env_mut().add(SymbolEntry::new_var(id, data_type));
    }

    pub fn get_var(&self, id: String) -> Option<&SymbolEntry> {
        return self.current_env().get(id);
    }
}

impl EnvEntry {
    pub fn new(env_id: String, return_type: Option<DataType>) -> EnvEntry {
        EnvEntry {
            env_id,
            return_type,
            symbols: HashMap::new(),
        }
    }

    pub fn from_func(func: FunctionSignature) -> EnvEntry {
        let mut symbols: HashMap<String, SymbolEntry> = HashMap::new();

        for param in func.params.iter() {
            let key = param.0.clone();
            let val = SymbolEntry::new_var(param.0.clone(), param.1.clone());
            symbols.insert(key, val);
        }

        EnvEntry {
            env_id: func.id,
            return_type: Some(func.data_type),
            symbols,
        }
    }

    pub fn add(&mut self, symbol: SymbolEntry) {
        let id = symbol.id.clone();
        if let Some(_) = &self.symbols.insert(id.clone(), symbol) {
            panic!("{} was already defined!", id.clone());
        }
    }

    pub fn get(&self, id: String) -> Option<&SymbolEntry> {
        return self.symbols.get(&id);
    }
}

impl SymbolEntry {
    pub fn new_var(id: String, data_type: DataType) -> SymbolEntry {
        SymbolEntry {
            id,
            symbol_type: SymbolType::Variable,
            data_type,
            dimension: 0,
            shape: vec![],
        }
    }

    pub fn new_vec(id: String, data_type: DataType, shape: Vec<i32>) -> SymbolEntry {
        SymbolEntry {
            id,
            symbol_type: SymbolType::Variable,
            data_type,
            dimension: shape.len(),
            shape,
        }
    }
}
