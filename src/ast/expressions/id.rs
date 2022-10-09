use crate::ast;
use crate::ast::types;

#[derive(Debug)]
pub struct Id<'m> {
    manager: Option<&'m ast::quadruples::Manager>,
    pub id: String,
    pub dtype: types::DataType,
}

impl<'m> Id<'m> {
    pub fn new(id: &str, dtype: types::DataType) -> Self {
        Id {
            manager: None,
            id: String::from(id),
            dtype
        }
    }
}



impl<'m> ast::node::Leaf for Id<'m> {
    fn dump(&self) -> String {
        return self.id.clone();
    }
}

impl<'m> ast::node::Node<'m> for Id<'m> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager) -> () {
        self.manager = Some(manager);
    }

    fn generate(&self) -> () {
        todo!()
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        self
    }
}
