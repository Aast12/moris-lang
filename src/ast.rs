pub mod expressions;
pub mod functions;
pub mod node;
pub mod statements;
pub mod types;

use std::{fmt::Debug, vec};

use self::{
    expressions::{constant::Const, Expression},
    node::Node,
};

#[derive(Debug, Clone)]
pub struct Dimension {
    pub dimensions: i8,
    pub shape: Vec<usize>,
    pub size: usize,
} // dimensions number, dimension sizes

impl Node for Dimension {
    fn generate(&mut self) -> () {
        // for dim in self.1.iter_mut() {
        //     dim.generate();
        // }
        todo!()
    }
}

impl Dimension {
    pub fn new_scalar() -> Dimension {
        Dimension {
            dimensions: 0,
            shape: vec![],
            size: 1,
        }
    }

    pub fn new(dimensions: i8, shape: Vec<Const>) -> Dimension {
        let usize_shape: Vec<usize> = shape
            .iter()
            .map(
                |constant| match str::parse::<usize>(constant.value.as_str()) {
                    Ok(size) => {
                        if size <= 0 {
                            panic!("Invalid dimension size {size} in array declaration.")
                        }
                        size
                    }
                    Err(error) => panic!("Can't parse variable dimension: {:#?}", error),
                },
            )
            .collect();

        let size = usize_shape.iter().fold(1, |acc, item| acc * item);

        Dimension {
            dimensions,
            shape: usize_shape,
            size,
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
