use memory::types::DataType;

use crate::Dimension;

use super::expressions::Expression;

#[derive(Clone, Copy, Debug)]
pub enum OperatorType {
    Arithmetic,
    Pipe,
    Boolean,
    Comparison,
    Assign,
    Neg,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
    Pipe,
    ForwardPipe,
    And,
    Or,
    LessThan,
    GreaterThan,
    LessOrEq,
    GreaterOrEq,
    NotEq,
    Eq,
    Assign,
    Not,
    Neg,
}

impl Operator {
    pub fn to_string(&self) -> &str {
        match self {
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Pipe => "|>",
            Operator::ForwardPipe => "|> fwd",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::LessThan => "<",
            Operator::GreaterThan => ">",
            Operator::NotEq => "!=",
            Operator::Eq => "==",
            Operator::Assign => "=",
            Operator::LessOrEq => "<=",
            Operator::GreaterOrEq => ">=",
            Operator::Not => "not",
            Operator::Neg => "neg",
        }
    }

    pub fn is_arithmetic(&self) -> bool {
        match self {
            Operator::Mul | Operator::Div | Operator::Add | Operator::Sub => true,
            _ => false,
        }
    }

    pub fn is_comparison(&self) -> bool {
        match self {
            Operator::LessThan
            | Operator::GreaterThan
            | Operator::LessOrEq
            | Operator::GreaterOrEq
            | Operator::NotEq
            | Operator::Eq => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Operator::And | Operator::Or | Operator::Not => true,
            _ => false,
        }
    }

    pub fn which(&self) -> OperatorType {
        match self {
            Operator::LessThan
            | Operator::GreaterThan
            | Operator::LessOrEq
            | Operator::GreaterOrEq
            | Operator::NotEq
            | Operator::Eq => OperatorType::Comparison,
            Operator::And | Operator::Or | Operator::Not => OperatorType::Boolean,
            Operator::Mul | Operator::Div | Operator::Add | Operator::Sub => {
                OperatorType::Arithmetic
            }
            Operator::Pipe | Operator::ForwardPipe => OperatorType::Pipe,
            Operator::Assign => OperatorType::Assign,
            Operator::Neg => OperatorType::Neg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub id: String,
    pub data_type: DataType,
    pub dimension: Dimension,
    pub value: Option<Box<Expression>>,
}

impl Variable {
    pub fn new(
        id: String,
        data_type: DataType,
        dimension: Dimension,
        value: Option<Box<Expression>>,
    ) -> Variable {
        Variable {
            id,
            data_type,
            dimension,
            value,
        }
    }

    // // TODO: Refactor to use Id
    // pub fn address(&self) -> MemAddress {
    //     if let Some(var_entry) = GlobalManager::get().get_env_mut().get_var(&self.id) {
    //         return var_entry.address;
    //     } else {
    //         panic!("Cannot find id {} in scope", self.id);
    //     }
    // }
}
