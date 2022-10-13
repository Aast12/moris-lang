use std::fmt::Debug;

use crate::ast::{
    expressions::{id::Access, Expression},
    types::{DataType, Operator},
};

use super::{
    node::Node,
    quadruples::{GlobalManager, Manager, Quadruple, QuadrupleHold, MANAGER},
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
                // TODO: Generalize for assign and var declaration

                let value_data_type = value.data_type();
                let access_data_type = access.data_type();
                assert!(
                    DataType::equivalent(&access.data_type(), &value_data_type).is_ok(),
                    "Data type {:?} cannot be assigned to a variable {:?}.",
                    value_data_type,
                    access_data_type
                );

                // Get temporal variable for assignment R-value
                let mut value_temp = value.reduce();

                if access_data_type != value_data_type {
                    // Emits type casting operation quadruple on r-value type mismatch
                    let mut manager = MANAGER.lock().unwrap();
                    let prev_value_temp = value_temp.clone();
                    value_temp = manager.new_temp(&access_data_type).reduce();

                    manager._emit(Quadruple(
                        String::from(format!("{:?}", access_data_type)),
                        prev_value_temp,
                        String::new(),
                        value_temp.clone(),
                    ))
                }

                let mut manager = MANAGER.lock().unwrap();
                manager._emit(Quadruple(
                    String::from(Operator::Assign.to_string()),
                    value_temp,
                    String::new(),
                    access.id.id.clone(),
                ));

                drop(manager);
            }
            Statement::Expression(exp) => exp.generate(),
            Statement::If {
                condition,
                if_block,
                else_block,
            } => {
                // TODO: move to an if struct
                // For If statements, the quadruples are generated as follows:
                //  1. [condition instructions]
                //  2. [goto if condition is false, jumps after 4.]
                //  3. [if-block instruction]
                //  4. [goto if condition was true, jumps after 5.]
                //  5. [else-block instruction]

                condition.generate();

                // goto instruction to skip if-true block
                let mut goto_if_false_quad = QuadrupleHold::new();

                if_block.generate();

                if let Some(block) = else_block {
                    // goto instruction to skip else block if condition was true
                    let mut goto_end_block = QuadrupleHold::new();

                    // Generate goto to skip to else block, if false
                    let goto_false_jump = GlobalManager::get_next_pos();
                    goto_if_false_quad.release(Quadruple::jump("gotoFalse", goto_false_jump));

                    block.generate();

                    // Update goto to skip else block
                    let end_pos = GlobalManager::get_next_pos();
                    goto_end_block.release(Quadruple::jump("goto", end_pos));
                } else {
                    // Update goto to skip if false
                    let end_pos = GlobalManager::get_next_pos();
                    goto_if_false_quad.release(Quadruple::jump("gotoFalse", end_pos));
                }
            }
            Statement::For {
                iterator_id: _,
                iterable: _,
                block: _,
            } => {
                todo!("For Statement generate");
            }
            Statement::While { condition, block } => {
                let start_pos = GlobalManager::get_next_pos();

                // Temporal storing condition value
                let condition_id = condition.reduce();

                // Goto instruction to exit the loop
                let mut goto_false_cond = QuadrupleHold::new();

                block.generate();

                // Emit instruction to return to condition evaluation
                GlobalManager::emit(Quadruple::new("goto", "", "", &start_pos.to_string()));

                let end_pos = GlobalManager::get_next_pos();

                // Emit instruction to return to condition evaluation
                goto_false_cond.release(Quadruple::new(
                    "gotoFalse",
                    &condition_id,
                    "",
                    &end_pos.to_string(),
                ));
            }
            Statement::FunctionDeclaration(func) => func.generate(),
            Statement::Return(ret) => ret.generate(),
        }
    }

    fn reduce(&self) -> String {
        todo!("reduce statement");
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
