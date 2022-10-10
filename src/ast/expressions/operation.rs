use crate::{
    ast::{self, types::DataType},
    semantics::SemanticRules,
};

use super::{types, Expression, ExpressionT};

#[derive(Debug)]
pub struct Operation<'m> {
    manager: Option<&'m ast::quadruples::Manager<'m>>,
    pub operator: types::Operator,
    pub left: Box<Expression<'m>>,
    pub right: Box<Expression<'m>>,
}

impl<'m> Operation<'m> {
    pub fn new(
        left: Box<Expression<'m>>,
        operator: types::Operator,
        right: Box<Expression<'m>>,
    ) -> Self {
        Operation {
            manager: None,
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

impl<'m> ast::node::Node<'m> for Operation<'m> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager<'m>) -> () {
        self.manager = Some(manager);
        self.left.set_manager(manager);
        self.right.set_manager(manager);
    }

    fn generate(&mut self) -> () {
        
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        todo!()
    }
}

impl<'m> ExpressionT<'m> for Operation<'m> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        expressions::{constant::Const, id::Id},
        node::Node,
        quadruples::Manager,
    };

    #[test]
    fn test_operation() {
        let manager = Manager::new();

        let mut op = Operation::new(
            Box::new(Expression::Id(Id::new("left", None))),
            types::Operator::Add,
            Box::new(Expression::Const(Const::new(
                "54.0",
                types::DataType::Float,
            ))),
        );

        op.set_manager(&manager);

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
