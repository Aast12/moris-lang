use std::collections::{HashMap, LinkedList};

use super::{
    resolver::{MemAddress, MemoryResolver, MemoryScope},
    types::DataType,
};

type MemoryCounter = HashMap<DataType, MemAddress>;

#[derive(Debug)]
pub struct VirtualAllocator {
    global_counters: MemoryCounter,
    local_counters: MemoryCounter,
    constant_counters: MemoryCounter,
}

impl VirtualAllocator {
    pub fn new() -> VirtualAllocator {
        VirtualAllocator {
            global_counters: HashMap::new(),
            local_counters: HashMap::new(),
            constant_counters: HashMap::new(),
        }
    }

    fn increase_counter(&mut self, scope: &MemoryScope, data_type: &DataType) -> MemAddress {
        let counter = match scope {
            MemoryScope::Global => &mut self.global_counters,
            MemoryScope::Local => &mut self.local_counters,
            MemoryScope::Constant => &mut self.constant_counters,
        };

        if let Some(last) = counter.get(&data_type) {
            let next_offset = last.clone();
            println!("CURRENT COUNTER {:#?}, {:#?}: {}", scope, data_type, next_offset);
            counter.insert(data_type.clone(), next_offset + 1);
            next_offset
        } else {
            counter.insert(data_type.clone(), 1);
            println!("STARTING COUNTER {:#?}, {:#?}: {}", scope, data_type, 1);
            0
        }
    }

    pub fn reset_locals(&mut self) {
        println!("RESET???");
        self.local_counters = HashMap::new();
    }

    pub fn assign_location(&mut self, scope: &MemoryScope, data_type: &DataType) -> MemAddress {
        println!("------------- CURRENT STATE -------------\n");
        println!("GLOBALS:");
        for (dt, mem) in self.global_counters.iter() {
            println!("({:#?}, {})", dt, mem);
        }
        println!("\nLOCALS:");
        for (dt, mem) in self.local_counters.iter() {
            println!("({:#?}, {})", dt, mem);
        }
        println!("\nCONSTANTS:");
        for (dt, mem) in self.constant_counters.iter() {
            println!("({:#?}, {})", dt, mem);
        }
        println!("------------- ------------- -------------\n");
        let next_offset = self.increase_counter(scope, data_type);
        MemoryResolver::to_address(scope, data_type, next_offset)
    }
}
