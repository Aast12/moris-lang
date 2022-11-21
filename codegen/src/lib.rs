use env::SymbolEntry;
use manager::Manager;
use memory::{
    resolver::{MemAddress, MemoryResolver},
    types::DataType,
};
use natives::NativeFunctions;
use node::Node;
use parser::{
    expressions::{
        call::Call,
        constant::Const,
        id::{Access, Id},
        operation::Operation,
        Expression, Index,
    },
    functions::Function,
    semantics::{ExitStatement, SemanticRules},
    statements::{Block, Program, Statement},
    try_file,
    types::{Operator, OperatorType, Variable},
    Dimension,
};
use quadruples::{Quadruple, QuadrupleHold};
use std::iter::zip;
use std::{cmp::Ordering, path::PathBuf};
pub mod env;
pub mod function;
pub mod manager;
pub mod meta;
pub mod natives;
pub mod quadruples;

pub mod node;

pub fn generate(path: &PathBuf, manager: &mut Manager) {
    let native_functions = NativeFunctions::get_function_definitions();

    native_functions.iter().for_each(|func| {
        let return_address = match func.data_type {
            DataType::Void => None,
            _ => Some(manager.new_global(&func.data_type)),
        };

        manager.new_func(&func, 0, return_address, false);
    });

    let path = path.to_str().unwrap();
    let mut test_program = try_file(path);
    test_program.generate(manager);
}

impl Node for Variable {
    // TODO: Refactor to use Id
    fn address(&self, manager: &mut Manager) -> MemAddress {
        if let Some(var_entry) = manager.get_env().get_var(&self.id) {
            return var_entry.address;
        } else {
            panic!("Cannot find id {} in scope", self.id);
        }
    }

    fn generate(&mut self, manager: &mut Manager) -> () {
        // Add variable to symbols table
        let var_address =
            manager
                .get_env_mut()
                .add_var(&self.id, &self.data_type, &self.dimension, false);

        if self.dimension.size > 1 {
            let array_address = manager
                .get_env_mut()
                .allocate_array(&self.data_type, &self.dimension);

            manager.emit(Quadruple::operation(
                Operator::Assign,
                format!("&{}", array_address).as_str(),
                "",
                format!("{}", var_address).as_str(),
            ));

            manager.emit(Quadruple::operation(
                Operator::Assign,
                "END",
                "",
                format!("{}", array_address + self.dimension.size as MemAddress).as_str(),
            ));
        }

        if let Some(value) = &self.value {
            let mut assign = Statement::VarAssign(
                Access::new(
                    Id::new(self.id.as_str(), Some(self.data_type.clone())),
                    vec![],
                ),
                value.to_owned(),
            );

            assign.generate(manager);
        }
    }

    fn reduce(&self, _: &mut Manager) -> String {
        todo!("reduce variable");
    }
}

impl Node for Block {
    fn generate(&mut self, manager: &mut Manager) -> () {
        for stmt in self.0.iter_mut() {
            stmt.generate(manager);
        }
    }
}
impl Node for Program {
    fn generate(&mut self, manager: &mut Manager) -> () {
        let Program(statements) = self;
        statements.sort_by(|a, b| match a {
            Statement::FunctionDeclaration(_) => Ordering::Greater,
            _ => match b {
                Statement::FunctionDeclaration(_) => Ordering::Less,
                _ => Ordering::Equal,
            },
        });

        // Pre-declare function signatures
        for stmt in statements.iter_mut().rev() {
            match stmt {
                Statement::FunctionDeclaration(func) => {
                    let return_address = match func.signature.data_type {
                        DataType::Void => None,
                        _ => Some(manager.new_global(&func.signature.data_type)),
                    };

                    manager.new_func(&func.signature, 0, return_address, false);
                    // TODO: improve undefined location
                }
                _ => break,
            }
        }

        let mut last_func_generated = false;

        for stmt in statements.iter_mut() {
            if !last_func_generated {
                match stmt {
                    Statement::FunctionDeclaration(_) => {
                        last_func_generated = true;
                        manager.emit(Quadruple::end_program());
                    }
                    _ => (),
                }
            }

            stmt.generate(manager);
        }
    }
}

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

    fn reduce(&self, _: &mut Manager) -> String {
        todo!("reduce statement");
    }
}

