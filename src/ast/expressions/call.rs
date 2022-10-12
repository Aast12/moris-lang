use crate::ast::{self, types::DataType};

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
        todo!("Implement fn call data type")
    }
}

impl<'m> ast::node::Node<'m> for Call {
    fn generate(&mut self) -> () {
        todo!()
    }

    fn reduce(&self) -> String {
        todo!()
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
                Box::new(Expression::Const(constant::Const::new(
                    "54",
                    types::DataType::Int,
                ))),
                Box::new(Expression::Call(Call::new("arg", vec![]))),
            ],
        );
    }
}
