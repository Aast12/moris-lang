use memory::types::{IntType, FloatType};

use super::memory_manager::Item;

pub fn unwrap_int_param(params: &Vec<Item>, index: usize) -> IntType {
    let item = params.get(index).unwrap().to_owned();
    item.unwrap_int()
}

pub fn unwrap_float_param(params: &Vec<Item>, index: usize) -> FloatType {
    let item = params.get(index).unwrap().to_owned();
    item.unwrap_float()
}

pub fn unwrap_str_param(params: &Vec<Item>, index: usize) -> String {
    let item = params.get(index).unwrap().to_owned();
    item.unwrap_string()
}
