pub mod expressions;
pub mod functions;
pub mod node;
pub mod statements;
pub mod types;

use std::fmt::Debug;

use self::{expressions::{Expression, constant::Const}, node::Node};

#[derive(Debug)]
pub struct Dimension(pub i8, pub Vec<Const>); // dimensions number, dimension sizes

impl Node for Dimension {
    fn generate(&mut self) -> () {
        // for dim in self.1.iter_mut() {
        //     dim.generate();
        // }
        todo!()
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