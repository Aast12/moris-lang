use super::Expression;

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

    // pub fn data_type(&self) -> DataType {
    //     GlobalManager::get().get_func(&self.id).return_type.clone()
    // }
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
