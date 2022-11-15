use core::panic;
use std::{
    collections::{HashMap, LinkedList},
    fs::File,
};

use crate::{
    codegen::{meta::ProgramMeta, quadruples::Quadruple},
    memory::{
        resolver::{MemAddress, MemoryResolver, MemoryScope},
        types::DataType,
    },
};

#[derive(Debug, Clone)]
pub enum Item {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    // DataFrame(),
    // Series,
    Pointer(MemAddress),
}

macro_rules! match_types {
    ($typ:tt, $op1:expr, $op2:expr) => {{
        if let Item::$typ(op1) = $op1 {
            if let Item::$typ(op2) = $op2 {
                return (op1, op2);
            }
        }
        panic!();
    }};
}

macro_rules! cast {
    ($op:expr, [ $($x:tt),*], $y:ty  ) =>
       {
        match $op {
            $(
                Item::$x(op) => op as $y,
            )*
            _ => panic!()
        }
       }
    ;
}

macro_rules! arith_operation {
    ($data_type:expr, $self: expr, $op: tt, $op1: expr, $op2: expr, $dest: expr) => {
        match $data_type {
            DataType::Int => {
                let (op1, op2) = Self::match_ints($op1, $op2);
                $self.update($dest, Item::Int(op1 $op op2));
            }
            DataType::Float => {
                let (op1, op2) = Self::match_floats($op1, $op2);
                $self.update($dest, Item::Float(op1 $op op2));
            }
            DataType::Pointer => {
                let (op1, op2) = Self::match_pointers($op1, $op2);
                $self.update($dest, Item::Pointer(op1 $op op2));
            }
            DataType::String => panic!(),
            _ => todo!(),
        }
    };
}

pub struct VirtualMachine {
    pub data: ProgramMeta,
    pub globals: HashMap<MemAddress, Item>,
    pub locals: LinkedList<HashMap<MemAddress, Item>>,
}

impl VirtualMachine {
    pub fn load(path: &str) -> VirtualMachine {
        let reader = File::open(path).unwrap();
        let data: ProgramMeta = serde_pickle::from_reader(reader, Default::default()).unwrap();
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

        VirtualMachine {
            data,
            globals: HashMap::from(constants),
            locals: LinkedList::new(),
        }
    }

    fn get_address(&self, address: &String) -> MemAddress {
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

    fn update(&mut self, address: MemAddress, item: Item) {
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

    fn get(&self, address: &String) -> Item {
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

    fn match_ints(op1: Item, op2: Item) -> (i32, i32) {
        match_types!(Int, op1, op2);
    }

    fn match_floats(op1: Item, op2: Item) -> (f32, f32) {
        match_types!(Float, op1, op2);
    }

    fn match_strings(op1: Item, op2: Item) -> (String, String) {
        match_types!(String, op1, op2);
    }

    fn match_pointers(op1: Item, op2: Item) -> (MemAddress, MemAddress) {
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

    fn operation(&mut self, quadruple: Quadruple) {
        let Quadruple(instruction, op1, op2, dest) = quadruple;
        let instruction = instruction.as_str();
        let op1 = self.get(&op1);
        let op2 = self.get(&op2);
        let dest = self.get_address(&dest);

        let (_, data_type, _) = MemoryResolver::get_offset(dest);

        match instruction {
            "+" => arith_operation!(data_type, self, +, op1, op2, dest),
            "-" => arith_operation!(data_type, self, -, op1, op2, dest),
            "*" => arith_operation!(data_type, self, *, op1, op2, dest),
            "/" => arith_operation!(data_type, self, /, op1, op2, dest),
            _ => todo!(),
        }
    }

    pub fn execute(&mut self) {
        self.data
            .quadruples
            .clone()
            .iter()
            .for_each(|quad| match quad.0.as_str() {
                "*" | "+" | "-" | "/" => self.operation(quad.clone()),
                "=" => {
                    let Quadruple(_, op, _, dest) = quad;
                    let op = self.get(&op);
                    let dest = self.get_address(&dest);

                    self.update(dest, op);
                }
                "Float" => {
                    let Quadruple(_, op, _, dest) = quad;
                    let op = self.get(&op);
                    let dest = self.get_address(&dest);

                    let op = match op {
                        Item::Bool(op) => Item::Float((op as u8) as f32),
                        _ => op,
                    };
                    let op = cast!(op, [Int, Float, Pointer], f32);

                    self.update(dest, Item::Float(op));
                }
                "Int" => {
                    let Quadruple(_, op, _, dest) = quad;
                    let op = self.get(&op);
                    let dest = self.get_address(&dest);

                    let op = cast!(op, [Int, Float, Pointer, Bool], i32);

                    self.update(dest, Item::Int(op));
                }
                _ => panic!(),
            })
    }
}
