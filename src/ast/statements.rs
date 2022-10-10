use std::fmt::{Debug, Error, Formatter};

use crate::ast::expressions::{id::Access, Expression, Index};

use super::{types::{self, Function}, node::Node};

#[derive(Debug)]
pub enum Statement<'m> {
    VarDeclaration(types::Variable<'m>),
    VarAssign(Access<'m>, Box<Expression<'m>>),
    Expression(Box<Expression<'m>>),
    If {
        condition: Box<Expression<'m>>,
        if_block: Block<'m>,
        else_block: Option<Block<'m>>,
    },
    For {
        iterator_id: String,
        iterable: Box<Expression<'m>>,
        block: Block<'m>,
    },
    While {
        condition: Box<Expression<'m>>,
        block: Block<'m>,
    },
    FunctionDeclaration(Function<'m>),
    Return(Box<Expression<'m>>),
}

impl<'m> Node<'m> for Statement<'m> {
    fn set_manager(&mut self, manager: &'m super::quadruples::Manager) -> () {
        match self {
            Statement::VarDeclaration(var) => var.set_manager(manager),
            Statement::VarAssign(access, value) => {
                access.set_manager(manager);
                value.set_manager(manager);
            },
            Statement::Expression(expr) => expr.set_manager(manager),
            Statement::If { condition, if_block, else_block } => {
                condition.set_manager(manager);
                if_block.set_manager(manager);
                if let Some(block) = else_block {
                    block.set_manager(manager);
                }    
            },
            Statement::For { iterator_id, iterable, block } => {
                iterable.set_manager(manager);
                block.set_manager(manager);
            },
            Statement::While { condition, block } => {
                condition.set_manager(manager);
                block.set_manager(manager);
            },
            Statement::FunctionDeclaration(func) => func.block.set_manager(manager),
            Statement::Return(stmt) => stmt.set_manager(manager),
        }
    }

    fn generate(&mut self) -> () {
        todo!()
    }

    fn reduce(&self) -> &dyn super::node::Leaf {
        todo!()
    }
}

#[derive(Debug)]
pub struct Block<'m>(pub Vec<Statement<'m>>);

impl<'m> Node<'m> for Block<'m> {
    fn set_manager(&mut self, manager: &'m super::quadruples::Manager) -> () {
        for stmt in self.0.iter_mut() {
            stmt.set_manager(manager);
        }
    }
}

#[derive(Debug)]
pub struct Program<'m>(pub Vec<Statement<'m>>);

impl<'m> Node<'m> for Program<'m> {
    fn set_manager(&mut self, manager: &'m super::quadruples::Manager) -> () {
        for stmt in self.0.iter_mut() {
            stmt.set_manager(manager);
        }
    }
}
