use crate::{ast::node::Node, memory::types::{DataType, IntType}, codegen::manager::GlobalManager};

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(IntType),
    Float(f32),
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

impl Node for Const {
    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        let const_address = GlobalManager::new_constant(&self.dtype, self);
        return const_address.to_string();
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
