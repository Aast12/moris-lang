use crate::types::Operator;

use super::Expression;

#[derive(Debug, Clone)]
pub struct Operation {
    pub operator: Operator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl Operation {
    pub fn new(left: Box<Expression>, operator: Operator, right: Box<Expression>) -> Self {
        Operation {
            operator,
            left,
            right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::{constant::Const, id::Id};
    use memory::types::DataType;

    #[test]
    fn test_operation() {
        let op = Operation::new(
            Box::new(Expression::Id(Id::new("left", None))),
            Operator::Add,
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
