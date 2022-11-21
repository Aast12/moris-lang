use memory::types::DataType;
use parser::{
    expressions::constant::Const, semantics::ExitStatement, statements::Statement, types::Operator,
    Dimension,
};

use crate::{
    ast_nodes::expressions::ImmutableVar,
    manager::Manager,
    node::{ExpressionNode, Node},
    quadruples::{Quadruple, QuadrupleHold},
};

impl Node for Statement {
    fn generate(&mut self, manager: &mut Manager) -> () {
        match self {
            Statement::VarDeclaration(var) => var.generate(manager),
            Statement::VarAssign(access, value) => {
                // TODO: Generalize for assign and var declaration
                // TODO: Generalize data type casting
                let value_data_type = value.data_type(manager);
                let access_data_type = access.data_type(manager);

                let access_dims = access.dimensionality(manager);
                let value_dims = value.dimensionality(manager);
                if access_dims != value_dims {
                    panic!(
                        "Can't assign item {} with dimensions {:#?} to value with dimensions {:#?}",
                        access.id.id, access_dims, value_dims
                    )
                }
                assert!(
                    DataType::equivalent(&access.data_type(manager), &value_data_type).is_ok(),
                    "Data type {:?} cannot be assigned to a variable {:?}.",
                    value_data_type,
                    access_data_type
                );

                // Get temporal variable for assignment R-value
                let mut value_temp = value.reduce(manager);

                if access_data_type != value_data_type {
                    // Emits type casting operation quadruple on r-value type mismatch
                    let prev_value_temp = value_temp.clone();
                    value_temp = manager.new_temp(&access_data_type).to_string();

                    manager.emit(Quadruple::type_cast(
                        &access_data_type,
                        prev_value_temp.as_str(),
                        value_temp.clone().as_str(),
                    ))
                }

                if access.is_immutable(manager) {
                    panic!("Variable {} can't be mutated", access.id.id);
                }

                let access = access.reduce(manager);

                manager.emit(Quadruple::unary(
                    Operator::Assign,
                    value_temp.as_str(),
                    access.as_str(),
                ));
            }
            Statement::Expression(exp) => exp.generate(manager),
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

                let mut condition_id = condition.reduce(manager);
                let condition_dt = condition.data_type(manager);
                if condition_dt != DataType::Bool {
                    condition_id = manager.emit_cast(&DataType::Bool, condition_id.as_str());
                }

                // goto instruction to skip if-true block
                let mut goto_if_false_quad = QuadrupleHold::new(manager);

                if_block.generate(manager);

                if let Some(block) = else_block {
                    // goto instruction to skip else block if condition was true
                    let mut goto_end_block = QuadrupleHold::new(manager);

                    // Generate goto to skip to else block, if false
                    let goto_false_jump = manager.get_next_pos();
                    goto_if_false_quad.release(
                        manager,
                        Quadruple::goto_false(&condition_id, goto_false_jump),
                    );

                    block.generate(manager);

                    // Update goto to skip else block
                    let end_pos = manager.get_next_pos();
                    goto_end_block.release(manager, Quadruple::goto(end_pos));
                } else {
                    // Update goto to skip if false
                    let end_pos = manager.get_next_pos();
                    goto_if_false_quad
                        .release(manager, Quadruple::goto_false(&condition_id, end_pos));
                }
            }
            Statement::For {
                iterator_id,
                range,
                block,
            } => {
                let (start, end, step) = range;
                let start_dt = start.data_type(manager);
                let end_dt = start.data_type(manager);
                let step_dt = if let Some(step) = step {
                    Some(step.data_type(manager))
                } else {
                    None
                };

                if DataType::equivalent(&start_dt, &DataType::Int).is_err() {
                    panic!("For start expression is not numeric");
                }
                if DataType::equivalent(&end_dt, &DataType::Int).is_err() {
                    panic!("For end expression is not numeric");
                }
                if let Some(step_dt) = step_dt {
                    if DataType::equivalent(&step_dt, &DataType::Int).is_err() {
                        panic!("For step expression is not numeric");
                    }
                }

                let start = start.reduce(manager);
                let end = end.reduce(manager);
                let step = if let Some(step) = step {
                    Some(step.reduce(manager))
                } else {
                    None
                };

                let iterator_address = manager
                    .get_env_mut()
                    .add_var(iterator_id, &DataType::Int, &Dimension::new_scalar(), true)
                    .to_string();

                manager.emit(Quadruple::unary(
                    Operator::Assign,
                    start.as_str(),
                    iterator_address.as_str(),
                ));

                let check_tmp = manager.new_temp(&DataType::Bool).to_string();

                let return_position = manager.get_next_pos();

                manager.emit(Quadruple::operation(
                    Operator::LessThan,
                    iterator_address.as_str(),
                    end.as_str(),
                    check_tmp.as_str(),
                ));

                let mut goto_false_hold = QuadrupleHold::new(manager);

                block.generate(manager);

                if let Some(step) = step {
                    manager.emit(Quadruple::operation(
                        Operator::Add,
                        iterator_address.as_str(),
                        step.as_str(),
                        iterator_address.as_str(),
                    ));
                } else {
                    let increment_tmp = manager
                        .new_constant(&DataType::Int, &Const::new("1", DataType::Int))
                        .to_string();

                    manager.emit(Quadruple::operation(
                        Operator::Add,
                        iterator_address.as_str(),
                        increment_tmp.as_str(),
                        iterator_address.as_str(),
                    ));
                }

                let to_start_pos_quadruple = Quadruple::goto(return_position);
                manager.emit(Quadruple::goto(return_position.clone()));

                let end_pos = manager.get_next_pos();

                let to_end_pos_quadruple = Quadruple::goto_false(check_tmp.as_str(), end_pos);
                goto_false_hold.release(manager, to_end_pos_quadruple.clone());

                manager.resolve_context(&ExitStatement::Continue, to_start_pos_quadruple);
                manager.resolve_context(&ExitStatement::Break, to_end_pos_quadruple);

                manager.get_env_mut().del_var(iterator_id);
                manager.emit(Quadruple::free(&iterator_address.as_str()));
            }
            Statement::While { condition, block } => {
                let start_pos = manager.get_next_pos();

                // Temporal storing condition value
                let mut condition_id = condition.reduce(manager);
                let condition_dt = condition.data_type(manager);
                if condition_dt != DataType::Bool {
                    condition_id = manager.emit_cast(&DataType::Bool, condition_id.as_str());
                }

                // Goto instruction to exit the loop
                let mut goto_false_cond = QuadrupleHold::new(manager);

                block.generate(manager);

                // Emit instruction to return to condition evaluation
                let to_start_pos_quadruple = Quadruple::goto(start_pos);
                manager.emit(to_start_pos_quadruple.clone());

                let end_pos = manager.get_next_pos();

                let to_end_pos_quadruple = Quadruple::goto(end_pos);

                // Emit instruction to return to condition evaluation
                goto_false_cond.release(manager, Quadruple::goto_false(&condition_id, end_pos));

                manager.resolve_context(&ExitStatement::Continue, to_start_pos_quadruple);
                manager.resolve_context(&ExitStatement::Break, to_end_pos_quadruple);
            }
            Statement::FunctionDeclaration(func) => func.generate(manager),
            Statement::Return(ret) => {
                let mut return_item = ret.reduce(manager);
                let context = manager.get_env().current_env();
                assert_eq!(context.is_global, false);
                let return_type = context.return_type.clone().unwrap();
                if return_type != ret.data_type(manager) {
                    return_item = manager.emit_cast(&return_type, return_item.as_str());
                }

                manager.emit(Quadruple::new_return(return_item.as_str()));
            }
            Statement::Break => manager.prepare_exit_stmt(&ExitStatement::Break),
            Statement::Continue => manager.prepare_exit_stmt(&ExitStatement::Continue),
            Statement::VoidReturn => manager.emit(Quadruple::void_return()),
        }
    }
}
