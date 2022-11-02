#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
    DataFrame,
    Void,
    Function(Box<DataType>),
}

impl DataType {
    fn hierarchy(dtype: &DataType) -> u8 {
        match dtype {
            DataType::Bool => 0,
            DataType::Int => 1,
            DataType::Float => 2,
            DataType::String => 3,
            DataType::DataFrame => 4,
            DataType::Void => 5,
            DataType::Function(_) => 6,
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
                DataType::Int => ok_ret,
                DataType::Float => ok_ret,
                DataType::Bool => ok_ret,
                _ => Err(()),
            },
            DataType::Float => match right {
                DataType::Int => ok_ret,
                DataType::Float => ok_ret,
                DataType::Bool => ok_ret,
                _ => Err(()),
            },
            DataType::Bool => match right {
                DataType::Int => ok_ret,
                DataType::Float => ok_ret,
                DataType::Bool => ok_ret,
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
            DataType::Void => Err(()),
            DataType::Function(func) => Self::equivalent(func, right),
        }
    }
}