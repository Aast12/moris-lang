use manager::Manager;
use memory::types::DataType;
use natives::NativeFunction;
use node::Node;
use parser::try_file;

pub mod ast_nodes;
pub mod env;
pub mod manager;
pub mod meta;
pub mod natives;
pub mod node;
pub mod quadruples;
pub mod symbols;

/// Generates the code for an input file.
///
/// All the program metadata and quadruples will be stored in the manager object.
pub fn generate(path: &str, manager: &mut Manager) {
    let native_functions = NativeFunction::get_function_definitions();

    native_functions.iter().for_each(|func| {
        let return_address = match func.data_type {
            DataType::Void => None,
            _ => Some(manager.new_global(&func.data_type)),
        };

        manager.new_func(&func, 0, return_address, false);
    });

    let mut test_program = try_file(path);
    test_program.generate(manager);
}
