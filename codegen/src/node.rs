use memory::{resolver::MemAddress, types::DataType};
use parser::Dimension;

/// Represents the node of an Abstract Syntax Tree
pub trait Node {
    fn dimensionality(&self) -> Vec<usize> {
        vec![]
    }

    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        todo!("reduce raw node");
    }

    fn data_type(&self) -> DataType {
        todo!("");
    }

    fn address(&self) -> MemAddress {
        todo!("");
    }
}
