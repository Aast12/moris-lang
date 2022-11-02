use std::collections::{HashMap, LinkedList};

use super::{
    resolver::{MemAddress, MemoryResolver, MemoryScope},
    types::DataType,
};

type MemoryCounter = HashMap<DataType, MemAddress>;
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

        if let Some(last) = counter.get_mut(&data_type) {
            let next_offset = last.clone();
            *last = next_offset + 1;
            next_offset
        } else {
            counter.insert(*data_type, 0);
            0
        }
    }

    pub fn assign_location(&mut self, scope: &MemoryScope, data_type: &DataType) -> MemAddress {
        let next_offset = self.increase_counter(scope, data_type);
        MemoryResolver::to_address(scope, data_type, next_offset)
    }
}