impl Node for Function {
    fn generate(&mut self, manager: &mut Manager) -> () {
        let next_position = manager.get_next_pos();

        manager.update_func_position(&self.signature.id, next_position);
        manager.get_env_mut().switch(&self.signature.id);

        self.block.generate(manager);

        manager.emit(Quadruple::end_func());

        manager.get_env_mut().switch(&String::from("global"));
    }

    fn reduce(&self, _: &mut Manager) -> String {
        todo!("Function reduce!");
    }
}

impl Node for Expression {
    fn dimensionality(&self, manager: &mut Manager) -> Vec<usize> {
        match &self {
            Expression::Const(constant) => constant.dimensionality(manager),
            Expression::Op(operation) => operation.dimensionality(manager),
            Expression::Access(access) => access.dimensionality(manager),
            Expression::Id(id) => id.dimensionality(manager),
            Expression::Call(call) => call.dimensionality(manager),
            Expression::Not(expr) => expr.dimensionality(manager),
            Expression::Negative(expr) => expr.dimensionality(manager),
        }
    }
    fn data_type(&self, manager: &mut Manager) -> DataType {
        match &self {
            Expression::Const(constant) => constant.dtype.clone(),
            Expression::Op(operation) => operation.data_type(manager),
            Expression::Access(access) => access.id.data_type(manager),
            Expression::Id(id) => id.data_type(manager),
            Expression::Call(call) => call.data_type(manager),
            Expression::Not(expr) => expr.data_type(manager),
            Expression::Negative(expr) => expr.data_type(manager),
        }
    }

    fn generate(&mut self, manager: &mut Manager) -> () {
        match self {
            Expression::Const(constant) => constant.generate(manager),
            Expression::Op(operation) => operation.generate(manager),
            Expression::Access(access) => access.generate(manager),
            Expression::Id(id) => id.generate(manager),
            Expression::Call(call) => call.generate(manager),
            Expression::Not(not) => not.generate(manager),
            Expression::Negative(_) => todo!(),
        }
    }

    fn reduce(&self, manager: &mut Manager) -> String {
        match self {
            Expression::Const(constant) => constant.reduce(manager),
            Expression::Op(operation) => operation.reduce(manager),
            Expression::Access(access) => access.reduce(manager),
            Expression::Id(id) => id.reduce(manager),
            Expression::Call(call) => call.reduce(manager),
            Expression::Not(not) => {
                let mut to_negate = not.reduce(manager);
                let expr_type =
                    MemoryResolver::get_type_from_address(to_negate.parse().unwrap()).unwrap();

                if DataType::equivalent(expr_type, &DataType::Bool).is_err() {
                    panic!("Expression can't be casted to boolean");
                }
                if *expr_type != DataType::Bool {
                    to_negate = manager.emit_cast(&DataType::Bool, &to_negate);
                }

                let dest = manager.new_temp(&DataType::Bool).to_string();
                manager.emit(Quadruple::unary(
                    Operator::Not,
                    to_negate.as_str(),
                    dest.as_str(),
                ));

                dest
            }
            Expression::Negative(expr) => {
                let addr = expr.reduce(manager);
                let expr_dt = expr.data_type(manager);
                let new_addr = manager.new_temp(&expr_dt).to_string();

                manager.emit(Quadruple::unary(Operator::Neg, &addr, &new_addr));

                new_addr
            }
        }
    }
}

impl Node for Index {
    fn reduce(&self, manager: &mut Manager) -> String {
        match self {
            Self::Simple(idx) => idx.reduce(manager),
            Self::Range(_, _) => panic!("Range not supported"), // TODO
        }
    }
}

trait Pipe {
    fn resolve_pipe_type(&self, _: &mut Manager) -> DataType {
        todo!()
    }

    fn resolve_pipe(&self, _: &mut Manager) -> Box<Expression> {
        todo!()
    }
}

