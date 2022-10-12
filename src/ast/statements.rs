use std::fmt::{Debug};

use crate::ast::expressions::{id::Access, Expression};

use super::{
    node::Node,
    types::{Function, Variable},
};

#[derive(Debug)]
pub enum Statement {
    VarDeclaration(Variable),
    VarAssign(Access, Box<Expression>),
    Expression(Box<Expression>),
    If {
        condition: Box<Expression>,
        if_block: Block,
        else_block: Option<Block>,
    },
    For {
        iterator_id: String,
        iterable: Box<Expression>,
        block: Block,
    },
    While {
        condition: Box<Expression>,
        block: Block,
    },
    FunctionDeclaration(Function),
    Return(Box<Expression>),
}

impl<'m> Node<'m> for Statement {
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
pub struct Block(pub Vec<Statement>);

impl<'m> Node<'m> for Block {
    fn generate(&mut self) -> () {
        for stmt in self.0.iter_mut() {
            stmt.generate();
        }
    }
}

#[derive(Debug)]
pub struct Program(pub Vec<Statement>);

impl<'m> Node<'m> for Program {
    fn generate(&mut self) -> () {
        for stmt in self.0.iter_mut() {
            stmt.generate();
        }
    }
}
