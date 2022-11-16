use core::panic;
use std::fs::File;

use crate::{
    codegen::{meta::ProgramMeta, quadruples::Quadruple},
    memory::{
        resolver::{MemAddress, MemoryResolver},
        types::DataType,
    },
};

use super::memory_manager::{Item, MemoryManager};

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
                $self.memory.update($dest, Item::Int(op1 $op op2));
            }
            DataType::Float => {
                let (op1, op2) = Self::match_floats($left, $right);
                $self.memory.update($dest, Item::Float(op1 $op op2));
            }
            DataType::Pointer => {
                let (op1, op2) = Self::match_pointers($left, $right);
                $self.memory.update($dest, Item::Pointer(op1 $op op2));
            }
            DataType::String => panic!(),
            _ => todo!(),
        }
    };
}

macro_rules! operate {
    ($left: expr, $op: tt, $right: expr) => {
        $left $op $right
    };
}

macro_rules! logic_cmp {
    ($data_type:expr, $self: expr, $op: tt, $curr_instruction: expr) => {
        match $data_type {
            DataType::Int => {
                let (_, left, right, dest) = $self.unpack_binary($curr_instruction);
                let (left, right) = Self::match_ints(left, right);

                $self.memory.update(dest, Item::Bool(left $op right));
            }
            DataType::Float => {
                let (_, left, right, dest) = $self.unpack_binary($curr_instruction);
                let (left, right) = Self::match_floats(left, right);
                $self.memory.update(dest, Item::Bool(left $op right));
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
    pub memory: MemoryManager,
}

impl VirtualMachine {
    pub fn load(path: &str) -> VirtualMachine {
        let reader = File::open(path).unwrap();
        let data: ProgramMeta = serde_pickle::from_reader(reader, Default::default()).unwrap();
        let memory = MemoryManager::from_data(&data);

        VirtualMachine { data, memory }
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
        let op = self.memory.get(&op);
        let dest = self.memory.get_address(&dest);

        (op, dest)
    }

    fn unpack_binary<'a>(&self, instruction: &'a Quadruple) -> (&'a str, Item, Item, MemAddress) {
        let Quadruple(operator, left, right, dest) = instruction;
        let left = self.memory.get(&left);
        let right = self.memory.get(&right);
        let dest = self.memory.get_address(&dest);

        (operator.as_str(), left, right, dest)
    }

    fn operation(&mut self, quadruple: Quadruple) {
        let Quadruple(instruction, left, right, dest) = quadruple;
        let instruction = instruction.as_str();
        let left = self.memory.get(&left);
        let right = self.memory.get(&right);
        let dest = self.memory.get_address(&dest);

        let data_type = MemoryResolver::get_type_from_address(dest).unwrap();

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
        let left_addr = self.memory.get_address(&quadruple.1);

        // Instructions for comparisons expect both operators to be of the same type
        // Proper casting instruction for compatible types is emitted during compile time
        let data_type = MemoryResolver::get_type_from_address(left_addr).unwrap();

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
            println!("EXEC {:#?}", curr_instruction.0.as_str());
            match curr_instruction.0.as_str() {
                "*" | "+" | "-" | "/" => self.operation(curr_instruction.clone()),
                ">" | ">=" | "<" | "<=" | "==" | "!=" => self.logic_cmp(curr_instruction.clone()),
                "&&" | "||" => {
                    let (op, left, right, dest) = self.unpack_binary(curr_instruction);

                    let left = left.unwrap_bool();
                    let right = right.unwrap_bool();

                    let result = match op {
                        "&&" => operate!(left, &&, right),
                        "||" => operate!(left, ||, right),
                        _ => todo!(),
                    };

                    self.memory.update(dest, Item::Bool(result));
                }
                "=" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);
                    self.memory.update(dest, op);
                }
                "Float" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);

                    let op = match op {
                        Item::Bool(op) => Item::Float((op as u8) as f32),
                        _ => op,
                    };
                    let op = cast!(op, [Int, Float, Pointer], f32);

                    self.memory.update(dest, Item::Float(op));
                }
                "Int" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);

                    let op = cast!(op, [Int, Float, Pointer, Bool], i32);

                    self.memory.update(dest, Item::Int(op));
                }
                "Bool" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);

                    let op = cast!(op, [Int, Float, Pointer, Bool], i32);
                    let op = if op > 0 { true } else { false };

                    self.memory.update(dest, Item::Bool(op));
                }
                "ver" => {
                    let Quadruple(_, value, _, bound) = curr_instruction;
                    let value = self.memory.get(&value);
                    let bound = self.memory.get(&bound);
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
                    let check = self.memory.get(check);
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
