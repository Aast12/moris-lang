use core::panic;
use std::{
    borrow::BorrowMut,
    collections::{HashMap, LinkedList},
};

use variantly::Variantly;

use crate::{
    codegen::meta::ProgramMeta,
    memory::{
        resolver::{MemAddress, MemoryResolver, MemoryScope},
        types::{DataType, FloatType, IntType},
    },
};

#[derive(Debug, Clone, Variantly, PartialEq)]
pub enum Item {
    Int(IntType),
    Float(FloatType),
    Bool(bool),
    String(String),
    // DataFrame(),
    // Series,
    Pointer(MemAddress),
}

impl Item {
    pub fn cast_int(item: Item) -> IntType {
        match item {
            Item::Int(item) => item,
            Item::Float(item) => item as IntType,
            Item::Bool(item) => {
                if item {
                    1
                } else {
                    0
                }
            }
            Item::Pointer(item) => item as IntType,
            _ => panic!("Cant cast {:#?} to int", item),
        }
    }
}

// (value address, param address)
type ParamMapping = (MemAddress, MemAddress);

#[derive(Debug)]
pub struct CallHold {
    pub call_params: Vec<ParamMapping>,
    pub procedure_id: String,
}

impl CallHold {
    pub fn new(procedure_id: String) -> CallHold {
        CallHold {
            procedure_id,
            call_params: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct CallContext {
    pub procedure_id: String,
    pub locals: HashMap<MemAddress, Item>,
}

impl CallContext {
    pub fn new(procedure_id: String, locals: HashMap<MemAddress, Item>) -> CallContext {
        CallContext {
            procedure_id,
            locals,
        }
    }
}

#[derive(Debug)]
pub struct MemoryManager {
    pub globals: HashMap<MemAddress, Item>,
    pub call_context: LinkedList<CallContext>,
    pub call_hold: LinkedList<CallHold>,
}

impl MemoryManager {
    pub fn from_data(data: &ProgramMeta) -> MemoryManager {
        let constants = data
            .constant_table
            .iter()
            .map(|(address, value)| {
                let (_, data_type, _) = MemoryResolver::get_offset(*address);

                let val = match data_type {
                    DataType::Int => Item::Int(value.parse::<IntType>().unwrap()),
                    DataType::Float => Item::Float(value.parse::<FloatType>().unwrap()),
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
            call_context: LinkedList::new(),
            call_hold: LinkedList::new(),
        }
    }

    pub fn curr_context(&self) -> &CallContext {
        if let Some(ctx) = self.call_context.back() {
            ctx
        } else {
            panic!("Cannot find current context!");
        }
    }

    pub fn curr_context_mut(&mut self) -> &mut CallContext {
        if let Some(ctx) = self.call_context.back_mut() {
            ctx
        } else {
            panic!("Cannot find current context!");
        }
    }

    pub fn curr_locals(&self) -> &HashMap<MemAddress, Item> {
        &self.curr_context().locals
    }

    pub fn curr_locals_mut(&mut self) -> &mut HashMap<MemAddress, Item> {
        self.call_context.back_mut().unwrap().locals.borrow_mut()
    }

    pub fn push_context(&mut self) {
        let call_params = self.curr_hold().call_params.clone();
        let procedure_id = self.curr_hold().procedure_id.clone();

        let locals: HashMap<MemAddress, Item> = call_params
            .iter()
            .map(|(value_addr, param_addr)| {
                let value = self.resolved_get(*value_addr);
                (param_addr.clone(), value)
            })
            .collect();

        self.call_context
            .push_back(CallContext::new(procedure_id, locals));

        self.pop_hold();
    }

    pub fn pop_context(&mut self) {
        self.call_context.pop_back();
    }

    pub fn curr_hold(&self) -> &CallHold {
        &self.call_hold.back().unwrap()
    }

    pub fn curr_hold_mut(&mut self) -> &mut CallHold {
        self.call_hold.back_mut().unwrap().borrow_mut()
    }

    pub fn push_hold(&mut self, procedure_id: String) {
        self.call_hold.push_back(CallHold::new(procedure_id));
    }

    pub fn delete(&mut self, address: MemAddress) {
        let (scope, _, _) = MemoryResolver::get_offset(address);
        match scope {
            MemoryScope::Global | MemoryScope::Constant => self.globals.remove(&address),
            MemoryScope::Local => self.curr_locals_mut().remove(&address),
        };
    }

    pub fn push_param(&mut self, value_address: MemAddress, param_address: MemAddress) {
        self.curr_hold_mut()
            .call_params
            .push((value_address, param_address));
    }

    pub fn pop_hold(&mut self) {
        self.call_hold.pop_back();
    }

    pub fn get_address(&self, address: &String) -> MemAddress {
        if address.starts_with("*") {
            let address = address[1..].parse::<MemAddress>().unwrap();

            let accesed = self._get(address);
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
            MemoryScope::Global | MemoryScope::Constant => self.globals.insert(address, item),
            MemoryScope::Local => self.curr_locals_mut().insert(address, item),
        };
    }

    fn _get(&self, address: MemAddress) -> &Item {
        if let Some(scope) = MemoryResolver::get_scope_from_address(address) {
            match scope {
                MemoryScope::Global | MemoryScope::Constant => self._get_global(address),
                MemoryScope::Local => self._get_local(address),
            }
        } else {
            panic!("{address} is not a valid address");
        }
    }

    fn _get_global(&self, address: MemAddress) -> &Item {
        if let Some(item) = self.globals.get(&address) {
            item
        } else {
            panic!("Cant find global address {address}");
        }
    }

    fn _get_local(&self, address: MemAddress) -> &Item {
        if let Some(item) = self.curr_locals().get(&address) {
            item
        } else {
            panic!("Cant find local address {address}");
        }
    }

    pub fn resolved_get(&mut self, address: MemAddress) -> Item {
        self._get(address).clone()
    }

    pub fn get(&mut self, address: &String) -> Item {
        if address.starts_with("&") {
            let address = address[1..].parse::<MemAddress>().unwrap();
            Item::Pointer(address)
        } else if address.starts_with("*") {
            let next = &address[1..].to_string();
            let next_address = self.get(next);

            if let Item::Pointer(address) = next_address {
                let q = self._get(address).clone();
                q
            } else {
                panic!("Item is not a pointer");
            }
        } else {
            let address = address.parse::<MemAddress>().unwrap();
            self.resolved_get(address)
        }
    }
}
