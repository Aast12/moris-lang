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

    // pub fn data_type(&self) -> DataType {
    //     match self.operator {
    //         Operator::Pipe => self.resolve_pipe_type(),
    //         _ => SemanticRules::match_type(
    //             self.operator,
    //             self.left.data_type(),
    //             self.right.data_type(),
    //         ),
    //     }
    // }

    // fn resolve_pipe_type(&self) -> DataType {
    //     let input_expr = self.left.to_owned();
    //     let piped_fn = self.right.to_owned();

    //     if let Expression::Access(access) = *piped_fn {
    //         match *input_expr {
    //             Expression::Op(op) => match op {
    //                 Operation {
    //                     operator: Operator::Pipe,
    //                     left: _,
    //                     right: _,
    //                 } => {
    //                     let call_param = op.resolve_pipe();
    //                     let call = Call::new(&access.id.id, vec![call_param]);
    //                     call.data_type()
    //                 }
    //                 _ => {
    //                     let call = Call::new(&access.id.id, vec![Box::new(Expression::Op(op))]);
    //                     call.data_type()
    //                 }
    //             },
    //             _ => panic!(),
    //         }
    //     } else {
    //         panic!()
    //     }
    // }

    // fn resolve_pipe(&self) -> Box<Expression> {
    //     let input_expr = self.left.to_owned();
    //     let piped_fn = self.right.to_owned();

    //     if let Expression::Access(access) = *piped_fn {
    //         match *input_expr {
    //             Expression::Op(op) => match op {
    //                 Operation {
    //                     operator: Operator::Pipe,
    //                     left: _,
    //                     right: _,
    //                 } => {
    //                     let call_param = op.resolve_pipe();
    //                     let call = Call::new(&access.id.id, vec![call_param]);
    //                     Box::new(Expression::Call(call))
    //                 }
    //                 _ => {
    //                     let call = Call::new(&access.id.id, vec![Box::new(Expression::Op(op))]);
    //                     Box::new(Expression::Call(call))
    //                 }
    //             },
    //             _ => panic!(),
    //         }
    //     } else {
    //         piped_fn
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::{constant::Const, id::Id};
    use memory::types::DataType;

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
