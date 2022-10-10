use crate::ast::{self, types::DataType};

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    // Vector(Vec<Box<ast::expressions::Expr>>),
}

#[derive(Debug)]
pub struct Const<'m> {
    manager: Option<&'m ast::quadruples::Manager>,
    pub value: String,
    pub dtype: DataType,
}

impl<'m> Const<'m> {
    pub fn new(value: &str, dtype: DataType) -> Self {
        Const {
            manager: None,
            value: String::from(value),
            dtype,
        }
    }
}

impl<'m> ast::node::Node<'m> for Const<'m> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager) -> () {
        self.manager = Some(manager);
    }

    fn generate(&mut self) -> () {
        todo!()
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        todo!()
    }
}

impl<'m> ast::expressions::ExpressionT<'m> for Const<'m> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{quadruples::Manager, node::Node};

    #[test]
    fn test_const() {
        let regi = Manager::new();
        
        let mut constant = Const::new("3", DataType::Int);

        constant.set_manager(&regi);

        assert_eq!(constant.value, "3");
        assert!(constant.manager.is_some());
    }
}
