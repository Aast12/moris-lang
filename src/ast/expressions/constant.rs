use crate::ast::{self, types::DataType};

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector(Vec<Box<ast::expressions::Expr>>),
}

pub struct Const<'m, T> {
    manager: Option<&'m ast::quadruples::Manager>,
    pub value: T,
    pub dtype: DataType,
}

impl<'m, T> Const<'m, T> {
    pub fn new(value: T, dtype: DataType) -> Self {
        Const {
            manager: None,
            value,
            dtype,
        }
    }
}

impl<'m, T> ast::node::Node<'m> for Const<'m, T> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager) -> () {
        self.manager = Some(manager);
    }

    fn generate(&self) -> () {
        todo!()
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        todo!()
    }
}

impl<'m, T> ast::expressions::Expression<'m> for Const<'m, T> {}
