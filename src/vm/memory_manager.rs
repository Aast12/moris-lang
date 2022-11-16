use core::panic;
use std::collections::{HashMap, LinkedList};

use variantly::Variantly;

use crate::{
    codegen::meta::ProgramMeta,
    memory::{
        resolver::{MemAddress, MemoryResolver, MemoryScope},
        types::DataType,
    },
};

#[derive(Debug, Clone, Variantly)]
pub enum Item {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    // DataFrame(),
    // Series,
    Pointer(MemAddress),
}

#[derive(Debug)]
pub struct MemoryManager {
    pub globals: HashMap<MemAddress, Item>,
    pub locals: LinkedList<HashMap<MemAddress, Item>>,
}

impl MemoryManager {
    pub fn from_data(data: &ProgramMeta) -> MemoryManager {
        let constants = data
            .constant_table
            .iter()
            .map(|(address, value)| {
                let (_, data_type, _) = MemoryResolver::get_offset(*address);

                let val = match data_type {
                    DataType::Int => Item::Int(value.parse::<i32>().unwrap()),
                    DataType::Float => Item::Float(value.parse::<f32>().unwrap()),
                    DataType::Bool => Item::Bool(value.parse::<bool>().unwrap()),
                    DataType::String => Item::String(value.clone()),
                    DataType::Pointer => Item::Pointer(value.parse::<MemAddress>().unwrap()),
                    _ => todo!(),
                };
                (*address, val)
            })
            .collect::<HashMap<MemAddress, Item>>();

        MemoryManager {
            globals: HashMap::from(constants),
            locals: LinkedList::new(),
        }
    }

    pub fn get_address(&self, address: &String) -> MemAddress {
        if address.starts_with("*") {
            let address = address[1..].parse::<MemAddress>().unwrap();
            let accesed = self.globals.get(&address).unwrap();

            match accesed {
                Item::Pointer(addr) => *addr as MemAddress,
                _ => panic!("Element is not a pointer"),
            }
        } else {
            let address = address.parse::<MemAddress>().unwrap();
            address
        }
    }

    pub fn update(&mut self, address: MemAddress, item: Item) {
        let (scope, _, _) = MemoryResolver::get_offset(address);
        match scope {
            MemoryScope::Global | MemoryScope::Constant => {
                self.globals.insert(address, item);
            }
            MemoryScope::Local => {
                if let Some(curr_context) = self.locals.back_mut() {
                    curr_context.insert(address, item);
                } else {
                    panic!("No current local context");
                }
            }
        }
    }

    pub fn get(&self, address: &String) -> Item {
        if address.starts_with("&") {
            let address = address[1..].parse::<MemAddress>().unwrap();
            Item::Pointer(address)
        } else {
            let address = address.parse::<MemAddress>().unwrap();
            let (scope, _, _) = MemoryResolver::get_offset(address);
            match scope {
                MemoryScope::Global | MemoryScope::Constant => {
                    self.globals.get(&address).unwrap().clone() // TODO: Remove constants if not reused
                }
                MemoryScope::Local => {
                    if let Some(curr) = self.locals.back() {
                        curr.get(&address).unwrap().clone()
                    } else {
                        panic!("No current local context");
                    }
                }
            }
        }
    }
}
