pub mod expressions;
pub mod functions;
pub mod node;
pub mod statements;
pub mod types;

use std::{fmt::Debug, vec};

use crate::memory::types::{FloatType, IntType};

use self::expressions::{constant::Const, Expression};

#[derive(Debug, Clone)]
pub struct Dimension {
    pub dimensions: i8,
    pub shape: Vec<usize>,
    pub size: usize,
    pub acc_size: Vec<usize>,
}

impl Dimension {
    pub fn new_scalar() -> Dimension {
        Dimension {
            dimensions: 0,
            shape: vec![],
            size: 1,
            acc_size: vec![],
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

        let mut new_dim = Dimension {
            dimensions,
            shape: usize_shape,
            size,
            acc_size: vec![],
        };

        new_dim.calc_acc_size();
        new_dim
    }

    fn calc_acc_size(&mut self) {
        if self.acc_size.len() != self.dimensions as usize {
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

            self.acc_size = new_acc_size;
        }
    }
}

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(IntType),
    Float(FloatType),
    String(String),
    Vector(Vec<Box<Expression>>),
}

#[cfg(test)]
mod tests {
    use crate::memory::types::DataType;

    use super::{expressions::constant::Const, Dimension};

    #[test]
    fn test_array_offset() {
        let dim = Dimension::new(
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
        assert_eq!(dim.acc_size, vec![24, 4, 1]);
        // assert_eq!(dim.get_array_offset(vec![0, 0, 3]), 3);
        // assert_eq!(dim.get_array_offset(vec![0, 3, 0]), 3 * 4);
        // assert_eq!(dim.get_array_offset(vec![2, 3, 3]), 63);

        // assert_eq!(dim.get_array_offset(vec![0, 1]), 1 * 4); // TODO: Decide if lower dimension index is valid
    }
}
