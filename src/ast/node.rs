use crate::ast::quadruples;

/// Represents the node of an Abstract Syntax Tree
pub trait Node<'m> {
    fn set_manager(&mut self, _: &'m quadruples::Manager) -> () {
        todo!()
    }

    fn generate(&mut self) -> () {
        todo!()
    }

    fn reduce(&self) -> &dyn Leaf {
        todo!()
    }
}

pub trait Leaf<'m>: Node<'m> {
    fn dump(&self) -> String;
}
