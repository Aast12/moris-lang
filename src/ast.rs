pub mod expressions;
pub mod functions;
pub mod node;
pub mod statements;
pub mod types;

use std::{fmt::Debug, iter::zip, thread::panicking, vec};

use self::{
    expressions::{constant::Const, Expression},
    node::Node,
};

#[derive(Debug, Clone)]
pub struct Dimension {
    pub dimensions: i8,
    pub shape: Vec<usize>,
    pub size: usize,
    pub acc_size: Option<Vec<usize>>,
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
            acc_size: None,
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
            acc_size: None,
        }
    }

    fn calc_acc_size(&mut self) -> Option<&Vec<usize>> {
        if let None = self.acc_size {
            let mut new_acc_size = self
                .shape
                .iter()
                .rev()
                .scan(1, |acc, &curr| {
                    *acc = *acc * curr;
                    Some(*acc / curr)
                })
                .collect::<Vec<usize>>();
            new_acc_size.reverse();
            self.acc_size = Some(new_acc_size);
        }

        self.acc_size.as_ref()
    }

    pub fn get_array_offset(&mut self, access: Vec<usize>) -> usize {
        println!("{:#?}", access);
        if access.len() > self.shape.len() {
            panic!("Incompatible index!")
        }
        let shape_cp = self.shape.clone();
        let mut curr_dim = shape_cp.iter();

        if let Some(acc_size) = self.calc_acc_size() {
            let offset = zip(access, acc_size).fold(0, |acc, (index, dim_size)| {
                if let Some(dim) = curr_dim.next() {
                    if index >= *dim {
                        panic!("Index out of bounds");
                    }
                }
                acc + index * *dim_size
            });
            offset
        } else {
            0
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

#[cfg(test)]
mod tests {
    use crate::memory::types::DataType;

    use super::{expressions::constant::Const, Dimension};

    #[test]
    fn test_array_offset() {
        let mut dim = Dimension::new(
            3,
            vec![
                Const {
                    dtype: DataType::Int,
                    value: String::from("7"),
                },
                Const {
                    dtype: DataType::Int,
                    value: String::from("6"),
                },
                Const {
                    dtype: DataType::Int,
                    value: String::from("4"),
                },
            ],
        );
        assert_eq!(dim.get_array_offset(vec![0, 0, 3]), 3);
        assert_eq!(dim.get_array_offset(vec![0, 3, 0]), 3 * 4);
        assert_eq!(dim.get_array_offset(vec![2, 3, 3]), 63);

        assert_eq!(dim.get_array_offset(vec![0, 1]), 1 * 4); // TODO: Decide if lower dimension index is valid
    }
}
