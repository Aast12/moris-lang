use crate::ast;
use crate::ast::expressions::Index;
use crate::ast::types;

use super::Expression;

#[derive(Debug)]
pub struct Id<'m> {
    manager: Option<&'m ast::quadruples::Manager>,
    pub id: String,
    pub dtype: Option<types::DataType>,
}

#[derive(Debug)]
pub struct Access<'m, T: Expression<'m>> {
    manager: Option<&'m ast::quadruples::Manager>,
    pub id: Id<'m>,
    pub indexing: Index<T>,
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

impl<'m> ast::expressions::Expression<'m> for Id<'m> {}

impl<'m, T: Expression<'m>> Access<'m, T> {
    pub fn new(id: Id<'m>, indexing: Index<T>) -> Self {
        Access {
            manager: None,
            id,
            indexing,
        }
    }
}

impl<'m, T: Expression<'m>> ast::node::Node<'m> for Access<'m, T> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager) -> () {
        self.manager = Some(manager);
        self.id.set_manager(manager);
        self.indexing.set_manager(manager);
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        todo!()
    }
}

impl<'m, T: Expression<'m>> ast::expressions::Expression<'m> for Access<'m, T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::node::Node;
    use crate::ast::quadruples::Manager;
    use crate::ast::types::DataType;
    use crate::ast::expressions::constant::Const;

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


        let regi = Manager::new();
        let mut access_ = Access::new(
            Id::new("testVec2", None),
            Index::Simple(Box::new(Const::new(3, DataType::Int)))
        );

        access_.set_manager(&regi);


        if let Index::Simple(indexing) = access_.indexing {
            assert_eq!(indexing.value, 3);
        }

        let mut access = Access::new(
            Id::new(vec_id_name, None),
            Index::Simple(Box::new(test_location_id)),
        );

        access.set_manager(&manager);

        assert_eq!(access.id.id, vec_id_name);
        match access.indexing {
            Index::Simple(idx) => {
                match idx.manager {
                    Some(_) => (),
                    None => panic!("Manager not propagated to index."),
                }
                assert_eq!(idx.id, idx_id_name);
            }
            _ => panic!(),
        }
        match access.id.manager {
            Some(_) => (),
            None => panic!("Manager not propagated to id."),
        }
    }
}
