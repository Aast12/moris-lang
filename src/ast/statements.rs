use std::fmt::{Debug, Error, Formatter};

use crate::ast::expressions::{id::Access, Expression, Index};

use super::{
    node::Node,
    quadruples::Manager,
    types::{self, Function},
};

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
    fn set_manager(&mut self, manager: &'m Manager) -> () {
        match self {
            Statement::VarDeclaration(var) => var.set_manager(manager),
            Statement::VarAssign(access, value) => {
                access.set_manager(manager);
                value.set_manager(manager);
            }
            Statement::Expression(expr) => expr.set_manager(manager),
            Statement::If {
                condition,
                if_block,
                else_block,
            } => {
                condition.set_manager(manager);
                if_block.set_manager(manager);
                if let Some(block) = else_block {
                    block.set_manager(manager);
                }
            }
            Statement::For {
                iterator_id: _,
                iterable,
                block,
            } => {
                iterable.set_manager(manager);
                block.set_manager(manager);
            }
            Statement::While { condition, block } => {
                condition.set_manager(manager);
                block.set_manager(manager);
            }
            Statement::FunctionDeclaration(func) => func.set_manager(manager),
            Statement::Return(stmt) => stmt.set_manager(manager),
        }
    }

    fn generate(&mut self) -> () {
        match self {
            Statement::VarDeclaration(var) => var.generate(),
            Statement::VarAssign(access, value) => {
                access.generate();
                value.generate();
            }
            Statement::Expression(exp) => exp.generate(),
            Statement::If {
                condition,
                if_block,
                else_block,
            } => {
                todo!("For Statement generate");
                condition.generate();
                if_block.generate();
                if let Some(block) = else_block {
                    block.generate();
                }
            }
            Statement::For {
                iterator_id,
                iterable,
                block,
            } => {
                todo!("For Statement generate");
            }
            Statement::While { condition, block } => todo!("While Loop generate"),
            Statement::FunctionDeclaration(func) => func.generate(),
            Statement::Return(ret) => ret.generate(),
        }
    }

    fn reduce(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub struct Block<'m>(pub Vec<Statement<'m>>);

impl<'m> Node<'m> for Block<'m> {
    fn set_manager(&mut self, manager: &'m Manager) -> () {
        for stmt in self.0.iter_mut() {
            stmt.set_manager(manager);
        }
    }

    fn generate(&mut self) -> () {
        for stmt in self.0.iter_mut() {
            stmt.generate();
        }
    }
}

#[derive(Debug)]
pub struct Program<'m>(pub Vec<Statement<'m>>);

impl<'m> Node<'m> for Program<'m> {
    fn set_manager(&mut self, manager: &'m Manager) -> () {
        for stmt in self.0.iter_mut() {
            stmt.set_manager(manager);
        }
    }

    fn generate(&mut self) -> () {
        for stmt in self.0.iter_mut() {
            stmt.generate();
        }
    }
}
