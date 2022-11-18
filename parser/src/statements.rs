use std::fmt::Debug;

use crate::{
    expressions::{id::Access, Expression},
    functions::Function,
    types::Variable,
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
    VoidReturn,
    Break,
    Continue,
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
            Statement::Break => write!(fmt, "Break"),
            Statement::Continue => write!(fmt, "Continue"),
            Statement::VoidReturn => write!(fmt, "VoidReturn"),
        }
    }
}

#[derive(Debug)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug)]
pub struct Program(pub Vec<Statement>);
