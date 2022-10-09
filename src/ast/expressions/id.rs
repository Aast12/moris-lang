use crate::ast;
use crate::ast::types;

#[derive(Debug)]
pub struct Id<'m> {
    manager: Option<&'m ast::quadruples::Manager>,
    pub id: String,
    pub dtype: Option<types::DataType>,
}

#[derive(Debug)]
pub struct Access<'m> {
    manager: Option<&'m ast::quadruples::Manager>,
    pub id: Id<'m>,
    pub indexing: ast::expressions::Index,
}

impl<'m> Id<'m> {
    pub fn new(id: &str, dtype: Option<types::DataType>) -> Self {
        Id {
            manager: None,
            id: String::from(id),
            dtype,
        }
    }
}

impl<'m> ast::node::Node<'m> for Id<'m> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager) -> () {
        self.manager = Some(manager);
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        self
    }
}

impl<'m> ast::node::Leaf<'m> for Id<'m> {
    fn dump(&self) -> String {
        return self.id.clone();
    }
}

impl<'m> Access<'m> {
    pub fn new(id: Id<'m>, indexing: ast::expressions::Index) -> Self {
        Access {
            manager: None,
            id,
            indexing,
        }
    }
}

impl<'m> ast::node::Node<'m> for Access<'m> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager) -> () {
        self.manager = Some(manager);
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::node::Node;
    use crate::ast::quadruples::Manager;
    use crate::ast::types::DataType;

    #[test]
    fn test_id() {
        let m = Manager::new();
        let test_ids = vec!["id1", "id2"];

        for id_name in test_ids {
            let mut id = Id::new(&id_name, Some(DataType::Float));
            id.set_manager(&m);

            assert_eq!(id.reduce().dump(), id_name);
        }
    }
}