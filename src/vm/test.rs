use std::{collections::HashMap, path::Path};

use codegen::generate;
use codegen::manager::{Manager};

use memory::resolver::MemAddress;

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
        let mut manager = Manager::new();
        generate(&path_buf, &mut manager);

        let target_meta: TargetMeta;
        if let Some(global_env) = manager.env.entries.get("global") {
            target_meta = global_env
                .3
                .iter()
                .map(|(id, entry)| (id.clone(), entry.address))
                .collect::<TargetMeta>();
        } else {
            panic!("Can't parse global environment!")
        }

        manager.dump();
        manager.reset();

        let mut vm = VirtualMachine::load("program.o");

        dbg!(&vm.data.quadruples);

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

    pub fn get_address(&self, id: &str) -> u16 {
        *self.target_meta.get(id).unwrap()
    }

    pub fn get(&self, id: &str) -> Item {
        let item_address = self.target_meta.get(id).unwrap();
        self.memory.get(item_address).unwrap().clone()
    }

    pub fn debug(&self) {
        self.target_meta.iter().for_each(|(key, value)| {
            let item = self.memory.get(value);

            if let Some(item) = item {
                println!("({}[{}], {:#?})", key, value, item);
            } else {
                println!("({}[{}], UNDEFINED)", key, value);

            }
        });
    }
}
