pub mod expressions;
pub mod functions;
pub mod node;
pub mod statements;
pub mod types;

use std::fmt::Debug;

use self::{expressions::Expression, node::Node};

#[derive(Debug)]
pub struct Dimension(pub i8, pub Vec<Box<Expression>>); // dimensions number, dimension sizes

impl<'m> Node<'m> for Dimension {
    fn generate(&mut self) -> () {
        for dim in self.1.iter_mut() {
            dim.generate();
        }
    }
}

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector(Vec<Box<Expression>>),
}