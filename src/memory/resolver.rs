use std::collections::HashMap;

use lazy_static::lazy_static;

use super::types::DataType;

pub type MemAddress = u16;

lazy_static! {
    pub static ref TYPE_OFFSETS: HashMap<DataType, MemAddress> = {
        HashMap::from([
            (DataType::Bool, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 0),       // 0 - 1,999
            (DataType::Float, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 1),      // 2,000 - 3,999
            (DataType::Int, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 2),        // 4,000 - 5,999
            (DataType::String, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 3),     // 6,000 - 7,999
            (DataType::DataFrame, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 4),  // 8,000 - 9,999
            (DataType::Pointer, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 5),    // 10,000 - 12,000
        ])
    };

    pub static ref TYPE_OFFSETS_INV: HashMap<MemAddress, DataType> =
        TYPE_OFFSETS.iter().map(|(data_type, offset)| (*offset, data_type.clone())).collect();

    pub static ref SCOPE_OFFSETS: HashMap<MemoryScope, MemAddress> = {
        HashMap::from([
            (MemoryScope::Global, MemoryResolver::SEGMENT_SIZE * 1),      // (12,000 - 23,999)
            (MemoryScope::Local, MemoryResolver::SEGMENT_SIZE * 2),       // (24,000 - 35,999)
            (MemoryScope::Constant, MemoryResolver::SEGMENT_SIZE * 3),    // (36,000 - 39,999)
        ])
    };

    pub static ref SCOPE_OFFSETS_INV: HashMap<MemAddress, MemoryScope> =
        SCOPE_OFFSETS.iter().map(|(scope, offset)| (*offset, *scope)).collect();
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MemoryScope {
    Global,
    Local,
    Constant,
}
pub struct MemoryResolver {}

impl MemoryResolver {
    pub const DATA_TYPE_ALLOC_SIZE: MemAddress = 2_000;
    pub const SEGMENT_SIZE: MemAddress = Self::DATA_TYPE_ALLOC_SIZE * 6; // 6 - Data types count
    pub const GLOBAL_OFFSET: MemAddress = Self::SEGMENT_SIZE * 1;
    pub const LOCAL_OFFSET: MemAddress = Self::SEGMENT_SIZE * 2;
    pub const CONSTANT_OFFSET: MemAddress = Self::SEGMENT_SIZE * 3;

    pub fn get_scope_from_address(address: MemAddress) -> Option<&'static MemoryScope> {
        let offset = address - (address % Self::SEGMENT_SIZE);
        if let Some(scope) = SCOPE_OFFSETS_INV.get(&offset) {
            Some(scope)
        } else {
            None
        }
    }

    pub fn get_type_from_address(address: MemAddress) -> Option<&'static DataType> {
        let type_offset = address % Self::SEGMENT_SIZE;
        let step_offset = type_offset - (type_offset % Self::DATA_TYPE_ALLOC_SIZE);
        if let Some(dtype) = TYPE_OFFSETS_INV.get(&step_offset) {
            Some(dtype)
        } else {
            None
        }
    }

    fn get_scope_offset(scope: &MemoryScope) -> MemAddress {
        if let Some(scope_offset) = SCOPE_OFFSETS.get(scope) {
            *scope_offset
        } else {
            panic!("Scope not supported in memory: {:#?}", scope)
        }
    }

    fn get_type_offset(scope: &MemoryScope, data_type: &DataType) -> MemAddress {
        if let Some(type_offset) = TYPE_OFFSETS.get(data_type) {
            Self::get_scope_offset(scope) + type_offset
        } else {
            panic!("Type not supported in memory: {:#?}", data_type)
        }
    }

    fn is_within(scope: &MemoryScope, data_type: &DataType, address: MemAddress) -> bool {
        let lower_bound = Self::get_type_offset(scope, data_type);
        let upper_bound = lower_bound + Self::DATA_TYPE_ALLOC_SIZE;

        address >= lower_bound && address < upper_bound
    }

    pub fn to_address(scope: &MemoryScope, data_type: &DataType, offset: MemAddress) -> MemAddress {
        let type_offset = Self::get_type_offset(scope, data_type);
        let address = type_offset + offset;
        if Self::is_within(scope, data_type, address) {
            address
        } else {
            panic!("Address is not within corresponding memory bounds")
        }
    }

    pub fn next_global_address(data_type: &DataType, offset: MemAddress) -> MemAddress {
        Self::to_address(&MemoryScope::Global, data_type, offset)
    }

    pub fn next_local_address(data_type: &DataType, offset: MemAddress) -> MemAddress {
        Self::to_address(&MemoryScope::Local, data_type, offset)
    }

    pub fn next_constant_address(data_type: &DataType, offset: MemAddress) -> MemAddress {
        Self::to_address(&MemoryScope::Constant, data_type, offset)
    }

    pub fn get_offset(address: MemAddress) -> (MemoryScope, DataType, MemAddress) {
        if let Some(scope) = Self::get_scope_from_address(address) {
            if let Some(data_type) = Self::get_type_from_address(address) {
                let item_offset = address % Self::DATA_TYPE_ALLOC_SIZE;

                return (scope.clone(), data_type.clone(), item_offset);
            }
        }

        panic!("Cannot resolve address");
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_offsets() {
        use crate::memory::{
            resolver::{MemoryScope, SCOPE_OFFSETS, TYPE_OFFSETS},
            types::DataType,
        };

        use super::MemoryResolver;

        assert_eq!(
            MemoryResolver::get_offset(
                SCOPE_OFFSETS[&MemoryScope::Global] + TYPE_OFFSETS[&DataType::Bool] + 0,
            ),
            (MemoryScope::Global, DataType::Bool, 0)
        );

        assert_eq!(
            MemoryResolver::get_offset(
                SCOPE_OFFSETS[&MemoryScope::Local] + TYPE_OFFSETS[&DataType::DataFrame] + 7,
            ),
            (MemoryScope::Local, DataType::DataFrame, 7)
        );

        assert_eq!(
            MemoryResolver::get_offset(
                SCOPE_OFFSETS[&MemoryScope::Constant]
                    + TYPE_OFFSETS[&DataType::String]
                    + MemoryResolver::DATA_TYPE_ALLOC_SIZE
                    - 1,
            ),
            (
                MemoryScope::Constant,
                DataType::String,
                MemoryResolver::DATA_TYPE_ALLOC_SIZE - 1
            )
        );

        assert_eq!(
            MemoryResolver::get_offset(
                SCOPE_OFFSETS[&MemoryScope::Constant]
                    + TYPE_OFFSETS[&DataType::String]
                    + MemoryResolver::DATA_TYPE_ALLOC_SIZE,
            ),
            (MemoryScope::Constant, DataType::DataFrame, 0)
        );

        assert_eq!(
            MemoryResolver::get_offset(
                SCOPE_OFFSETS[&MemoryScope::Local]
                    + TYPE_OFFSETS[&DataType::DataFrame]
                    + MemoryResolver::DATA_TYPE_ALLOC_SIZE
                    + 5,
            ),
            (MemoryScope::Local, DataType::Pointer, 5)
        );
    }
}
