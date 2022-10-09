use crate::ast::quadruples;

/// Represents the node of an Abstract Syntax Tree
pub trait Node<'m> {
    fn set_manager(&mut self, manager: &'m quadruples::Manager) -> ();
    fn generate(&self) -> ();
    fn reduce(&self) -> &dyn Leaf;
}

pub trait Leaf {
    fn dump(&self) -> String;
}

impl<'m> Node<'m> for dyn Leaf {
    fn set_manager(&mut self, _: &'m quadruples::Manager) -> () {
        todo!()
    }

    fn generate(&self) -> () {
        panic!("Leaf node do not generate quadruples.")
    }

    fn reduce(&self) -> &dyn Leaf {
        panic!("Leaf node cannot be more reduced.")
    }
}
