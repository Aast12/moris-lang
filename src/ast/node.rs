use super::quadruples::Manager;

/// Represents the node of an Abstract Syntax Tree
pub trait Node<'m> {
    fn generate(&mut self) -> () {
        todo!()
    }

    fn reduce(&self) -> String {
        todo!()
    }

}

pub trait Leaf<'m>: Node<'m> {
    fn dump(&self) -> String;
}
