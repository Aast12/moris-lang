use std::fmt::Debug;

use crate::ast::expressions::{id::Access, Expression};

use super::{
    node::Node,
    types::{Function, Variable},
};

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
                condition: _,
                if_block: _,
                else_block: _,
            } => {
                todo!("For Statement generate");
                // condition.generate();
                // if_block.generate();
                // if let Some(block) = else_block {
                //     block.generate();
                // }
            }
            Statement::For {
                iterator_id: _,
                iterable: _,
                block: _,
            } => {
                todo!("For Statement generate");
            }
            Statement::While {
                condition: _,
                block: _,
            } => todo!("While Loop generate"),
            Statement::FunctionDeclaration(func) => func.generate(),
            Statement::Return(ret) => ret.generate(),
        }
    }

    fn reduce(&self) -> String {
        todo!()
    }
}

impl Debug for Statement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::VarDeclaration(var) => write!(fmt, "{:#?}", var),
            Statement::VarAssign(access, expr) => write!(fmt, "{:#?} = \n {:#?}", access, expr),
            Statement::Expression(expr) => write!(fmt, "{:#?}", expr),
            Statement::If {
                condition,
                if_block,
                else_block,
            } => {
                if let Some(block) = else_block {
                    write!(
                        fmt,
                        "IF ({:#?}) {{\n {:#?}\n}} ELSE {{\n {:#?} \n}}",
                        condition, if_block, block
                    )
                } else {
                    write!(fmt, "IF ({:#?}) {{\n {:#?}\n}}", condition, if_block)
                }
            }
            Statement::For {
                iterator_id,
                iterable,
                block,
            } => write!(
                fmt,
                "FOR ({:#?} IN {:#?}) {{\n {:#?}\n}}",
                iterator_id, iterable, block
            ),
            Statement::While { condition, block } => {
                write!(fmt, "WHILE ({:#?}) {{\n {:#?}\n}}", condition, block)
            }
            Statement::FunctionDeclaration(func) => write!(fmt, "{:#?}", func),
            Statement::Return(stmt) => write!(fmt, "RETURN {:#?}", stmt),
        }
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
