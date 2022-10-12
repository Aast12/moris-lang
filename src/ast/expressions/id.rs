use crate::ast;
use crate::ast::expressions::Index;
use crate::ast::quadruples::MANAGER;
use crate::ast::types::{self, DataType};

#[derive(Debug)]
pub struct Id {
    pub id: String,
    pub dtype: Option<types::DataType>,
}

#[derive(Debug)]
pub struct Access {
    pub id: Id,
    pub indexing: Vec<Index>,
}

impl Id {
    pub fn new(id: &str, dtype: Option<types::DataType>) -> Self {
        Id {
            id: String::from(id),
            dtype,
        }
    }

    pub fn data_type(&self) -> DataType {
        match &self.dtype {
            Some(dtype) => dtype.clone(),
            _ => {
                let mut man = MANAGER.lock().unwrap();
                if let Some(id) = man.get_env().get_var(&self.id) {
                    return id.data_type.clone();
                }
                panic!("id {} is not defined", self.id);
            }
        }
    }
}

impl<'m> ast::node::Node<'m> for Id {
    fn reduce(&self) -> String {
        return self.id.clone();
    }
}

impl Access {
    pub fn new(id: Id, indexing: Vec<Index>) -> Self {
        Access { id, indexing }
    }

    pub fn data_type(&self) -> DataType {
        todo!("Implement access data type")
    }
}

impl<'m> ast::node::Node<'m> for Access {
    fn reduce(&self) -> String {
        return self.id.id.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::Expression;
    use crate::ast::node::Node;
    use crate::ast::types::DataType;

    #[test]
    fn test_id() {
        let test_ids = vec!["id1", "id2"];

        for id_name in test_ids {
            let id = Id::new(&id_name, Some(DataType::Float));

            assert_eq!(id.reduce(), id_name);
        }
    }

    #[test]
    fn test_access() {
        let vec_id_name = "testVec";
        let idx_id_name = "vecIdx";

        let test_location_id = Id::new(idx_id_name, Some(DataType::Int));

        let access = Access::new(
            Id::new(vec_id_name, None),
            vec![Index::Simple(Box::new(Expression::Id(test_location_id)))],
        );

        assert_eq!(access.id.id, vec_id_name);
        let indexing_fst = access.indexing.get(0);
        if let Some(indexing) = indexing_fst {
            match indexing {
                Index::Simple(expr) => {
                    if let Expression::Id(id) = expr.as_ref() {
                        assert_eq!(id.id, idx_id_name);
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            }
        }
    }
}
