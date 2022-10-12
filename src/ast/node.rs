use super::quadruples::Manager;

/// Represents the node of an Abstract Syntax Tree
pub trait Node<'m> {
    fn set_manager(&mut self, _: &'m Manager) -> () {
        todo!()
    }

    fn generate(&mut self) -> () {
        todo!()
    }

    fn reduce(&self) -> String {
        todo!()
    }

    fn gen(&mut self, _: &'m mut Manager) -> &'m mut Manager {
        todo!()
    }
}

pub trait Leaf<'m>: Node<'m> {
    fn dump(&self) -> String;
}
