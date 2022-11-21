use core::panic;
use memory::{
    resolver::{MemAddress, MemoryScope},
    types::DataType,
};
use std::{collections::HashMap, fmt::Debug, fs::File, path::PathBuf};

use crate::{
    env::Environment,
    symbols::{FunctionEntry, ParamAddress},
};
use parser::{
    expressions::constant::Const,
    functions::{FunctionParam, FunctionSignature},
    semantics::ExitStatement,
    types::Variable,
    Dimension,
};

use super::{meta::ProgramMeta, quadruples::Quadruple};

/// Object to manage the code generation.
///
/// Holds an environment object to track all variable and
/// function (environments) declarations and their
/// corresponding memory addresses.
///
/// Serves as interface to emit quadruples from external
/// code generation.
///
#[derive(Debug)]
pub struct Manager {
    pub env: Environment,
    instruction_counter: i32,
    pub quadruples: Vec<Quadruple>,
    pub unresolved: HashMap<ExitStatement, Vec<usize>>,
    constant_table: HashMap<MemAddress, Const>,
    procedure_table: HashMap<String, FunctionEntry>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            instruction_counter: 0,
            quadruples: vec![],
            env: Environment::new(),
            unresolved: HashMap::new(),
            constant_table: HashMap::new(),
            procedure_table: HashMap::new(),
        }
    }

    /// Resets the manager to a clean state.
    pub fn reset(&mut self) {
        self.instruction_counter = 0;
        self.quadruples = vec![];
        self.env = Environment::new();
        self.unresolved = HashMap::new();
        self.constant_table = HashMap::new();
        self.procedure_table = HashMap::new();
    }

    /// Serializes and dumps the generated program data
    /// (quadruples, constant and procedure tables) into an object file.
    pub fn dump(&self, file_path: &PathBuf) {
        let meta = ProgramMeta {
            quadruples: self.quadruples.clone(),
            constant_table: self
                .constant_table
                .iter()
                .map(|(k, v)| (*k, v.value.clone()))
                .collect::<HashMap<MemAddress, String>>(),
            procedure_table: self
                .procedure_table
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<String, FunctionEntry>>(),
        };

        let mut buffer = File::create(file_path.as_os_str()).unwrap();

        serde_pickle::to_writer(&mut buffer, &meta, Default::default()).unwrap();
    }

    pub fn get_env(&self) -> &Environment {
        return &self.env;
    }

    pub fn get_env_mut(&mut self) -> &mut Environment {
        return &mut self.env;
    }

    /// Adds a new function to the procedure table and adds the parameters as
    /// local variables to its corresponding environment (symbol table).
    ///
    /// # Panics
    ///
    /// Panics if a function with the same id as func has been declared before
    pub fn new_func(
        &mut self,
        func: &FunctionSignature,
        location: usize,
        return_address: Option<MemAddress>,
        switch: bool,
    ) {
        if self.procedure_table.contains_key(&func.id)
            || self.get_env_mut().entries.contains_key(&func.id)
        {
            panic!("A symbol with id {} has been already defined", func.id)
        }

        self.get_env_mut().from_function(&func);

        // Assigns a local variable address to each of the function parameters
        let params: Vec<ParamAddress> = func
            .params
            .iter()
            .map(
                |FunctionParam(Variable {
                     id,
                     data_type,
                     dimension: _,
                     value: _,
                 })| {
                    let param_symbol = self.get_env_mut().get_var(id).unwrap();
                    (
                        param_symbol.address,
                        data_type.clone(),
                        param_symbol.point_address,
                    )
                },
            )
            .collect();

        self.procedure_table.insert(
            func.id.clone(),
            FunctionEntry::new(location, return_address, params, func),
        );

        if !switch {
            self.get_env_mut().switch(&String::from("global"));
        }
    }

    /// Updates the quadruple start position for a given function.
    ///
    /// # Panics
    ///
    /// Panics if the provided function id has not been declared before
    /// in the procedure table.
    pub fn update_func_position(&mut self, func_id: &String, position: usize) {
        if let Some(func) = self.procedure_table.get_mut(func_id) {
            func.procedure_address = position;
        } else {
            panic!("Can't find function {func_id}")
        }
    }

    /// Returns the address for the global variable holding a functions
    /// return value.
    ///
    /// The manager assumes every function gets assigned a unique global
    /// address to store a return value once it has been called.
    ///
    /// # Panics
    ///
    /// Panics if the provided function id has not been declared before
    /// in the procedure table.
    pub fn get_func_return(&self, func_id: &String) -> Option<MemAddress> {
        if let Some(func) = self.procedure_table.get(func_id) {
            func.return_address
        } else {
            panic!("No funcion {}", func_id);
        }
    }

    pub fn get_func(&self, func_id: &String) -> &FunctionEntry {
        if let Some(func) = self.procedure_table.get(func_id) {
            func
        } else {
            panic!("No funcion {}", func_id);
        }
    }

    pub fn drop_func(&mut self, func_id: &String) {
        self.get_env_mut().switch(&String::from("global"));
        self.get_env_mut().drop_env(func_id);
    }

    pub fn new_variable(
        &mut self,
        id: &String,
        data_type: &DataType,
        dimension: &Dimension,
        immutable: bool,
    ) {
        self.get_env_mut()
            .add_var(id, data_type, dimension, immutable);
    }

    pub fn remove_variable(&mut self, id: &String) {
        self.get_env_mut().del_var(id);
    }

    pub fn new_global(&mut self, data_type: &DataType) -> MemAddress {
        self.env
            .allocator
            .assign_location(&MemoryScope::Global, data_type, 1)
    }

    pub fn new_temp(&mut self, data_type: &DataType) -> MemAddress {
        self.env
            .allocator
            .assign_location(&self.env.current_scope, data_type, 1)
    }

    pub fn new_constant(&mut self, data_type: &DataType, value: &Const) -> MemAddress {
        let address = self
            .env
            .allocator
            .assign_location(&MemoryScope::Constant, data_type, 1);

        self.constant_table.insert(address, value.clone());

        address
    }

    pub fn emit(&mut self, quadruple: Quadruple) {
        self.quadruples.push(quadruple);
        self.instruction_counter += 1;
    }

    /// Shortcut to emit a type cast quadruple.
    pub fn emit_cast(&mut self, target_dt: &DataType, target: &str) -> String {
        let new = self.new_temp(target_dt).to_string();
        self.emit(Quadruple::type_cast(target_dt, target, new.as_str()));
        new
    }

    pub fn get_next_pos(&self) -> usize {
        return self.quadruples.len();
    }

    pub fn update_instruction(&mut self, id: usize, quad: Quadruple) {
        if let Some(local) = self.quadruples.get_mut(id) {
            *local = quad;
        }
    }

    /// Prepares a context exit statment.
    ///
    /// An exit statement is defined as some statment that breaks out of
    /// some context, such as breaks and continues in loops and returns in functions.
    /// 
    /// This function holds all of these statements so they can be resolved 
    /// further in the code generation once all the information needed to generate
    /// them has been figured out. For instance, setting the end position in a 
    /// break statement once the loop block has generated all its quadruples.
    ///
    pub fn prepare_exit_stmt(&mut self, stmt_type: &ExitStatement) {
        let stmt_position = self.get_next_pos();
        self.emit(Quadruple::new_empty());

        if let Some(context) = self.unresolved.get_mut(&stmt_type) {
            context.push(stmt_position);
        } else {
            self.unresolved.insert(*stmt_type, vec![stmt_position]);
        }
    }

    /// Rsolves a context exit statments.
    ///
    /// Replaces all the quadruples matching an exit statement type with the same
    /// quadruple valuee.
    ///
    pub fn resolve_context(&mut self, stmt_type: &ExitStatement, quadruple: Quadruple) {
        let unresolved = self.unresolved.get(stmt_type).unwrap_or(&vec![]).clone();

        for ref_quadruple in unresolved {
            if let Some(to_update) = self.quadruples.get_mut(ref_quadruple) {
                quadruple.clone_into(to_update);
            }
        }

        self.clean_exit_stmt(stmt_type);
    }

    /// Resets an exit statment state.
    pub fn clean_exit_stmt(&mut self, stmt_type: &ExitStatement) {
        self.unresolved.insert(*stmt_type, vec![]);
    }
}
