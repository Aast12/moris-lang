use core::panic;
use std::{collections::LinkedList, fs::File};

use crate::{
    codegen::{function::FunctionEntry, meta::ProgramMeta, quadruples::Quadruple},
    memory::{
        resolver::{MemAddress, MemoryResolver},
        types::{DataType, IntType, FloatType},
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

    fn match_ints(left: Item, right: Item) -> (IntType, IntType) {
        (Item::cast_int(left), Item::cast_int(right))
    }

    fn match_floats(op1: Item, op2: Item) -> (FloatType, FloatType) {
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

    fn unpack_unary(&mut self, instruction: &Quadruple) -> (Item, MemAddress) {
        let Quadruple(_, op, _, dest) = instruction;
        let op = self.memory.get(&op);
        let dest = self.memory.get_address(&dest);

        (op, dest)
    }

    fn unpack_binary<'a>(
        &mut self,
        instruction: &'a Quadruple,
    ) -> (&'a str, Item, Item, MemAddress) {
        let Quadruple(operator, left, right, dest) = instruction;
        let left = self.memory.get(&left);
        let right = self.memory.get(&right);
        let dest = self.memory.get_address(&dest);

        (operator.as_str(), left, right, dest)
    }

    fn arithmetic_op(&mut self, quadruple: &Quadruple) {
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

    fn boolean_op(&mut self, quadruple: &Quadruple) {
        let (op, left, right, dest) = self.unpack_binary(quadruple);

        let left = left.unwrap_bool();
        let right = right.unwrap_bool();

        let result = match op {
            "&&" => operate!(left, &&, right),
            "||" => operate!(left, ||, right),
            _ => todo!(),
        };

        self.memory.update(dest, Item::Bool(result));
    }

    fn logic_cmp(&mut self, quadruple: &Quadruple) {
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
        let mut call_pointer: LinkedList<usize> = LinkedList::new();
        let mut pre_call_stack: LinkedList<String> = LinkedList::new();

        let quadruples: Vec<Quadruple> = self.data.quadruples.drain(..).collect();

        while instruction_pointer < quadruples.len() {
            let curr_instruction = quadruples.get(instruction_pointer).unwrap();
            match curr_instruction.0.as_str() {
                "*" | "+" | "-" | "/" => self.arithmetic_op(curr_instruction),
                ">" | ">=" | "<" | "<=" | "==" | "!=" => self.logic_cmp(curr_instruction),
                "&&" | "||" => self.boolean_op(curr_instruction),
                "not" => {
                    let Quadruple(_, to_negate, _, dest) = curr_instruction;
                    let to_negate = self.memory.get(to_negate);
                    let to_negate = to_negate.unwrap_bool();
                    let dest = self.memory.get_address(dest);

                    self.memory.update(dest, Item::Bool(!to_negate));
                }
                "=" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);
                    self.memory.update(dest, op);
                }
                "Float" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);

                    let op = match op {
                        Item::Bool(op) => Item::Float((op as u8) as FloatType),
                        _ => op,
                    };
                    let op = cast!(op, [Int, Float, Pointer], FloatType);

                    self.memory.update(dest, Item::Float(op));
                }
                "Int" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);

                    let op = cast!(op, [Int, Float, Pointer, Bool], IntType);

                    self.memory.update(dest, Item::Int(op));
                }
                "Bool" => {
                    let (op, dest) = self.unpack_unary(curr_instruction);

                    let op = cast!(op, [Int, Float, Pointer, Bool], IntType);
                    let op = if op > 0 { true } else { false };

                    self.memory.update(dest, Item::Bool(op));
                }
                "ver" => {
                    let Quadruple(_, value, _, bound) = curr_instruction;
                    // let value = self.memory.get(&value);
                    // let bound = self.memory.get(&bound);
                    // let (value, bound) = Self::match_ints(value, bound);
                    let value = self.memory.get(&value).unwrap_int();
                    let bound: IntType = bound.parse().unwrap();

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
                "era" => {
                    let Quadruple(_, _, _, function_id) = curr_instruction;
                    self.memory.push_hold(function_id.clone());
                    pre_call_stack.push_back(function_id.clone());
                }
                "param" => {
                    let Quadruple(_, arg_addr, _, param_index) = curr_instruction;
                    let function_id = pre_call_stack.back().unwrap();
                    let call_context = self.data.get_func(function_id);

                    let value_addr = self.memory.get_address(arg_addr);
                    let (param_addr, _) = call_context
                        .params
                        .get(param_index.parse::<usize>().unwrap())
                        .unwrap();

                    self.memory.push_param(value_addr, param_addr.clone());
                }
                "gosub" => {
                    let Quadruple(_, _, _, function_id) = curr_instruction;

                    call_pointer.push_back(instruction_pointer + 1);

                    let func_meta = self.data.get_func(&function_id);
                    self.memory.push_context();
                    pre_call_stack.pop_back();

                    // Cleanup function return address to catch no-return errors
                    let return_addr = func_meta.return_address.unwrap();
                    self.memory.delete(return_addr);

                    instruction_pointer = func_meta.procedure_address;
                    continue;
                }
                "return" => {
                    let Quadruple(_, _, _, return_value_addr) = curr_instruction;
                    let function_id = &self.memory.curr_context().procedure_id;

                    let func_meta = self.data.get_func(function_id);
                    // TODO: void return
                    let return_addr = func_meta.return_address.unwrap();
                    let value = self.memory.get(return_value_addr);
                    self.memory.update(return_addr, value);
                    self.memory.pop_context();

                    instruction_pointer = call_pointer.pop_back().unwrap();
                    continue;
                }
                "endFunc" => {
                    self.memory.pop_context();
                    instruction_pointer = call_pointer.pop_back().unwrap();
                    continue;
                }
                "endProgram" => {
                    break;
                }
                _ => panic!(),
            }

            instruction_pointer += 1;
        }
    }
}