impl Pipe for Operation {
    fn resolve_pipe_type(&self, manager: &mut Manager) -> DataType {
        let input_expr = self.left.to_owned();
        let piped_fn = self.right.to_owned();

        if let Expression::Access(access) = *piped_fn {
            match *input_expr {
                Expression::Op(op) => match op {
                    Operation {
                        operator: Operator::Pipe,
                        left: _,
                        right: _,
                    } => {
                        let call_param = op.resolve_pipe(manager);
                        let call = Call::new(&access.id.id, vec![call_param]);
                        call.data_type(manager)
                    }
                    _ => {
                        let call = Call::new(&access.id.id, vec![Box::new(Expression::Op(op))]);
                        call.data_type(manager)
                    }
                },
                _ => panic!(),
            }
        } else {
            panic!()
        }
    }

    fn resolve_pipe(&self, manager: &mut Manager) -> Box<Expression> {
        let input_expr = self.left.to_owned();
        let piped_fn = self.right.to_owned();

        if let Expression::Access(access) = *piped_fn {
            match *input_expr {
                Expression::Op(op) => match op {
                    Operation {
                        operator: Operator::Pipe,
                        left: _,
                        right: _,
                    } => {
                        let call_param = op.resolve_pipe(manager);
                        let call = Call::new(&access.id.id, vec![call_param]);
                        Box::new(Expression::Call(call))
                    }
                    _ => {
                        let call = Call::new(&access.id.id, vec![Box::new(Expression::Op(op))]);
                        Box::new(Expression::Call(call))
                    }
                },
                _ => panic!(),
            }
        } else {
            piped_fn
        }
    }
}

impl Node for Operation {
    fn dimensionality(&self, manager: &mut Manager) -> Vec<usize> {
        if self.operator == Operator::Pipe {
            let new_tree = self.resolve_pipe(manager);
            new_tree.dimensionality(manager)
        } else {
            self.left.dimensionality(manager)
        }
    }

    fn data_type(&self, manager: &mut Manager) -> DataType {
        match self.operator {
            Operator::Pipe => self.resolve_pipe_type(manager),
            _ => SemanticRules::match_type(
                self.operator,
                self.left.data_type(manager),
                self.right.data_type(manager),
            ),
        }
    }

    fn generate(&mut self, manager: &mut Manager) -> () {
        self.reduce(manager);
    }

    fn reduce(&self, manager: &mut Manager) -> String {
        if self.operator == Operator::Pipe {
            let new_tree = self.resolve_pipe(manager);
            return new_tree.reduce(manager);
        }
        let left_dims = self.left.dimensionality(manager);
        let right_dims = self.right.dimensionality(manager);
        if left_dims != right_dims {
            panic!(
                "Can't operate items with dimensions {:#?} and {:#?}",
                left_dims, right_dims
            )
        }

        let mut left = self.left.reduce(manager);
        let left_dt = self.left.data_type(manager);
        let mut right = self.right.reduce(manager);
        let right_dt = self.right.data_type(manager);

        let dt = self.data_type(manager);

        match dt {
            DataType::Int | DataType::Float | DataType::String | DataType::Pointer => {
                if left_dt != dt {
                    let new_left = manager.new_temp(&dt).to_string();
                    manager.emit(Quadruple::type_cast(&dt, left.as_str(), new_left.as_str()));

                    left = new_left
                }

                if right_dt != dt {
                    let new_right = manager.new_temp(&dt).to_string();
                    manager.emit(Quadruple::type_cast(
                        &dt,
                        right.as_str(),
                        new_right.as_str(),
                    ));

                    right = new_right
                }
            }
            DataType::Bool => {
                if self.operator.is_arithmetic() {
                    panic!()
                }

                // TODO: Type casting compatibility validations
                match self.operator.which() {
                    OperatorType::Boolean => {
                        if left_dt != DataType::Bool {
                            left = manager.emit_cast(&DataType::Bool, left.as_str());
                        }
                        if right_dt != DataType::Bool {
                            right = manager.emit_cast(&DataType::Bool, left.as_str());
                        }
                    }
                    OperatorType::Comparison => {
                        if left_dt != right_dt {
                            let max_dt = DataType::max(&left_dt, &right_dt);
                            if max_dt != left_dt {
                                left = manager.emit_cast(&max_dt, left.as_str());
                            } else {
                                right = manager.emit_cast(&max_dt, right.as_str());
                            }
                        }
                    }
                    OperatorType::Arithmetic => todo!(),
                    OperatorType::Pipe | OperatorType::Assign => todo!(),
                    OperatorType::Neg => todo!(),
                }
            }
            _ => (),
        }

        let tmp = manager.new_temp(&dt).to_string();

        manager.emit(Quadruple::operation(
            self.operator,
            left.as_str(),
            right.as_str(),
            tmp.as_str(),
        ));

        return tmp;
    }
}

