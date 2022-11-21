use memory::{resolver::MemAddress, types::DataType};

use crate::manager::Manager;

/// Represents the node of an Abstract Syntax Tree
pub trait Node {
    fn dimensionality(&self, _: &mut Manager) -> Vec<usize> {
        vec![]
    }

    fn generate(&mut self, manager: &mut Manager) -> () {
        self.reduce(manager);
    }

    fn reduce(&self, _: &mut Manager) -> String {
        todo!("reduce raw node");
    }

    fn data_type(&self, _: &mut Manager) -> DataType {
        todo!();
    }

    fn address(&self, _: &mut Manager) -> MemAddress {
        todo!();
    }
}
