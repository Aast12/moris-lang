use serde::{Deserialize, Serialize};

pub type IntType = i64;
pub type FloatType = f64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
    DataFrame,
    Series,
    Iterable(Box<DataType>),
    Void,
    Function(Box<DataType>),
    Pointer,
}

impl DataType {
    fn hierarchy(dtype: &DataType) -> u8 {
        match dtype {
            DataType::Bool => 0,
            DataType::Int => 1,
            DataType::Float => 2,
            DataType::String => 3,
            DataType::Series => 4,
            DataType::DataFrame => 5,
            DataType::Void => 6,
            DataType::Function(_) => 7,
            DataType::Iterable(_) => todo!(),
            _ => todo!(),
        }
    }

    pub fn max(left: &DataType, right: &DataType) -> DataType {
        if Self::hierarchy(&left) > Self::hierarchy(&right) {
            left.clone()
        } else {
            right.clone()
        }
    }

    pub fn equivalent(left: &DataType, right: &DataType) -> Result<DataType, ()> {
        let ok_ret = Ok(Self::max(&left, &right));
        match left {
            DataType::Int => match right {
                DataType::Int | DataType::Float | DataType::Bool => ok_ret,
                _ => Err(()),
            },
            DataType::Float => match right {
                DataType::Int | DataType::Float | DataType::Bool => ok_ret,
                _ => Err(()),
            },
            DataType::Bool => match right {
                DataType::Int | DataType::Float | DataType::Bool => ok_ret,
                _ => Err(()),
            },
            DataType::String => match right {
                DataType::String => ok_ret,
                _ => Err(()),
            },
            DataType::DataFrame => match right {
                DataType::DataFrame => ok_ret,
                _ => Err(()),
            },
            DataType::Series => match right {
                DataType::Series => ok_ret,
                _ => Err(()),
            },
            DataType::Void => Err(()),
            DataType::Function(func) => Self::equivalent(func, right),
            DataType::Iterable(_) => todo!(),
            _ => todo!(),
        }
    }
}
