use crate::ast::{self, types::DataType};

use super::Expression;

#[derive(Debug)]
pub struct Call<'m> {
    manager: Option<&'m ast::quadruples::Manager<'m>>,
    pub id: String,
    pub params: Vec<Box<Expression<'m>>>,
}

impl<'m> Call<'m> {
    pub fn new(id: &str, params: Vec<Box<Expression<'m>>>) -> Self {
        Call {
            manager: None,
            id: String::from(id),
            params,
        }
    }

    pub fn data_type(&self) -> DataType {
        todo!("Implement fn call data type")
    }
}

impl<'m> ast::node::Node<'m> for Call<'m> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager<'m>) -> () {
        self.manager = Some(manager);
        for param in self.params.iter_mut() {
            param.set_manager(manager);
        }
    }

    fn generate(&mut self) -> () {
        todo!()
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        todo!()
    }
}

impl<'m> ast::expressions::ExpressionT<'m> for Call<'m> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::*;
    use crate::ast::node::Node;
    use crate::ast::quadruples::Manager;

    #[test]
    fn test_function() {
        let manager = Manager::new();
        let fn_name = "testFn";

        let mut call = Call::new(
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

        call.set_manager(&manager);
    }
}
