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
}
