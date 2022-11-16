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
    ($typ:tt, $left:expr, $right:expr) => {{
        if let Item::$typ(op1) = $left {
            if let Item::$typ(op2) = $right {
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
    ($data_type:expr, $self: expr, $op: tt, $left: expr, $right: expr, $dest: expr) => {
        match $data_type {
            DataType::Int => {
                let (op1, op2) = Self::match_ints($left, $right);
                $self.update($dest, Item::Int(op1 $op op2));
            }
            DataType::Float => {
                let (op1, op2) = Self::match_floats($left, $right);
                $self.update($dest, Item::Float(op1 $op op2));
            }
            DataType::Pointer => {
                let (op1, op2) = Self::match_pointers($left, $right);
                $self.update($dest, Item::Pointer(op1 $op op2));
            }
            DataType::String => panic!(),
            _ => todo!(),
        }
    };
}

macro_rules! logic_cmp {
    ($data_type:expr, $self: expr, $op: tt, $curr_instruction: expr) => {
        match $data_type {
            DataType::Int => {
                let (left, right, dest) = $self.unpack_binary($curr_instruction);
                let (left, right) = Self::match_ints(left, right);

                $self.update(dest, Item::Bool(left $op right));
            }
            DataType::Float => {
                let (left, right, dest) = $self.unpack_binary($curr_instruction);
                let (left, right) = Self::match_floats(left, right);
                $self.update(dest, Item::Bool(left $op right));
            }
            DataType::Bool => todo!(),
            DataType::String => todo!(),
            DataType::Series => todo!(),
            _ => panic!(),
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

    fn unpack_unary(&self, instruction: &Quadruple) -> (Item, MemAddress) {
        let Quadruple(_, op, _, dest) = instruction;
        let op = self.get(&op);
        let dest = self.get_address(&dest);

        (op, dest)
    }

    fn unpack_binary(&self, instruction: &Quadruple) -> (Item, Item, MemAddress) {
        let Quadruple(_, left, right, dest) = instruction;
        let left = self.get(&left);
        let right = self.get(&right);
        let dest = self.get_address(&dest);

        (left, right, dest)
    }

    fn operation(&mut self, quadruple: Quadruple) {
        let Quadruple(instruction, left, right, dest) = quadruple;
        let instruction = instruction.as_str();
        let left = self.get(&left);
        let right = self.get(&right);
        let dest = self.get_address(&dest);

        let (_, data_type, _) = MemoryResolver::get_offset(dest);

        match instruction {
            "+" => arith_operation!(data_type, self, +, left, right, dest),
            "-" => arith_operation!(data_type, self, -, left, right, dest),
            "*" => arith_operation!(data_type, self, *, left, right, dest),
            "/" => arith_operation!(data_type, self, /, left, right, dest),
            _ => todo!(),
        }
    }

    fn logic_cmp(&mut self, quadruple: Quadruple) {
        let operator = quadruple.0.as_str();
        let left_addr = self.get_address(&quadruple.1);
        let (_, data_type, _) = MemoryResolver::get_offset(left_addr);

        match operator {
            ">" => logic_cmp!(data_type, self, >, &quadruple),
            ">=" => logic_cmp!(data_type, self, >=, &quadruple),
            "<" => logic_cmp!(data_type, self, <, &quadruple),
            "<=" => logic_cmp!(data_type, self, <=, &quadruple),
            "==" => logic_cmp!(data_type, self, ==, &quadruple),
            "!=" => logic_cmp!(data_type, self, !=, &quadruple),
            _ => todo!(),
        }
    }

    pub fn execute(&mut self) {
        let mut instruction_pointer = 0;
        let quadruples: Vec<Quadruple> = self.data.quadruples.drain(..).collect();

        while instruction_pointer < quadruples.len() {
            let curr_instruction = quadruples.get(instruction_pointer).unwrap();
            println!("Evaluating {:#?}", curr_instruction);
            match curr_instruction.0.as_str() {
                "*" | "+" | "-" | "/" => self.operation(curr_instruction.clone()),
                ">" | ">=" | "<" | "<=" | "==" | "!=" => self.logic_cmp(curr_instruction.clone()),
                "=" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);
                    self.update(dest, op);
                }
                "Float" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);

                    let op = match op {
                        Item::Bool(op) => Item::Float((op as u8) as f32),
                        _ => op,
                    };
                    let op = cast!(op, [Int, Float, Pointer], f32);

                    self.update(dest, Item::Float(op));
                }
                "Int" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);

                    let op = cast!(op, [Int, Float, Pointer, Bool], i32);

                    self.update(dest, Item::Int(op));
                }
                "Bool" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);

                    let op = cast!(op, [Int, Float, Pointer, Bool], i32);
                    let op = if op > 0 { true } else { false };

                    self.update(dest, Item::Bool(op));
                }
                "ver" => {
                    let Quadruple(_, value, _, bound) = curr_instruction;
                    let value = self.get(&value);
                    let bound = self.get(&bound);
                    let (value, bound) = Self::match_ints(value, bound);

                    if value >= bound {
                        panic!("Index out of bounds!");
                    }
                }
                "goto" => {
                    let Quadruple(_, _, _, next) = curr_instruction;
                    instruction_pointer = next.parse::<usize>().unwrap();
                    continue;
                }
                "gotoFalse" => {
                    let Quadruple(_, check, _, next) = curr_instruction;
                    let check = self.get(check);
                    match check {
                        Item::Bool(check) => {
                            if !check {
                                instruction_pointer = next.parse::<usize>().unwrap();
                                continue;
                            }
                        }
                        _ => panic!("Can't check non-boolean condition."),
                    };
                }
                _ => panic!(),
            }

            instruction_pointer += 1;
        }
    }
}
