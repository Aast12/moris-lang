use crate::ast;

use super::Expression;

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector(Vec<Box<ast::expressions::Expr>>),
}

pub struct Const<'m> {
    manager: &'m ast::quadruples::Manager
}

impl<'m> ast::node::Node<'m> for Const<'m> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager) -> () {
        self.manager = manager;
    }
    
    fn generate(&self) -> () {
        todo!()
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        todo!()
    }
}

impl<'m> ast::expressions::Expression<'m> for Const<'m> {}

