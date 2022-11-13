use crate::{
    codegen::{manager::GlobalManager, quadruples::Quadruple},
    memory::{resolver::MemAddress, types::DataType},
};

use super::{expressions::Expression, node::Node, Dimension};

#[derive(Clone, Copy, Debug)]
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
        if let Some(var_entry) = GlobalManager::get().get_env().get_var(&self.id) {
            return var_entry.address;
        } else {
            panic!("Cannot find id {} in scope", self.id);
        }
    }
}

impl Node for Variable {
    fn generate(&mut self) -> () {
        // Add variable to symbols table
        let mut manager = GlobalManager::get();
        manager.get_env().add_var(&self.id, &self.data_type, &self.dimension);
        drop(manager);

        let self_address = self.address();

        if let Some(value) = &self.value {
            let value_data_type = value.data_type();
            // TODO: Refactor to use VarAssign
            assert!(
                DataType::equivalent(&self.data_type, &value_data_type).is_ok(),
                "Data type {:?} cannot be assigned to a variable {:?}.",
                value_data_type,
                self.data_type
            );

            // Get temporal variable for assignment R-value
            let mut value_temp = value.reduce();
            manager = GlobalManager::get();

            if self.data_type != value_data_type {
                // Emits type casting operation quadruple on r-value type mismatch
                let prev_value_temp = value_temp.clone();
                value_temp = manager.new_temp_address(&self.data_type).to_string();

                manager.emit(Quadruple(
                    String::from(format!("{:?}", self.data_type)),
                    prev_value_temp,
                    String::new(),
                    value_temp.clone(),
                ))
            }

            manager.emit(Quadruple(
                String::from(Operator::Assign.to_string()),
                value_temp,
                String::new(),
                self_address.to_string(),
            ));

            drop(manager);
        }
    }

    fn reduce(&self) -> String {
        todo!("reduce variable");
    }
}
