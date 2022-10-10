use crate::ast;
use crate::ast::expressions::Index;
use crate::ast::types;

use super::{Expression, ExpressionT};

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
    pub indexing: Index<'m>,
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

impl<'m> ast::expressions::ExpressionT<'m> for Id<'m> {}

impl<'m> Access<'m> {
    pub fn new(id: Id<'m>, indexing: Index<'m>) -> Self {
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
        self.id.set_manager(manager);
        self.indexing.set_manager(manager);
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        todo!()
    }
}

impl<'m> ast::expressions::ExpressionT<'m> for Access<'m> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::node::Node;
    use crate::ast::quadruples::Manager;
    use crate::ast::types::DataType;

    #[test]
    fn test_id() {
        let manager = Manager::new();
        let test_ids = vec!["id1", "id2"];

        for id_name in test_ids {
            let mut id = Id::new(&id_name, Some(DataType::Float));
            id.set_manager(&manager);

            assert_eq!(id.reduce().dump(), id_name);
        }
    }

    #[test]
    fn test_access() {
        let manager = Manager::new();
        let vec_id_name = "testVec";
        let idx_id_name = "vecIdx";

        let test_location_id = Id::new(idx_id_name, Some(DataType::Int));

        let mut access = Access::new(
            Id::new(vec_id_name, None),
            Index::Simple(Box::new(Expression::Id(test_location_id))),
        );

        access.set_manager(&manager);

        assert_eq!(access.id.id, vec_id_name);
        match access.indexing {
            Index::Simple(expr) => {
                if let Expression::Id(id) = expr.as_ref() {
                    match id.manager {
                        Some(_) => (),
                        None => panic!("Manager not propagated to index."),
                    }
                    assert_eq!(id.id, idx_id_name);
                } else {
                    panic!()
                }
            }
            _ => panic!(),
        }
        match access.id.manager {
            Some(_) => (),
            None => panic!("Manager not propagated to id."),
        }
    }
}
