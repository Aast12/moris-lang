use super::{
    expressions::Expression,
    node::Node,
    quadruples::{Quadruple, GlobalManager},
    statements::Block,
    Dimension,
};

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
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug)]
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
}

impl<'m> Node<'m> for Variable {
    fn generate(&mut self) -> () {
        // Add variable to symbols table
        let mut manager = GlobalManager::get();
        manager.get_env().add_var(&self.id, &self.data_type);
        drop(manager);

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
                value_temp = manager.new_temp(&self.data_type).reduce();

                manager._emit(Quadruple(
                    String::from(format!("{:?}", self.data_type)),
                    prev_value_temp,
                    String::new(),
                    value_temp.clone(),
                ))
            }

            manager._emit(Quadruple(
                String::from(Operator::Assign.to_string()),
                value_temp,
                String::new(),
                self.id.clone(),
            ));

            drop(manager);
        }
    }

    fn reduce(&self) -> String {
        todo!("reduce variable");
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub id: String,
    pub data_type: DataType,
    pub params: Vec<FunctionParam>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParam(pub String, pub DataType);

#[derive(Debug)]
pub struct Function {
    pub signature: FunctionSignature,
    pub block: Block,
}

impl<'m> Function {
    pub fn new(signature: FunctionSignature, block: Block) -> Function {
        Function { signature, block }
    }
}

impl<'m> Node<'m> for Function {
    fn generate(&mut self) -> () {
        let mut manager = GlobalManager::get();

        manager
            .get_env()
            .from_function(&self.signature.id, self.signature.clone(), true);
        
        drop(manager);

        self.block.generate();

        GlobalManager::get().env.switch(&String::from("global"));
    }

    fn reduce(&self) -> String {
        todo!("Function reduce!");
    }
}
