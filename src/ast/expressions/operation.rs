use crate::{
    ast::node::Node,
    codegen::{manager::GlobalManager, quadruples::Quadruple},
    semantics::SemanticRules,
};

use super::{types, Expression};

#[derive(Debug, Clone)]
pub struct Operation {
    pub operator: types::Operator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl Operation {
    pub fn new(left: Box<Expression>, operator: types::Operator, right: Box<Expression>) -> Self {
        Operation {
            operator,
            left,
            right,
        }
    }

    pub fn data_type(&self) -> crate::memory::types::DataType {
        return SemanticRules::match_type(
            self.operator,
            self.left.data_type(),
            self.right.data_type(),
        );
    }
}

impl Node for Operation {
    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        let mut left = self.left.reduce();
        let mut right = self.right.reduce();

        let dt = self.data_type();

        if self.left.data_type() != dt {
            let new_left = GlobalManager::new_temp(&dt).to_string();
            GlobalManager::emit(Quadruple::type_cast(&dt, left.as_str(), new_left.as_str()));

            left = new_left
        }

        if self.right.data_type() != dt {
            let new_right = GlobalManager::new_temp(&dt).to_string();
            GlobalManager::emit(Quadruple::type_cast(
                &dt,
                right.as_str(),
                new_right.as_str(),
            ));

            right = new_right
        }

        let mut manager = GlobalManager::get();

        let tmp = manager.new_temp_address(&dt).to_string();

        manager.emit(Quadruple::operation(
            self.operator,
            left.as_str(),
            right.as_str(),
            tmp.as_str(),
        ));

        return tmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::expressions::{constant::Const, id::Id},
        memory::types::DataType,
    };

    #[test]
    fn test_operation() {
        let op = Operation::new(
            Box::new(Expression::Id(Id::new("left", None))),
            types::Operator::Add,
            Box::new(Expression::Const(Const::new("54.0", DataType::Float))),
        );

        if let Expression::Id(left) = op.left.as_ref() {
            assert_eq!(left.id, "left");
        } else {
            panic!()
        }

        if let Expression::Const(right) = op.right.as_ref() {
            assert_eq!(right.value, "54.0");
        } else {
            panic!()
        }
    }
}
