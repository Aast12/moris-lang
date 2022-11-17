use core::panic;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    sync::{Mutex, MutexGuard},
};

use crate::{
    ast::{
        expressions::constant::Const,
        functions::{Function, FunctionParam},
        types::Variable,
        Dimension,
    },
    codegen::function::{FunctionEntry, ParamAddress},
    env::Environment,
    memory::{
        resolver::{MemAddress, MemoryScope},
        types::DataType,
    },
    semantics::ExitStatement,
};

use super::{meta::ProgramMeta, quadruples::Quadruple};

lazy_static! {
    pub static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::new());
}

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

    pub fn dump(&self) {
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

        let v = serde_pickle::to_vec(&meta, Default::default()).unwrap();

        let mut buffer = File::create("program.o").unwrap();

        serde_pickle::to_writer(&mut buffer, &meta, Default::default()).unwrap();
    }

    pub fn get_env(&self) -> &Environment {
        return &self.env;
    }

    pub fn get_env_mut(&mut self) -> &mut Environment {
        return &mut self.env;
    }

    pub fn new_func(
        &mut self,
        func: &Function,
        location: usize,
        return_address: Option<MemAddress>,
    ) {
        if self.procedure_table.contains_key(&func.signature.id)
            || self.get_env_mut().entries.contains_key(&func.signature.id)
        {
            panic!(
                "A symbol with id {} has been already defined",
                func.signature.id
            )
        }

        self.get_env_mut().from_function(&func.signature, true);

        let params: Vec<ParamAddress> = func
            .signature
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
                    (param_symbol.address, data_type.clone())
                },
            )
            .collect();

        self.procedure_table.insert(
            func.signature.id.clone(),
            FunctionEntry::new(location, return_address, params, func),
        );
    }

    pub fn update_func_position(&mut self, func_id: &String, position: usize) {
        if let Some(func) = self.procedure_table.get_mut(func_id) {
            func.procedure_address = position;
        } else {
            panic!("Can't find function {func_id}")
        }
    }

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

    pub fn new_variable(&mut self, id: &String, data_type: &DataType, dimension: &Dimension) {
        self.get_env_mut().add_var(id, data_type, dimension);
    }

    pub fn remove_variable(&mut self, id: &String) {
        self.get_env_mut().del_var(id);
    }

    pub fn new_global(&mut self, data_type: &DataType) -> MemAddress {
        self.env
            .allocator
            .assign_location(&MemoryScope::Global, data_type, 1)
    }

    pub fn new_temp_address(&mut self, data_type: &DataType) -> MemAddress {
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

    pub fn emit_cast(&mut self, target_dt: &DataType, target: &str) -> String {
        let new = self.new_temp_address(target_dt).to_string();
        self.emit(Quadruple::type_cast(target_dt, target, new.as_str()));
        new
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

    pub fn new_temp(data_type: &DataType) -> MemAddress {
        Self::get().new_temp_address(data_type)
    }

    pub fn new_constant(data_type: &DataType, value: &Const) -> MemAddress {
        Self::get().new_constant(data_type, value)
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

    pub fn emit_cast(target_dt: &DataType, target: &str) -> String {
        GlobalManager::get().emit_cast(target_dt, target)
    }

    pub fn get_next_pos() -> usize {
        Self::get().get_next_id()
    }
}
