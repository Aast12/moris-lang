use std::fmt::Debug;

use crate::ast::{
    expressions::{id::Access, Expression},
    types::{DataType, Operator},
};

use super::{
    node::Node,
    quadruples::{Quadruple, MANAGER},
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

                println!("Assigning var value");

                // Get temporal variable for assignment R-value
                let mut value_temp = value.reduce();

                if access_data_type != value_data_type {
                    // Emits type casting operation quadruple on r-value type mismatch
                    println!("To emit type cast");
                    let mut manager = MANAGER.lock().unwrap();
                    let prev_value_temp = value_temp.clone();
                    value_temp = manager.new_temp(&access_data_type).reduce();

                    manager.emit(Quadruple(
                        String::from(format!("{:?}", access_data_type)),
                        prev_value_temp,
                        String::new(),
                        value_temp.clone(),
                    ))
                }

                {
                    println!("To emit assign");
                    MANAGER.lock().unwrap().emit(Quadruple(
                        String::from(Operator::Assign.to_string()),
                        value_temp,
                        String::new(),
                        access.id.id.clone(),
                    ));
                }
            }
            Statement::Expression(exp) => exp.generate(),
            Statement::If {
                condition,
                if_block,
                else_block,
            } => {
                condition.generate();

                let goto_false_id: usize; // Id of if-false goto quadruple
                let end_id: usize; // Id of goto end to skip else block
                {
                    println!("To emit goto False");
                    // Generate goto if false
                    let mut manager = MANAGER.lock().unwrap();
                    goto_false_id = manager.get_next_id();
                    manager.emit(Quadruple::new_empty());
                }

                if_block.generate();

                if let Some(block) = else_block {
                    println!("To emit ekse");
                    let goto_end_id: usize;
                    {
                        let mut manager = MANAGER.lock().unwrap();

                        let goto_end_quad = Quadruple::new_empty();
                        goto_end_id = manager.get_next_id();
                        manager.emit(goto_end_quad);

                        let goto_false_jump = manager.get_next_id();
                        manager.update_instruction(
                            goto_false_id,
                            Quadruple::new(
                                "gotoFalse",
                                "",
                                "",
                                goto_false_jump.to_string().as_str(),
                            ),
                        );
                    }

                    block.generate();

                    {
                        let mut manager = MANAGER.lock().unwrap();

                        end_id = manager.get_next_id();

                        manager.update_instruction(
                            goto_end_id,
                            Quadruple::new("goto", "", "", end_id.to_string().as_str()),
                        );
                    }
                } else {
                    println!("To emit end");
                    let mut manager = MANAGER.lock().unwrap();
                    end_id = manager.get_next_id();
                    manager.update_instruction(
                        goto_false_id,
                        Quadruple::new("gotoFalse", "", "", end_id.to_string().as_str()),
                    );
                }
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
