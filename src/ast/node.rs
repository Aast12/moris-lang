use crate::memory::resolver::MemAddress;

/// Represents the node of an Abstract Syntax Tree
pub trait Node<'m> {
    fn generate(&mut self) -> () {
        todo!("generate raw node");
    }

    fn reduce(&self) -> String {
        todo!("reduce raw node");
    }
}
