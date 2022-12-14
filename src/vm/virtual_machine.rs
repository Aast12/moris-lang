use core::panic;
use std::{cmp::Ordering, collections::LinkedList, fs::File, str::FromStr};

use codegen::{meta::ProgramMeta, natives::NativeFunction, quadruples::Quadruple};

use memory::{
    resolver::{MemAddress, MemoryResolver},
    types::{DataType, FloatType, IntType},
};

use crate::plots::context::PlotContext;

use super::{
    memory_manager::{Item, MemoryManager},
    natives::run_native,
};

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
                let (op1, op2) = Item::match_ints($left, $right);
                $self.memory.update($dest, Item::Int(op1 $op op2));
            }
            DataType::Float => {
                let (op1, op2) = Item::match_floats($left, $right);
                $self.memory.update($dest, Item::Float(op1 $op op2));
            }
            DataType::Pointer => {
                let (op1, op2) = Item::match_pointers($left, $right);
                $self.memory.update($dest, Item::Pointer(op1 $op op2));
            }
            DataType::String => {
                let (left, right) = Item::match_strings($left, $right);
                $self.memory.update($dest, Item::String(format!("{}{}", left, right)));
            },
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
                let (left, right) = Item::match_ints(left, right);

                $self.memory.update(dest, Item::Bool(left $op right));
            }
            DataType::Float => {
                let (_, left, right, dest) = $self.unpack_binary($curr_instruction);
                let (left, right) = Item::match_floats(left, right);
                $self.memory.update(dest, Item::Bool(left $op right));
            }
            DataType::Bool => todo!(),
            DataType::String => panic!(),
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
        let (instruction, left, right, dest) = self.unpack_binary(quadruple);

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

        match data_type {
            DataType::String => {
                let (_, left, right, dest) = self.unpack_binary(quadruple);
                let (left, right) = Item::match_strings(left, right);
                let cmp = left.cmp(&right);
                let result: bool = match operator {
                    ">" => cmp == Ordering::Greater,
                    ">=" => cmp == Ordering::Greater || cmp == Ordering::Equal,
                    "<" => cmp == Ordering::Less,
                    "<=" => cmp == Ordering::Less || cmp == Ordering::Equal,
                    "==" => cmp == Ordering::Equal,
                    "!=" => cmp != Ordering::Equal,
                    _ => panic!(),
                };

                self.memory.update(dest, Item::Bool(result));
            }
            _ => match operator {
                ">" => logic_cmp!(data_type, self, >, &quadruple),
                ">=" => logic_cmp!(data_type, self, >=, &quadruple),
                "<" => logic_cmp!(data_type, self, <, &quadruple),
                "<=" => logic_cmp!(data_type, self, <=, &quadruple),
                "==" => logic_cmp!(data_type, self, ==, &quadruple),
                "!=" => logic_cmp!(data_type, self, !=, &quadruple),
                _ => todo!(),
            },
        }
    }

    pub fn return_value(&mut self, function_id: &String, value: Item) {
        let func_meta = self.data.get_func(&function_id);
        let return_addr = func_meta.return_address.unwrap();
        self.memory.update(return_addr, value);
    }

    pub fn return_value_native(&mut self, function_id: NativeFunction, value: Item) {
        self.return_value(&function_id.to_string(), value)
    }

    pub fn execute(&mut self) {
        let mut instruction_pointer = 0;
        let mut call_pointer: LinkedList<usize> = LinkedList::new();
        let mut pre_call_stack: LinkedList<String> = LinkedList::new();
        let mut plot_ctx = PlotContext::new();
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
                "neg" => {
                    let Quadruple(_, to_negate, _, dest) = curr_instruction;
                    let to_negate = self.memory.get(to_negate);
                    let dest = self.memory.get_address(dest);

                    match to_negate {
                        Item::Int(item) => self.memory.update(dest, Item::Int(-item)),
                        Item::Float(item) => self.memory.update(dest, Item::Float(-item)),
                        _ => panic!("{:#?} can't be negated", to_negate),
                    }
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
                    let Quadruple(_, arg_addr, _, _) = curr_instruction;

                    let value_addr = self.memory.get_address(arg_addr);

                    self.memory.push_param(value_addr);
                }
                "gosub" => {
                    let Quadruple(_, _, _, function_id) = curr_instruction;

                    if let Ok(native_func) = NativeFunction::from_str(function_id) {
                        let return_value = run_native(&mut plot_ctx, native_func, &mut self.memory);
                        if let Some((func, value)) = return_value {
                            self.return_value_native(func, value);
                        }
                        pre_call_stack.pop_back();
                    } else {
                        call_pointer.push_back(instruction_pointer + 1);

                        let func_meta = self.data.get_func(&function_id);
                        self.memory.push_context(func_meta);
                        pre_call_stack.pop_back();

                        // Cleanup function return address to catch no-return errors
                        if let Some(return_addres) = func_meta.return_address {
                            self.memory.delete(return_addres);
                        }

                        instruction_pointer = func_meta.procedure_address;
                        continue;
                    }
                }
                "return" | "voidReturn" => {
                    let Quadruple(_, _, _, return_value_addr) = curr_instruction;
                    let function_id = &self.memory.curr_context().procedure_id;

                    let func_meta = self.data.get_func(function_id);
                    if let None = func_meta.return_address {
                        if !return_value_addr.is_empty() {
                            panic!("Can't return value for void function");
                        }
                    } else {
                        let return_addr = func_meta.return_address.unwrap();
                        let value = self.memory.get(return_value_addr);
                        self.memory.update(return_addr, value);
                    }

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
                "print" => {
                    let Quadruple(_, _, _, print_target) = curr_instruction;

                    if let Ok(print_target) = self.memory.safe_get(print_target) {
                        print!("{} ", print_target);
                    } else {
                        print!("{}", print_target);
                    }
                }
                "free" => {
                    let Quadruple(_, _, _, address) = curr_instruction;

                    let del_address = self.memory.get_address(address);
                    self.memory.delete(del_address);
                }
                _ => panic!(),
            }

            instruction_pointer += 1;
        }
    }
}
