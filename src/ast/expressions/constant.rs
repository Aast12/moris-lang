use crate::ast::{self, node::Leaf, quadruples::Manager, types::DataType};

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    // Vector(Vec<Box<ast::expressions::Expr>>),
}

#[derive(Debug)]
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

impl<'m> ast::node::Node<'m> for Const {
    fn generate(&mut self) -> () {
        todo!("Const generate");
    }

    fn reduce(&self) -> String {
        return self.value.clone();
    }
}

impl<'m> ast::expressions::ExpressionT<'m> for Const {}

impl<'m> Leaf<'m> for Const {
    fn dump(&self) -> String {
        return self.value.clone();
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
