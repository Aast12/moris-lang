use crate::{
    ast::{
        self,
        quadruples::{GlobalManager, Quadruple},
    },
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

impl<'m> ast::node::Node<'m> for Call {
    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        for (index, param) in self.params.iter().enumerate() {
            let param_address = param.reduce();
            GlobalManager::emit(Quadruple::new(
                "param",
                param_address.as_str(),
                "",
                index.to_string().as_str(),
            ))
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