impl Node for Const {
    fn generate(&mut self, manager: &mut Manager) -> () {
        self.reduce(manager);
    }

    fn reduce(&self, manager: &mut Manager) -> String {
        let const_address = manager.new_constant(&self.dtype, self);
        return const_address.to_string();
    }
}

impl Node for Id {
    fn dimensionality(&self, manager: &mut Manager) -> Vec<usize> {
        if let Some(id) = manager.get_env_mut().get_var(&self.id) {
            return id.dimension.shape.clone();
        }
        panic!("id {} is not defined", self.id);
    }

    fn reduce(&self, manager: &mut Manager) -> String {
        self.address(manager).to_string()
    }

    fn data_type(&self, manager: &mut Manager) -> DataType {
        match &self.dtype {
            Some(dtype) => dtype.clone(),
            _ => {
                if let Some(id) = manager.get_env_mut().get_var(&self.id) {
                    return id.data_type.clone();
                }
                panic!("id {} is not defined", self.id);
            }
        }
    }

    fn address(&self, manager: &mut Manager) -> MemAddress {
        if let Some(var_entry) = manager.get_env_mut().get_var(&self.id) {
            return var_entry.address;
        } else {
            panic!("Cannot find id {} in scope", self.id);
        }
    }
}

trait ImmutableVar {
    fn is_immutable(&self, _: &mut Manager) -> bool {
        todo!()
    }
}

impl ImmutableVar for Access {
    fn is_immutable(&self, manager: &mut Manager) -> bool {
        let id_var = manager.get_env_mut().get_var(&self.id.id).cloned();

        if let Some(id_var) = id_var {
            id_var.immutable
        } else {
            panic!("Variable {} is not defined", self.id.id);
        }
    }
}

impl Node for Access {
    fn dimensionality(&self, manager: &mut Manager) -> Vec<usize> {
        let to_access_shape = self.id.dimensionality(manager);
        let to_access_dims = to_access_shape.len();
        let indexing_dims = self.indexing.len();

        if indexing_dims > to_access_dims {
            panic!();
        }

        let slice_dims = to_access_dims - indexing_dims;

        if indexing_dims >= to_access_dims {
            vec![]
        } else {
            to_access_shape[(to_access_dims - slice_dims as usize)..]
                .to_vec()
                .clone()
        }
    }

    fn data_type(&self, manager: &mut Manager) -> DataType {
        return self.id.data_type(manager);
    }

    fn generate(&mut self, manager: &mut Manager) -> () {
        self.reduce(manager);
    }

