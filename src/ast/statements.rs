use std::fmt::{Debug, Error, Formatter};

use crate::ast::expressions::{id::Access, Expression, Index};

use super::{types::{self, Function}};

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

#[derive(Debug)]
pub struct Block<'m>(pub Vec<Statement<'m>>);

#[derive(Debug)]
pub struct Program<'m>(pub Vec<Statement<'m>>);
