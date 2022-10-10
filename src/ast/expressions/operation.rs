use crate::ast;

use super::{types, Expression};

pub struct Operation<'m, L: Expression<'m>, R: Expression<'m>> {
    manager: Option<&'m ast::quadruples::Manager>,
    pub operator: types::Operator,
    pub left: Box<L>,
    pub right: Box<R>,
}

impl<'m, L: Expression<'m>, R: Expression<'m>> Operation<'m, L, R> {
    pub fn new(left: Box<L>, operator: types::Operator, right: Box<R>) -> Self {
        Operation {
            manager: None,
            operator,
            left,
            right,
        }
    }
}

impl<'m, L: Expression<'m>, R: Expression<'m>> ast::node::Node<'m> for Operation<'m, L, R> {
    fn set_manager(&mut self, manager: &'m ast::quadruples::Manager) -> () {
        self.manager = Some(manager);
        self.left.set_manager(manager);
        self.right.set_manager(manager);
    }

    fn generate(&self) -> () {
        todo!()
    }

    fn reduce(&self) -> &dyn ast::node::Leaf {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::node::Node;
    use crate::ast::quadruples::Manager;
    use crate::ast::types::DataType;

    #[test]
    fn test_operation() {
        let m = Manager::new();
        
    }
}
