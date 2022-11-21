use std::cmp::Ordering;

use memory::{resolver::MemAddress, types::DataType};
use parser::{types::{Variable, Operator}, expressions::id::{Access, Id}, statements::{Statement, Block, Program}, functions::Function};

use crate::{node::{Node, AccessNode}, manager::Manager, quadruples::Quadruple};

pub mod expressions;
pub mod statements;

impl AccessNode for Variable {
    // TODO: Refactor to use Id
    fn address(&self, manager: &mut Manager) -> MemAddress {
        if let Some(var_entry) = manager.get_env().get_var(&self.id) {
            return var_entry.address;
        } else {
            panic!("Cannot find id {} in scope", self.id);
        }
    }
}

impl Node for Variable {
    fn generate(&mut self, manager: &mut Manager) -> () {
        // Add variable to symbols table
        let var_address =
            manager
                .get_env_mut()
                .add_var(&self.id, &self.data_type, &self.dimension, false);

        if self.dimension.size > 1 {
            let array_address = manager
                .get_env_mut()
                .allocate_array(&self.data_type, &self.dimension);

            manager.emit(Quadruple::operation(
                Operator::Assign,
                format!("&{}", array_address).as_str(),
                "",
                format!("{}", var_address).as_str(),
            ));

            manager.emit(Quadruple::operation(
                Operator::Assign,
                "END",
                "",
                format!("{}", array_address + self.dimension.size as MemAddress).as_str(),
            ));
        }

        if let Some(value) = &self.value {
            let mut assign = Statement::VarAssign(
                Access::new(
                    Id::new(self.id.as_str(), Some(self.data_type.clone())),
                    vec![],
                ),
                value.to_owned(),
            );

            assign.generate(manager);
        }
    }
}

impl Node for Block {
    fn generate(&mut self, manager: &mut Manager) -> () {
        for stmt in self.0.iter_mut() {
            stmt.generate(manager);
        }
    }
}

impl Node for Program {
    fn generate(&mut self, manager: &mut Manager) -> () {
        let Program(statements) = self;
        statements.sort_by(|a, b| match a {
            Statement::FunctionDeclaration(_) => Ordering::Greater,
            _ => match b {
                Statement::FunctionDeclaration(_) => Ordering::Less,
                _ => Ordering::Equal,
            },
        });

        // Pre-declare function signatures
        for stmt in statements.iter_mut().rev() {
            match stmt {
                Statement::FunctionDeclaration(func) => {
                    let return_address = match func.signature.data_type {
                        DataType::Void => None,
                        _ => Some(manager.new_global(&func.signature.data_type)),
                    };

                    manager.new_func(&func.signature, 0, return_address, false);
                    // TODO: improve undefined location
                }
                _ => break,
            }
        }

        let mut last_func_generated = false;

        for stmt in statements.iter_mut() {
            if !last_func_generated {
                match stmt {
                    Statement::FunctionDeclaration(_) => {
                        last_func_generated = true;
                        manager.emit(Quadruple::end_program());
                    }
                    _ => (),
                }
            }

            stmt.generate(manager);
        }
    }
}

impl Node for Function {
    fn generate(&mut self, manager: &mut Manager) -> () {
        let next_position = manager.get_next_pos();

        manager.update_func_position(&self.signature.id, next_position);
        manager.get_env_mut().switch(&self.signature.id);

        self.block.generate(manager);

        manager.emit(Quadruple::end_func());

        manager.get_env_mut().switch(&String::from("global"));
    }
}
