use std::collections::HashMap;

use lazy_static::lazy_static;

use super::types::DataType;

lazy_static! {
    pub static ref TYPE_OFFSETS: HashMap<DataType, u16> = {
        HashMap::from([
            (DataType::Bool, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 0),       // 0 - 1,999
            (DataType::Float, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 1),      // 2,000 - 3,999
            (DataType::Int, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 2),        // 4,000 - 5,999
            (DataType::String, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 3),     // 6,000 - 7,999
            (DataType::DataFrame, MemoryResolver::DATA_TYPE_ALLOC_SIZE * 4),  // 8,000 - 9,999
        ])
    };
    pub static ref SCOPE_OFFSETS: HashMap<MemoryScope, u16> = {
        HashMap::from([
            (MemoryScope::Global, MemoryResolver::SEGMENT_SIZE * 1),      // (10,000 - 19,999)
            (MemoryScope::Local, MemoryResolver::SEGMENT_SIZE * 2),       // (20,000 - 29,999)
            (MemoryScope::Constant, MemoryResolver::SEGMENT_SIZE * 3),    // (30,000 - 39,999)
        ])
    };
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum MemoryScope {
    Global,
    Local,
    Constant,
}
pub struct MemoryResolver {}

impl MemoryResolver {
    pub const DATA_TYPE_ALLOC_SIZE: u16 = 2_000;
    pub const SEGMENT_SIZE: u16 = 10_000;
    pub const GLOBAL_OFFSET: u16 = 10_000;
    pub const LOCAL_OFFSET: u16 = 20_000;
    pub const CONSTANT_OFFSET: u16 = 30_000;

    pub fn get_type_by_offset(offset: u16) -> Option<DataType> {
        for (data_type, d_offset) in TYPE_OFFSETS.iter() {
            if offset >= *d_offset && offset < d_offset + Self::DATA_TYPE_ALLOC_SIZE {
                return Some(data_type.clone());
            }
        }
        None
    }

    fn get_scope_offset(scope: &MemoryScope) -> u16 {
        if let Some(scope_offset) = SCOPE_OFFSETS.get(scope) {
            *scope_offset
        } else {
            panic!("Scope not supported in memory: {:#?}", scope)
        }
    }

    fn get_type_offset(scope: &MemoryScope, data_type: &DataType) -> u16 {
        if let Some(type_offset) = TYPE_OFFSETS.get(data_type) {
            Self::get_scope_offset(scope) + type_offset
        } else {
            panic!("Type not supported in memory: {:#?}", data_type)
        }
    }

    fn is_within_scope(scope: &MemoryScope, address: u16) -> bool {
        let lower_bound = Self::get_scope_offset(scope);
        let upper_bound = lower_bound + Self::SEGMENT_SIZE;

        address >= lower_bound && address < upper_bound
    }

    fn is_within(scope: &MemoryScope, data_type: &DataType, address: u16) -> bool {
        let lower_bound = Self::get_type_offset(scope, data_type);
        let upper_bound = lower_bound + Self::DATA_TYPE_ALLOC_SIZE;

        println!("BOUNDS {lower_bound} {upper_bound}");

        address >= lower_bound && address < upper_bound
    }

    pub fn next_address(scope: &MemoryScope, data_type: &DataType, offset: u16) -> u16 {
        let type_offset = Self::get_type_offset(scope, data_type);
        let address = type_offset + offset + 1;
        if Self::is_within(scope, data_type, address) {
            address
        } else {
            panic!("Address is not within corresponding memory bounds")
        }
    }

    pub fn next_global_address(data_type: &DataType, offset: u16) -> u16 {
        Self::next_address(&MemoryScope::Global, data_type, offset)
    }

    pub fn next_local_address(data_type: &DataType, offset: u16) -> u16 {
        Self::next_address(&MemoryScope::Local, data_type, offset)
    }

    pub fn next_constant_address(data_type: &DataType, offset: u16) -> u16 {
        Self::next_address(&MemoryScope::Constant, data_type, offset)
    }

    fn get_scope_by_offset(address: u16) -> Option<MemoryScope> {
        for (scope, offset) in SCOPE_OFFSETS.iter() {
            if address >= *offset && address < *offset + Self::SEGMENT_SIZE {
                return Some(*scope);
            }
        }
        None
    }

    pub fn get_offset(address: u16) -> (MemoryScope, DataType, u16) {
        println!("RESOLVE {address}");
        if let Some(scope) = Self::get_scope_by_offset(address) {
            println!("SCOPE {:#?}", scope);
            for data_type in TYPE_OFFSETS.keys() {
                if Self::is_within(&scope, data_type, address) {
                    println!("TYPE {:#?}", data_type);
                    let full_offset = Self::get_type_offset(&scope, data_type);
                    let item_offset = address % full_offset;

                    return (scope.clone(), data_type.clone(), item_offset);
                }
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
                SCOPE_OFFSETS[&MemoryScope::Constant] + TYPE_OFFSETS[&DataType::String] + 1999,
            ),
            (MemoryScope::Constant, DataType::String, 1999)
        );

        assert_eq!(
            MemoryResolver::get_offset(
                SCOPE_OFFSETS[&MemoryScope::Constant] + TYPE_OFFSETS[&DataType::String] + 2000,
            ),
            (MemoryScope::Constant, DataType::DataFrame, 0)
        );

        assert_eq!(
            MemoryResolver::get_offset(
                SCOPE_OFFSETS[&MemoryScope::Local] + TYPE_OFFSETS[&DataType::DataFrame] + 2005,
            ),
            (MemoryScope::Constant, DataType::Bool, 5)
        );
    }
}
