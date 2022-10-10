use crate::ast::{self, types::DataType, node::Leaf};

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
    manager: Option<&'m ast::quadruples::Manager<'m>>,
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
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager<'m>) -> () {
        self.manager = Some(manager);
    }

    fn generate(&mut self) -> () {}

    fn reduce(&self) -> &dyn ast::node::Leaf {
        return self;
    }
}

impl<'m> ast::expressions::ExpressionT<'m> for Const<'m> {}

impl<'m> Leaf<'m> for Const<'m> {
    fn dump(&self) -> String {
        return self.value.clone();
    }
}

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
