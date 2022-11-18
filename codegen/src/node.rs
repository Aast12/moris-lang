use memory::{types::DataType, resolver::MemAddress};

/// Represents the node of an Abstract Syntax Tree
pub trait Node {
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
