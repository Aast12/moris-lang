use crate::{
    ast::{
        expressions::id::{Access, Id},
        statements::Statement,
    },
    codegen::{
        manager::{GlobalManager, Manager},
        quadruples::Quadruple,
    },
    memory::{resolver::MemAddress, types::DataType},
};

use super::{expressions::Expression, node::Node, Dimension};

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

    // TODO: Refactor to use Id
    pub fn address(&self) -> MemAddress {
        if let Some(var_entry) = GlobalManager::get().get_env_mut().get_var(&self.id) {
            return var_entry.address;
        } else {
            panic!("Cannot find id {} in scope", self.id);
        }
    }

    pub fn _generate(&mut self, manager: &mut Manager) -> () {
        // Add variable to symbols table
        let var_address = manager
            .get_env_mut()
            .add_var(&self.id, &self.data_type, &self.dimension);

        if self.dimension.size > 1 {
            let array_address = manager
                .get_env_mut()
                .allocate_array(&self.data_type, &self.dimension);
            
            manager.emit(Quadruple::operation(
                Operator::Assign,
                format!("&{}", array_address).as_str(),
                "",
                format!("{}", var_address).as_str(),
            ))
        }

        drop(manager);

        if let Some(value) = &self.value {
            let mut assign = Statement::VarAssign(
                Access::new(
                    Id::new(self.id.as_str(), Some(self.data_type.clone())),
                    vec![],
                ),
                value.to_owned(),
            );

            assign.generate();
        }
    }
}

impl Node for Variable {
    fn generate(&mut self) -> () {
        // Add variable to symbols table
        let mut manager = GlobalManager::get();
        let var_address = manager
            .get_env_mut()
            .add_var(&self.id, &self.data_type, &self.dimension);

        if self.dimension.size > 1 {
            let array_address = manager
                .get_env_mut()
                .allocate_array(&self.data_type, &self.dimension);
            println!(
                "ALLOCATION ARRAY ADDRESS {} - {}",
                var_address, array_address
            );
            manager.emit(Quadruple::operation(
                Operator::Assign,
                format!("&{}", array_address).as_str(),
                "",
                format!("{}", var_address).as_str(),
            ))
        }

        drop(manager);
        if let Some(value) = &self.value {
            let mut assign = Statement::VarAssign(
                Access::new(
                    Id::new(self.id.as_str(), Some(self.data_type.clone())),
                    vec![],
                ),
                value.to_owned(),
            );

            assign.generate();
        }
    }

    fn reduce(&self) -> String {
        todo!("reduce variable");
    }
}
