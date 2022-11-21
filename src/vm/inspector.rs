use std::{collections::HashMap, path::Path};

use memory::resolver::MemAddress;

use crate::vm::runner::Runner;

use super::memory_manager::Item;

type TargetMeta = HashMap<String, MemAddress>;

pub struct Inspector {
    pub target_meta: HashMap<String, MemAddress>,
    pub memory: HashMap<MemAddress, Item>,
}

impl Inspector {
    pub fn new(path: &str) -> Inspector {
        let path_buf = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);

        let mut runner = Runner::new(path_buf.to_str().unwrap()).unwrap();

        runner.compile();

        let manager = &runner.manager;

        let target_meta: TargetMeta;
        if let Some(global_env) = manager.env.entries.get("global") {
            target_meta = global_env
                .symbols
                .iter()
                .map(|(id, entry)| (id.clone(), entry.address))
                .collect::<TargetMeta>();
        } else {
            panic!("Can't parse global environment!")
        }

        runner.clean();
        let vm = runner.run();

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
