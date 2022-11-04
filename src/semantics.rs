use crate::{
    ast::types::{Operator, Operator::*},
    memory::types::{DataType, DataType::*},
};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum SemanticContext {
    Global,
    Function,
    Loop,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum ExitStatement {
    Break,
    Continue,
    Return,
}

pub struct SemanticRules {}

impl SemanticRules {
    pub fn match_type(operator: Operator, left: DataType, right: DataType) -> DataType {
        match operator {
            Div => match left {
                Int | Bool | Float => match right {
                    Int | Float | Bool => Float,
                    _ => Self::fail(left, right),
                },
                _ => Self::fail(left, right),
            },
            Add => Self::arith_match(left, right, true),
            Sub | Mul => Self::arith_match(left, right, false),
            Pipe | ForwardPipe => Self::pipe_match(left, right),
            And | Or => Self::bool_match(left, right),
            LessThan | GreaterThan | NotEq | Eq | LessOrEq | GreaterOrEq => {
                Self::comparison_match(left, right)
            }
            Assign => Self::assign_match(left, right),
        }
    }

    fn fail(left: DataType, right: DataType) -> ! {
        panic!(
            "{}",
            format!("Types {:?} and {:?} are not compatible!", left, right)
        );
    }

    fn pipe_match_r(left: DataType, right: DataType) -> DataType {
        match right {
            Function(func) => *func.clone(),
            _ => Self::fail(left, right),
        }
    }

    fn pipe_match(left: DataType, right: DataType) -> DataType {
        match left {
            Int | Float | Bool | String | DataFrame => Self::pipe_match_r(left, right),
            _ => Self::fail(left, right),
        }
    }

    fn assign_match(left: DataType, right: DataType) -> DataType {
        match left {
            Int => match right {
                Int | Float | Bool => Int,
                _ => Self::fail(left, right),
            },
            Float => match right {
                Int | Float | Bool => Float,
                _ => Self::fail(left, right),
            },
            Bool => match right {
                Int | Float | Bool => Bool,
                _ => Self::fail(left, right),
            },
            String => match right {
                String => String,
                _ => Self::fail(left, right),
            },
            DataFrame => match right {
                DataFrame => DataFrame,
                _ => Self::fail(left, right),
            },
            _ => Self::fail(left, right),
        }
    }

    fn comparison_match_r(left: DataType, right: DataType) -> DataType {
        match right {
            Int | Float | Bool => Bool,
            _ => Self::fail(left, right),
        }
    }

    fn comparison_match(left: DataType, right: DataType) -> DataType {
        match left {
            Int | Float | Bool => Self::comparison_match_r(left, right),
            String => match right {
                String => Bool,
                _ => Self::fail(left, right),
            },
            _ => Self::fail(left, right),
        }
    }

    fn bool_match_r(left: DataType, right: DataType) -> DataType {
        match right {
            Int | Float | Bool | String | DataFrame => Bool,
            _ => Self::fail(left, right),
        }
    }

    fn bool_match(left: DataType, right: DataType) -> DataType {
        match left {
            Int | Float | Bool | String | DataFrame => Self::bool_match_r(left, right),
            _ => Self::fail(left, right),
        }
    }

    fn arith_match(left: DataType, right: DataType, is_add: bool) -> DataType {
        match left {
            Int => match right {
                Int | Bool => Int,
                Float => Float,
                _ => Self::fail(left, right),
            },
            Float => match right {
                Int | Float | Bool => Float,
                _ => Self::fail(left, right),
            },
            Bool => match right {
                Int | Bool => Int,
                Float => Float,
                _ => Self::fail(left, right),
            },
            String => match right {
                String => {
                    if is_add {
                        String
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
