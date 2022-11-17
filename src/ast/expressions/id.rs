use std::iter::zip;

use crate::ast;
use crate::ast::expressions::Index;
use crate::ast::node::Node;
use crate::ast::types::Operator;
use crate::codegen::manager::GlobalManager;
use crate::codegen::quadruples::Quadruple;
use crate::memory::resolver::MemAddress;
use crate::memory::types::DataType;

#[derive(Debug, Clone)]
pub struct Id {
    pub id: String,
    pub dtype: Option<DataType>,
}

#[derive(Debug, Clone)]
pub struct Access {
    pub id: Id,
    pub indexing: Vec<Index>,
}

impl Id {
    pub fn new(id: &str, dtype: Option<DataType>) -> Self {
        Id {
            id: String::from(id),
            dtype,
        }
    }

    pub fn data_type(&self) -> DataType {
        match &self.dtype {
            Some(dtype) => dtype.clone(),
            _ => {
                let mut man = GlobalManager::get();
                if let Some(id) = man.get_env_mut().get_var(&self.id) {
                    return id.data_type.clone();
                }
                panic!("id {} is not defined", self.id);
            }
        }
    }

    pub fn address(&self) -> MemAddress {
        if let Some(var_entry) = GlobalManager::get().get_env_mut().get_var(&self.id) {
            return var_entry.address;
        } else {
            panic!("Cannot find id {} in scope", self.id);
        }
    }
}

impl ast::node::Node for Id {
    fn reduce(&self) -> String {
        self.address().to_string()
    }
}

impl Access {
    pub fn new(id: Id, indexing: Vec<Index>) -> Self {
        Access { id, indexing }
    }

    pub fn data_type(&mut self) -> DataType {
        return self.id.data_type();
    }
}

impl Node for Access {
    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        if self.indexing.len() == 0 {
            return self.id.address().to_string();
        }

        let indexing_addresses = self
            .indexing
            .iter()
            .map(|index| index.reduce())
            .collect::<Vec<String>>();

        let id_var = GlobalManager::get().get_env_mut().get_var(&self.id.id).cloned();

        if let Some(access_item) = id_var {
            if indexing_addresses.len() > access_item.dimension.dimensions as usize {
                panic!("Incompatible index!");
            }

            let shape_cp = access_item.dimension.shape.clone();
            let mut curr_dim = shape_cp.iter();
            let acc_tmp = GlobalManager::new_temp(&DataType::Int).to_string();
            let first_run = true;

            zip(&indexing_addresses, &access_item.dimension.acc_size).for_each(
                |(index, dim_size)| {
                    if let Some(dim) = curr_dim.next() {
                        GlobalManager::emit(Quadruple::verify(
                            index.as_str(),
                            dim.to_string().as_str(),
                        ))
                    }

                    if first_run {
                        GlobalManager::emit(Quadruple::operation(
                            Operator::Mul,
                            index.as_str(),
                            dim_size.to_string().as_str(),
                            acc_tmp.as_str(),
                        ));
                    } else {
                        let tmp = GlobalManager::new_temp(&DataType::Int);
                        let tmp_str = tmp.to_string();

                        GlobalManager::emit(Quadruple::operation(
                            Operator::Mul,
                            index.as_str(),
                            dim_size.to_string().as_str(),
                            tmp_str.as_str(),
                        ));

                        GlobalManager::emit(Quadruple::operation(
                            Operator::Add,
                            acc_tmp.as_str(),
                            tmp_str.as_str(),
                            acc_tmp.as_str(),
                        ));
                    }
                },
            );

            let access_tmp = GlobalManager::new_temp(&DataType::Pointer);

            GlobalManager::emit(Quadruple::operation(
                Operator::Add,
                format!("&{}", access_item.address).as_str(),
                acc_tmp.as_str(),
                access_tmp.to_string().as_str(),
            ));

            format!("*{}", access_tmp)
        } else {
            panic!("Item {} does not exist!", self.id.id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::Expression;
    use crate::memory::types::DataType;

    #[test]
    fn test_id() {
        let test_ids = vec!["id1", "id2"];

        for id_name in test_ids {
            let id = Id::new(&id_name, Some(DataType::Float));
            assert_eq!(id.id, id_name);
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
