use crate::{
    ast::{
        self,
        quadruples::{Quadruple, MANAGER},
        types::DataType,
    },
    semantics::SemanticRules,
};

use super::{types, Expression};

#[derive(Debug)]
pub struct Operation {
    pub operator: types::Operator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl<'m> Operation {
    pub fn new(left: Box<Expression>, operator: types::Operator, right: Box<Expression>) -> Self {
        Operation {
            operator,
            left,
            right,
        }
    }

    pub fn data_type(&self) -> DataType {
        return SemanticRules::match_type(
            self.operator,
            self.left.data_type(),
            self.right.data_type(),
        );
    }
}

impl<'m> ast::node::Node<'m> for Operation {
    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        let left = self.left.reduce();
        let right = self.right.reduce();

        let dt = self.data_type();
        let mut manager = MANAGER.lock().unwrap();

        let tmp = manager.new_temp(dt);

        manager.emit(Quadruple(
            String::from(self.operator.to_string()),
            left,
            right,
            tmp.reduce(),
        ));

        return tmp.reduce();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::{constant::Const, id::Id};

    #[test]
    fn test_operation() {
        let op = Operation::new(
            Box::new(Expression::Id(Id::new("left", None))),
            types::Operator::Add,
            Box::new(Expression::Const(Const::new(
                "54.0",
                types::DataType::Float,
            ))),
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
