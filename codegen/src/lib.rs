use manager::Manager;
use memory::types::DataType;
use natives::NativeFunctions;
use node::Node;
use parser::try_file;
use std::path::PathBuf;

pub mod ast_nodes;
pub mod env;
pub mod manager;
pub mod meta;
pub mod natives;
pub mod quadruples;
pub mod symbols;
pub mod node;

pub fn generate(path: &PathBuf, manager: &mut Manager) {
    let native_functions = NativeFunctions::get_function_definitions();

    native_functions.iter().for_each(|func| {
        let return_address = match func.data_type {
            DataType::Void => None,
            _ => Some(manager.new_global(&func.data_type)),
        };

        manager.new_func(&func, 0, return_address, false);
    });

    let path = path.to_str().unwrap();
    let mut test_program = try_file(path);
    test_program.generate(manager);
}
