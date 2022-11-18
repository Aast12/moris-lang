use std::{collections::HashMap, path::Path};

use codegen::manager::GlobalManager;
use parser::try_file;

use memory::resolver::MemAddress;

use codegen::node::Node;

use super::memory_manager::Item;
use super::virtual_machine::VirtualMachine;

type TargetMeta = HashMap<String, MemAddress>;

pub struct Inspector {
    pub target_meta: HashMap<String, MemAddress>,
    pub memory: HashMap<MemAddress, Item>,
}

impl Inspector {
    pub fn new(path: &str) -> Inspector {
        let path_buf = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        let path = path_buf.to_str().unwrap();

        let mut test_program = try_file(path);
        // println!("Program {:#?}", test_program);
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
        GlobalManager::get().reset();
        let mut vm = VirtualMachine::load("program.o");
        // println!("QUADS {:#?}", vm.data.quadruples);
        // println!("MEM {:#?}", vm.memory);
        // println!("PROCS {:#?}", vm.data.procedure_table);
        vm.execute();

        // println!("MEM {:#?}", vm.memory);

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
