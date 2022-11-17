use std::{collections::HashMap, fs, path::Path};

use crate::codegen::manager::GlobalManager;
use crate::parser::grammar::PProgramParser as Parser;

use crate::{ast::statements::Program, env::Environment, memory::resolver::MemAddress};

use crate::ast::node::Node;

use super::memory_manager::Item;
use super::virtual_machine::VirtualMachine;

type TargetMeta = HashMap<String, MemAddress>;

fn try_file(path: &str) -> Program {
    match fs::read_to_string(path) {
        Ok(file_content) => Parser::new().parse(file_content.as_str()).unwrap(),
        Err(error) => panic!("path: {} -> {}", path, error),
    }
}

pub struct Inspector {
    target_meta: HashMap<String, MemAddress>,
    memory: HashMap<MemAddress, Item>,
}

impl Inspector {
    pub fn new(path: &str) -> Inspector {
        let path_buf = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        let path = path_buf.to_str().unwrap();

        let mut test_program = try_file(path);
        test_program.generate();

        let target_meta: TargetMeta;
        if let Some(global_env) = GlobalManager::get().env.entries.get("global") {
            target_meta = global_env
                .symbols
                .iter()
                .map(|(id, entry)| (id.clone(), entry.address))
                .collect::<TargetMeta>();
        } else {
            panic!("Can't parse global environment!")
        }

        GlobalManager::get().dump();
        let mut vm = VirtualMachine::load("program.o");
        vm.execute();

        Inspector {
            target_meta,
            memory: vm.memory.globals,
        }
    }

    pub fn validate(&self, id: &str, value: Item) -> bool {
        let item_address = self.target_meta.get(id).unwrap();
        let item = self.memory.get(item_address).unwrap();

        *item == value
    }

    pub fn get(&self, id: &str) -> Item {
        let item_address = self.target_meta.get(id).unwrap();
        self.memory.get(item_address).unwrap().clone()
    }
}
