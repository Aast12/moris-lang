use std::collections::HashMap;

use super::{
    resolver::{MemAddress, MemoryResolver, MemoryScope},
    types::DataType,
};

pub type MemoryCounter = HashMap<DataType, usize>;

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

    pub fn get_counter(&mut self, scope: &MemoryScope) -> &mut MemoryCounter {
        match scope {
            MemoryScope::Global => &mut self.global_counters,
            MemoryScope::Local => &mut self.local_counters,
            MemoryScope::Constant => &mut self.constant_counters,
        }
    }

    pub fn update_counter(&mut self, scope: &MemoryScope, data_type: &DataType, size: usize) {
        let counter = self.get_counter(scope);

        counter.insert(data_type.clone(), size);
    }

    pub fn increase_counter_by(
        &mut self,
        scope: &MemoryScope,
        data_type: &DataType,
        size: usize,
    ) -> usize {
        let counter = self.get_counter(scope);

        if let Some(last) = counter.get(&data_type) {
            let next_offset = last.clone();
            counter.insert(data_type.clone(), next_offset + size);
            next_offset
        } else {
            counter.insert(data_type.clone(), size);
            0
        }
    }

    pub fn reset_locals(&mut self) {
        self.local_counters = HashMap::new();
    }

    pub fn assign_location(
        &mut self,
        scope: &MemoryScope,
        data_type: &DataType,
        size: usize,
    ) -> MemAddress {
        let start_address = self.increase_counter_by(scope, data_type, size);
        MemoryResolver::to_address(scope, data_type, start_address as MemAddress)
    }
}
