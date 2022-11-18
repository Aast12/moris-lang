use crate::{
    ast::{node::Node, types::Operator},
    codegen::{manager::GlobalManager, quadruples::Quadruple},
};

use super::Expression;
use memory::types::DataType;

#[derive(Debug, Clone)]
pub struct Call {
    pub id: String,
    pub params: Vec<Box<Expression>>,
}

impl Call {
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

impl Node for Call {
    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        let man = GlobalManager::get();
        let func = man.get_func(&self.id).clone();
        let return_type = func.return_type.clone();
        let param_defintions = func.params.clone();
        drop(man);
        let target_params_len = param_defintions.len();
        if self.params.len() != target_params_len {
            panic!(
                "Params size do not match {} {} - {}",
                self.id,
                self.params.len(),
                target_params_len
            );
        }

        GlobalManager::emit(Quadruple::era(self.id.as_str()));

        for (index, param) in self.params.iter().enumerate() {
            let (_, def_param_data_type, param_pointer_addr) = param_defintions.get(index).unwrap();
            let mut param_address = param.reduce();
            let param_data_type = param.data_type();
            assert!(
                DataType::equivalent(&param_data_type, def_param_data_type).is_ok(),
                "Data type {:?} cannot be assigned to a variable {:?}.",
                param_data_type,
                def_param_data_type
            );

            // TODO: Refactor type casting instruction into func
            if param_data_type != *def_param_data_type {
                let value_temp = GlobalManager::new_temp(&def_param_data_type).to_string();

                GlobalManager::emit(Quadruple::type_cast(
                    &def_param_data_type,
                    param_address.as_str(),
                    value_temp.as_str(),
                ));

                param_address = value_temp.clone();
            }

            // if let Some(_) = param_pointer_addr {
            //     GlobalManager::emit(Quadruple::param(format!("*{}", param_address).as_str(), index));
            // } else {
            //     GlobalManager::emit(Quadruple::param(param_address.as_str(), index));
            // }
            GlobalManager::emit(Quadruple::param(param_address.as_str(), index));
        }

        GlobalManager::emit(Quadruple::go_sub(self.id.as_str()));

        let mut manager = GlobalManager::get();
        if let Some(address) = manager.get_func_return(&self.id) {
            let return_value = manager.new_temp_address(&return_type).to_string();
            manager.emit(Quadruple::unary(
                Operator::Assign,
                address.to_string().as_str(),
                return_value.as_str(),
            ));
            return_value
        } else {
            String::from("VOID")
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
