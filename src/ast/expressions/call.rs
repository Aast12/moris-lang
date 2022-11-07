use crate::{
    ast::node::Node,
    codegen::{manager::GlobalManager, quadruples::Quadruple},
    memory::types::DataType,
};

use super::Expression;

#[derive(Debug)]
pub struct Call {
    pub id: String,
    pub params: Vec<Box<Expression>>,
}

impl<'m> Call {
    pub fn new(id: &str, params: Vec<Box<Expression>>) -> Self {
        Call {
            id: String::from(id),
            params,
        }
    }

    pub fn data_type(&self) -> DataType {
        GlobalManager::get().get_func(&self.id).return_type.clone()
    }
}

impl<'m> Node<'m> for Call {
    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        let target_params = GlobalManager::get().get_func(&self.id).params.clone();
        let target_params_len = target_params.len();
        if self.params.len() != target_params_len {
            panic!(
                "Params size does not match {} {} - {}",
                self.id,
                self.params.len(),
                target_params_len
            );
        }

        GlobalManager::emit(Quadruple::new("era", "", "", self.id.as_str()));

        for (index, param) in self.params.iter().enumerate() {
            let (_, target_data_type) = target_params.get(index).unwrap();
            let mut param_address = param.reduce();
            let param_data_type = param.data_type();
            assert!(
                DataType::equivalent(&param_data_type, target_data_type).is_ok(),
                "Data type {:?} cannot be assigned to a variable {:?}.",
                param_data_type,
                target_data_type
            );

            // TODO: Refactor type casting instruction into func
            if param_data_type != *target_data_type {
                let value_temp = GlobalManager::new_temp(&target_data_type).to_string();

                GlobalManager::emit(Quadruple(
                    String::from(format!("{:?}", target_data_type)),
                    param_address,
                    String::new(),
                    value_temp.clone(),
                ));

                param_address = value_temp.clone();
            }

            GlobalManager::emit(Quadruple::new(
                "param",
                param_address.as_str(),
                "",
                index.to_string().as_str(),
            ));
        }

        GlobalManager::emit(Quadruple::new("gosub", "", "", self.id.as_str()));

        if let Some(address) = GlobalManager::get().get_func_return(&self.id) {
            address.to_string()
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::*;

    #[test]
    fn test_function() {
        let fn_name = "testFn";

        Call::new(
            fn_name,
            vec![
                Box::new(Expression::Id(id::Id::new(fn_name, None))),
                Box::new(Expression::Const(constant::Const::new("54", DataType::Int))),
                Box::new(Expression::Call(Call::new("arg", vec![]))),
            ],
        );
    }
}