    fn reduce(&self, manager: &mut Manager) -> String {
        let access_item: SymbolEntry;
        let id_var = manager.get_env_mut().get_var(&self.id.id).cloned();
        if let Some(entry) = id_var {
            access_item = entry;
        } else {
            panic!("Item {} does not exist!", self.id.id);
        }

        if self.indexing.len() == 0 {
            return self.id.address(manager).to_string();
        } else if access_item.dimension.size == 1 {
            panic!("Can't index scalar value {}", self.id.id);
        }

        // Address to values used to index the array
        let indexing_addresses = self
            .indexing
            .iter()
            .map(|index| index.reduce(manager))
            .collect::<Vec<String>>();

        if indexing_addresses.len() > access_item.dimension.dimensions as usize {
            panic!("Incompatible index!");
        }

        let shape_cp = access_item.dimension.shape.clone();
        let mut array_shape = shape_cp.iter();
        let acc_tmp = manager.new_temp(&DataType::Pointer).to_string();
        let mut first_run = true;

        zip(&indexing_addresses, &access_item.dimension.acc_size).for_each(|(index, dim_size)| {
            if let Some(dim) = array_shape.next() {
                manager.emit(Quadruple::verify(index.as_str(), dim.to_string().as_str()))
            }

            let dim_const = manager.new_constant(
                &DataType::Pointer,
                &Const::new(dim_size.to_string().as_str(), DataType::Pointer),
            );
            let dim_const = dim_const.to_string();

            if first_run {
                manager.emit(Quadruple::operation(
                    Operator::Mul,
                    index.as_str(),
                    dim_const.as_str(),
                    acc_tmp.as_str(),
                ));
                first_run = false;
            } else {
                let tmp = manager.new_temp(&DataType::Pointer);
                let tmp_str = tmp.to_string();

                manager.emit(Quadruple::operation(
                    Operator::Mul,
                    index.as_str(),
                    dim_const.as_str(),
                    tmp_str.as_str(),
                ));

                manager.emit(Quadruple::operation(
                    Operator::Add,
                    acc_tmp.as_str(),
                    tmp_str.as_str(),
                    acc_tmp.as_str(),
                ));
            }
        });

        let access_tmp = manager.new_temp(&DataType::Pointer);

        manager.emit(Quadruple::operation(
            Operator::Add,
            access_item.address.to_string().as_str(),
            acc_tmp.as_str(),
            access_tmp.to_string().as_str(),
        ));

        if self.indexing.len() == access_item.dimension.dimensions as usize {
            format!("*{}", access_tmp)
        } else {
            format!("{}", access_tmp)
        }
    }
}

impl Node for Call {
    fn data_type(&self, manager: &mut Manager) -> DataType {
        if let Some(data_type) = NativeFunctions::data_type(&self.id, manager) {
            data_type
        } else {
            manager.get_func(&self.id).return_type.clone()
        }
    }

    fn generate(&mut self, manager: &mut Manager) -> () {
        self.reduce(manager);
    }

    fn reduce(&self, manager: &mut Manager) -> String {
        if let Some(return_value) = NativeFunctions::call_reduce(self, manager) {
            return return_value;
        }

        let func = manager.get_func(&self.id).clone();
        let return_type = func.return_type.clone();
        let param_defintions = func.params.clone();

        let target_params_len = param_defintions.len();
        if self.params.len() != target_params_len {
            panic!(
                "Params size do not match {} {} - {}",
                self.id,
                self.params.len(),
                target_params_len
            );
        }

        manager.emit(Quadruple::era(self.id.as_str()));

        for (index, param) in self.params.iter().enumerate() {
            let (_, def_param_data_type, _) = param_defintions.get(index).unwrap();

            if def_param_data_type == &DataType::Pointer && param.dimensionality(manager).len() > 0
            {
                let param_address = param.reduce(manager);
                manager.emit(Quadruple::param(param_address.as_str(), index));
                continue;
            }

            let mut param_address = param.reduce(manager);
            let param_data_type = param.data_type(manager);

            assert!(
                DataType::equivalent(&param_data_type, def_param_data_type).is_ok(),
                "Data type {:?} cannot be assigned to a variable {:?}.",
                param_data_type,
                def_param_data_type
            );

            // TODO: Refactor type casting instruction into func
            if param_data_type != *def_param_data_type {
                let value_temp = manager.new_temp(&def_param_data_type).to_string();

                manager.emit(Quadruple::type_cast(
                    &def_param_data_type,
                    param_address.as_str(),
                    value_temp.as_str(),
                ));

                param_address = value_temp.clone();
            }

            manager.emit(Quadruple::param(param_address.as_str(), index));
        }

        manager.emit(Quadruple::go_sub(self.id.as_str()));

        if let Some(address) = manager.get_func_return(&self.id) {
            let return_value = manager.new_temp(&return_type).to_string();
            manager.emit(Quadruple::unary(
                Operator::Assign,
                address.to_string().as_str(),
                return_value.as_str(),
            ));
            return_value
        } else {
            String::from("VOID")
        }
    }
}
