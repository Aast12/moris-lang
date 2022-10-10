use crate::ast::types::{DataType, Operator};

pub struct SemanticRules {}

impl SemanticRules {
    pub fn match_type(operator: Operator, left: DataType, right: DataType) -> DataType {
        match operator {
            Operator::Mul => Self::arith_match(left, right, false),
            Operator::Div => Self::arith_match(left, right, false),
            Operator::Add => Self::arith_match(left, right, true),
            Operator::Sub => Self::arith_match(left, right, false),
            Operator::Pipe => Self::pipe_match(left, right),
            Operator::ForwardPipe => Self::pipe_match(left, right),
            Operator::And => Self::bool_match(left, right),
            Operator::Or => Self::bool_match(left, right),
            Operator::LessThan => Self::comparison_match(left, right),
            Operator::GreaterThan => Self::comparison_match(left, right),
            Operator::NotEq => Self::comparison_match(left, right),
            Operator::Eq => Self::comparison_match(left, right),
            Operator::Assign => Self::assign_match(left, right),
        }
    }

    fn fail(left: DataType, right: DataType) -> ! {
        panic!("{}", format!("Types {:?} and {:?} are not compatible!", left, right));
    }

    fn pipe_match_r(left: DataType, right: DataType) -> DataType {
        match right {
            DataType::Function(func) => DataType::from(func.data_type),
            _ => Self::fail(left, right)
        }
    }

    fn pipe_match(left: DataType, right: DataType) -> DataType {
        match left {
            DataType::Int => Self::pipe_match_r(left, right),
            DataType::Float => Self::pipe_match_r(left, right),
            DataType::Bool => Self::pipe_match_r(left, right),
            DataType::String => Self::pipe_match_r(left, right),
            DataType::DataFrame => Self::pipe_match_r(left, right),
            _ => Self::fail(left, right)
        }
    }

    fn assign_match(left: DataType, right: DataType) -> DataType {
        match left {
            DataType::Int => match right {
                DataType::Int => DataType::Int,
                DataType::Float => DataType::Int,
                DataType::Bool => DataType::Int,
                _ => Self::fail(left, right)
            },
            DataType::Float => match right {
                DataType::Int => DataType::Float,
                DataType::Float => DataType::Float,
                DataType::Bool => DataType::Float,
                _ => Self::fail(left, right)
            },
            DataType::Bool => match right {
                DataType::Int => DataType::Bool,
                DataType::Float => DataType::Bool,
                DataType::Bool => DataType::Bool,
                _ => Self::fail(left, right)
            },
            DataType::String => match right {
                DataType::String => DataType::String,
                _ => Self::fail(left, right)
            },
            DataType::DataFrame => match right {
                DataType::DataFrame => DataType::DataFrame,
                _ => Self::fail(left, right)
            },
            _ => Self::fail(left, right),
        }
    }
    
    fn comparison_match_r(left: DataType, right: DataType) -> DataType {
        match right {
            DataType::Int => DataType::Bool,
            DataType::Float => DataType::Bool,
            DataType::Bool => DataType::Bool,
            _ => Self::fail(left, right),
        }
    }

    fn comparison_match(left: DataType, right: DataType) -> DataType {
        match left {
            DataType::Int => Self::comparison_match_r(left, right),
            DataType::Float => Self::comparison_match_r(left, right),
            DataType::Bool => Self::comparison_match_r(left, right),
            DataType::String => match right {
                DataType::String => DataType::Bool,
                _ => Self::fail(left, right),
            },
            _ => Self::fail(left, right),
        }
    }

    fn bool_match_r(left: DataType, right: DataType) -> DataType {
        match right {
            DataType::Int => DataType::Bool,
            DataType::Float => DataType::Bool,
            DataType::Bool => DataType::Bool,
            DataType::String => DataType::Bool,
            DataType::DataFrame => DataType::Bool,
            _ => Self::fail(left, right),
        }
    }

    fn bool_match(left: DataType, right: DataType) -> DataType {
        match left {
            DataType::Int => Self::bool_match_r(left, right),
            DataType::Float => Self::bool_match_r(left, right),
            DataType::Bool => Self::bool_match_r(left, right),
            DataType::String => Self::bool_match_r(left, right),
            DataType::DataFrame => Self::bool_match_r(left, right),
            _ => Self::fail(left, right),
        }
    }

    fn arith_match(left: DataType, right: DataType, is_add: bool) -> DataType {
        match left {
            DataType::Int => match right {
                DataType::Int => DataType::Int,
                DataType::Float => DataType::Float,
                DataType::Bool => DataType::Int,
                _ => Self::fail(left, right),
            },
            DataType::Float => match right {
                DataType::Int => DataType::Float,
                DataType::Float => DataType::Float,
                DataType::Bool => DataType::Float,
                _ => Self::fail(left, right),
            },
            DataType::Bool => match right {
                DataType::Int => DataType::Int,
                DataType::Float => DataType::Float,
                DataType::Bool => DataType::Int,
                _ => Self::fail(left, right),
            },
            DataType::String => match right {
                DataType::String => {
                    if is_add {
                        DataType::String
                    } else {
                        Self::fail(left, right)
                    }
                }
                _ => Self::fail(left, right),
            },
            _ => Self::fail(left, right),
        }
    }
}
