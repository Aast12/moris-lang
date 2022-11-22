use core::panic;
use std::{
    borrow::BorrowMut,
    collections::{HashMap, LinkedList},
    fmt::Display,
};

use polars::{prelude::DataFrame, series::Series};
use variantly::Variantly;

use codegen::{meta::ProgramMeta, symbols::FunctionEntry};

use memory::{
    resolver::{MemAddress, MemoryResolver, MemoryScope},
    types::{DataType, FloatType, IntType},
};

macro_rules! match_types {
    ($typ:tt, $left:expr, $right:expr) => {{
        if let Item::$typ(op1) = $left {
            if let Item::$typ(op2) = $right {
                return (op1, op2);
            }
        }
        panic!();
    }};
}

#[derive(Debug, Clone, Variantly, PartialEq)]
pub enum Item {
    Int(IntType),
    Float(FloatType),
    Bool(bool),
    String(String),
    DataFrame(DataFrame),
    Series(Series),
    Pointer(MemAddress),
    ArrayEnd,
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

    pub fn match_ints(left: Item, right: Item) -> (IntType, IntType) {
        (Item::cast_int(left), Item::cast_int(right))
    }

    pub fn match_floats(op1: Item, op2: Item) -> (FloatType, FloatType) {
        match_types!(Float, op1, op2);
    }

    pub fn match_strings(op1: Item, op2: Item) -> (String, String) {
        match_types!(String, op1, op2);
    }

    pub fn match_pointers(op1: Item, op2: Item) -> (MemAddress, MemAddress) {
        match op1 {
            Item::Int(op1) => match op2 {
                Item::Int(op2) => (op1 as MemAddress, op2 as MemAddress),
                Item::Pointer(op2) => (op1 as MemAddress, op2),
                _ => panic!(),
            },
            Item::Pointer(op1) => match op2 {
                Item::Int(op2) => (op1, op2 as MemAddress),
                Item::Pointer(op2) => (op1, op2),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

impl Display for Item {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Int(item) => write!(fmt, "{}", item),
            Item::Float(item) => write!(fmt, "{}", item),
            Item::Bool(item) => write!(fmt, "{}", item),
            Item::String(item) => write!(fmt, "{}", item),
            Item::Pointer(item) => write!(fmt, "prt({})", item),
            Item::ArrayEnd => write!(fmt, "END"),
            Item::DataFrame(df) => write!(fmt, "{:#?}", df),
            Item::Series(item) => write!(fmt, "{:#?}", item),
        }
    }
}

// (value address, param address)
type ParamMapping = MemAddress;

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

    pub fn pop_params_address(&mut self) -> Vec<MemAddress> {
        let call_params = self.curr_hold().call_params.clone();

        let params: Vec<MemAddress> = call_params.iter().map(|value_addr| *value_addr).collect();

        self.pop_hold();

        params
    }

    pub fn pop_params(&mut self) -> Vec<Item> {
        let call_params = self.curr_hold().call_params.clone();

        let params: Vec<Item> = call_params
            .iter()
            .map(|value_addr| {
                let value = self.resolved_get(*value_addr);
                value
            })
            .collect();

        self.pop_hold();

        params
    }

    pub fn push_context(&mut self, context: &FunctionEntry) {
        let call_params = self.curr_hold().call_params.clone();

        let procedure_id = self.curr_hold().procedure_id.clone();

        let call_params = call_params
            .iter()
            .enumerate()
            .map(|(param_index, value_addr)| {
                let (param_addr, _, _) = context.params.get(param_index).unwrap();

                (*value_addr, *param_addr)
            })
            .collect::<Vec<(MemAddress, MemAddress)>>();

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

    pub fn push_param(&mut self, value_address: MemAddress) {
        self.curr_hold_mut().call_params.push(value_address);
    }

    pub fn pop_hold(&mut self) {
        self.call_hold.pop_back();
    }

    pub fn get_address(&self, address: &String) -> MemAddress {
        if address.starts_with("*") {
            let address = address[1..].parse::<MemAddress>().unwrap();

            let accesed = self._get(address).unwrap();
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

    fn _get(&self, address: MemAddress) -> Result<&Item, String> {
        if let Some(scope) = MemoryResolver::get_scope_from_address(address) {
            match scope {
                MemoryScope::Global | MemoryScope::Constant => self._get_global(address),
                MemoryScope::Local => self._get_local(address),
            }
        } else {
            Err(format!("{address} is not a valid address"))
        }
    }

    fn _get_global(&self, address: MemAddress) -> Result<&Item, String> {
        if let Some(item) = self.globals.get(&address) {
            Ok(item)
        } else {
            Err(format!("Cant find global address {address}"))
        }
    }

    fn _get_local(&self, address: MemAddress) -> Result<&Item, String> {
        if let Some(item) = self.curr_locals().get(&address) {
            Ok(item)
        } else {
            Err(format!("Cant find local address {address}"))
        }
    }

    pub fn resolved_get(&mut self, address: MemAddress) -> Item {
        self._get(address).unwrap().clone()
    }

    pub fn safe_resolved_get(&mut self, address: MemAddress) -> Result<Item, String> {
        let res = self._get(address);
        if let Ok(item) = res {
            Ok(item.clone())
        } else {
            Err(res.unwrap_err())
        }
    }

    pub fn safe_get(&mut self, address: &String) -> Result<Item, String> {
        if address.starts_with("&") {
            let address = address[1..].parse::<MemAddress>().unwrap();
            Ok(Item::Pointer(address))
        } else if address.starts_with("*") {
            let next = &address[1..].to_string();
            let attempt = self.safe_get(next);
            if let Ok(next_address) = attempt {
                if let Item::Pointer(address) = next_address {
                    self.safe_get(&format!("{}", address))
                } else {
                    Err(String::from("Item is not a pointer"))
                }
            } else {
                attempt
            }
        } else {
            let parse = address.parse::<MemAddress>();
            if let Ok(address) = parse {
                Ok(self.resolved_get(address))
            } else {
                let err = format!("Item {} is not an address", address).to_owned();
                Err(err.to_owned())
            }
        }
    }

    pub fn get(&mut self, address: &String) -> Item {
        if address == "END" {
            return Item::ArrayEnd;
        } else {
            self.safe_get(address).unwrap()
        }
    }

    pub fn get_array(&mut self, start_address: &MemAddress) -> Vec<Option<Item>> {
        let mut curr_address = *start_address;
        let mut items: Vec<Option<Item>> = Vec::new();
        loop {
            let current = self.safe_resolved_get(curr_address);

            if let Ok(item) = current {
                if item == Item::ArrayEnd {
                    break;
                }
                items.push(Some(item));
            } else {
                items.push(None);
            }

            curr_address += 1;
        }

        items
    }

    pub fn alter_array<F>(&mut self, start_address: &MemAddress, cb: F)
    where
        F: Fn(&mut Self, (MemAddress, Option<Item>)) -> (),
    {
        let mut curr_address = *start_address;
        loop {
            let current = self.safe_resolved_get(curr_address);

            if let Ok(item) = current {
                if item == Item::ArrayEnd {
                    break;
                }
                cb(self, (curr_address, Some(item)));
            } else {
                cb(self, (curr_address, None));
            }

            curr_address += 1;
        }
    }
}
