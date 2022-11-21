use core::panic;
use std::collections::HashMap;

use parser::{
    functions::{FunctionParam, FunctionSignature},
    Dimension,
};

use memory::{
    resolver::{MemAddress, MemoryScope},
    types::DataType,
    virtual_allocator::VirtualAllocator,
};

use crate::symbols::SymbolEntry;

/// Represents an Environment entry to represent call contexts / scopes.
#[derive(Debug)]
pub struct EnvEntry {
    pub is_global: bool,
    pub env_id: String,
    pub return_type: Option<DataType>,
    pub symbols: HashMap<String, SymbolEntry>,
}

/// Manager for the current declaration environments.
///
/// Keeps track of the current environment and the allocation of
/// items within it. Also holds the data of multiple environments
/// and allow switching betweent them.
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

    pub fn current_env(&self) -> &EnvEntry {
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

    /// Switches the current environment to one with a given id ("global" or a function id).
    /// New entries will be declared under this new environment.
    pub fn switch(&mut self, id: &String) {
        if let Some(_) = self.entries.get(id) {
            self.current_env = id.clone();
            if id == "global" {
                self.current_scope = MemoryScope::Global;
            } else {
                self.current_scope = MemoryScope::Local;
                self.allocator.reset_locals();
                let current_context = self.entries.get(id).unwrap();
                let mut counters: HashMap<DataType, usize> = HashMap::new();

                // TODO: Store initial counters
                current_context.symbols.iter().for_each(|(_, entry)| {
                    if entry.dimension.size > 1 {
                        counters.insert(
                            DataType::Pointer,
                            counters.get(&DataType::Pointer).unwrap_or(&0) + 1,
                        );
                        counters.insert(
                            entry.data_type.clone(),
                            counters.get(&entry.data_type.clone()).unwrap_or(&0)
                                + entry.dimension.size,
                        );
                    } else {
                        counters.insert(
                            entry.data_type.clone(),
                            counters.get(&entry.data_type.clone()).unwrap_or(&0) + 1,
                        );
                    }
                });

                counters
                    .iter()
                    .for_each(|(k, v)| self.allocator.update_counter(&MemoryScope::Local, k, *v));
            }
        } else {
            panic!("Environment {} does not exist!", id);
        }
    }

    /// Creates a new environment to keep a function's local variables.
    ///
    /// # Panics
    ///
    /// Panics if an environment with the same name (function id) has
    /// been declared before.
    pub fn from_function(&mut self, func: &FunctionSignature) {
        let id = &func.id;
        if let Some(_) = self.entries.get(id) {
            panic!("Environment {} already exist!", id);
        }

        self.current_env = id.clone();

        self.current_scope = MemoryScope::Local;
        self.allocator.reset_locals();

        self.entries.insert(id.clone(), EnvEntry::from_func(func));

        for FunctionParam(variable) in func.params.iter() {
            self.add_var(
                &variable.id,
                &variable.data_type,
                &variable.dimension,
                false,
            );
        }
    }

    /// Separates the space for an array of a given dimension
    pub fn allocate_array(&mut self, data_type: &DataType, dimension: &Dimension) -> MemAddress {
        let Dimension {
            dimensions: _,
            shape: _,
            size,
            acc_size: _,
        } = dimension;

        let address: u16;

        address = self
            .allocator
            .assign_location(&self.current_scope, data_type, *size + 1); // Adds an extra space to mark end of array

        address
    }

    /// Adds a new variable to the current declaration environment.
    pub fn add_var(
        &mut self,
        id: &String,
        data_type: &DataType,
        dimension: &Dimension,
        immutable: bool,
    ) -> MemAddress {
        let Dimension {
            dimensions: dim,
            shape: _,
            size: _,
            acc_size: _,
        } = dimension;

        let address: u16;

        if *dim > 0 {
            address = self
                .allocator
                .assign_location(&self.current_scope, &DataType::Pointer, 1);

            let array_address = self.allocate_array(data_type, dimension);

            self.current_env_mut().add(SymbolEntry::new_vec(
                id.clone(),
                data_type.clone(),
                address,
                dimension.clone(),
                array_address,
            ));
        } else {
            address = self
                .allocator
                .assign_location(&self.current_scope, data_type, 1);

            self.current_env_mut().add(SymbolEntry::new_var(
                id.clone(),
                data_type.clone(),
                address,
                dimension.clone(),
                immutable,
            ));
        }

        address
    }

    /// Deletes a variable to the current declaration environment.
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

    /// Adds the parameters metadata from a function signature to an environment.
    pub fn from_func(func: &FunctionSignature) -> EnvEntry {
        EnvEntry {
            is_global: false,
            env_id: func.id.clone(),
            return_type: Some(func.data_type.clone()),
            symbols: HashMap::new(),
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
    pub fn new_var(
        id: String,
        data_type: DataType,
        address: MemAddress,
        dimension: Dimension,
        immutable: bool,
    ) -> SymbolEntry {
        SymbolEntry {
            id,
            data_type,
            dimension,
            address,
            point_address: None,
            immutable,
        }
    }

    pub fn new_vec(
        id: String,
        data_type: DataType,
        address: MemAddress,
        dimension: Dimension,
        point_address: MemAddress,
    ) -> SymbolEntry {
        SymbolEntry {
            id,
            data_type,
            dimension,
            address,
            point_address: Some(point_address),
            immutable: false,
        }
    }
}
