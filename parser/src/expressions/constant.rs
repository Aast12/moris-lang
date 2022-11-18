use memory::types::{DataType, FloatType, IntType};

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(IntType),
    Float(FloatType),
    String(String),
    // Vector(Vec<Box<ast::expressions::Expr>>),
}

#[derive(Debug, Clone)]
pub struct Const {
    pub value: String,
    pub dtype: DataType,
}

impl Const {
    pub fn new(value: &str, dtype: DataType) -> Self {
        Const {
            value: String::from(value),
            dtype,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const() {
        let constant = Const::new("3", DataType::Int);

        assert_eq!(constant.value, "3");
    }
}
