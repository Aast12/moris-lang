use env::SymbolEntry;
use manager::GlobalManager;
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

pub fn generate(path: &PathBuf) {
    let native_functions = NativeFunctions::get_function_definitions();

    let mut manager = GlobalManager::get();

    native_functions.iter().for_each(|func| {
        let return_address = match func.data_type {
            DataType::Void => None,
            _ => Some(manager.new_global(&func.data_type)),
        };

        manager.new_func(&func, 0, return_address, false);
    });

    drop(manager);

    let path = path.to_str().unwrap();
    let mut test_program = try_file(path);
    test_program.generate();
}

impl Node for Variable {
    // TODO: Refactor to use Id
    fn address(&self) -> MemAddress {
        if let Some(var_entry) = GlobalManager::get().get_env_mut().get_var(&self.id) {
            return var_entry.address;
        } else {
            panic!("Cannot find id {} in scope", self.id);
        }
    }
    fn generate(&mut self) -> () {
        // Add variable to symbols table
        let mut manager = GlobalManager::get();
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

        drop(manager);

        if let Some(value) = &self.value {
            let mut assign = Statement::VarAssign(
                Access::new(
                    Id::new(self.id.as_str(), Some(self.data_type.clone())),
                    vec![],
                ),
                value.to_owned(),
            );

            assign.generate();
        }
    }

    fn reduce(&self) -> String {
        todo!("reduce variable");
    }
}

impl Node for Block {
    fn generate(&mut self) -> () {
        for stmt in self.0.iter_mut() {
            stmt.generate();
        }
    }
}
impl Node for Program {
    fn generate(&mut self) -> () {
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
                    let mut manager = GlobalManager::get();

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
                        GlobalManager::emit(Quadruple::end_program());
                    }
                    _ => (),
                }
            }

            stmt.generate();
        }
    }
}

impl Node for Statement {
    fn generate(&mut self) -> () {
        match self {
            Statement::VarDeclaration(var) => var.generate(),
            Statement::VarAssign(access, value) => {
                // TODO: Generalize for assign and var declaration
                // TODO: Generalize data type casting
                let value_data_type = value.data_type();
                let access_data_type = access.data_type();

                let access_dims = access.dimensionality();
                let value_dims = value.dimensionality();
                if access_dims != value_dims {
                    panic!(
                        "Can't assign item {} with dimensions {:#?} to value with dimensions {:#?}",
                        access.id.id, access_dims, value_dims
                    )
                }
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
                    let mut manager = GlobalManager::get();
                    let prev_value_temp = value_temp.clone();
                    value_temp = manager.new_temp_address(&access_data_type).to_string();

                    manager.emit(Quadruple::type_cast(
                        &access_data_type,
                        prev_value_temp.as_str(),
                        value_temp.clone().as_str(),
                    ))
                }

                if access.is_immutable() {
                    panic!("Variable {} can't be mutated", access.id.id);
                }

                let access = access.reduce();

                GlobalManager::emit(Quadruple::unary(
                    Operator::Assign,
                    value_temp.as_str(),
                    access.as_str(),
                ));
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

                let mut condition_id = condition.reduce();
                let condition_dt = condition.data_type();
                if condition_dt != DataType::Bool {
                    condition_id = GlobalManager::emit_cast(&DataType::Bool, condition_id.as_str());
                }

                // goto instruction to skip if-true block
                let mut goto_if_false_quad = QuadrupleHold::new();

                if_block.generate();

                if let Some(block) = else_block {
                    // goto instruction to skip else block if condition was true
                    let mut goto_end_block = QuadrupleHold::new();

                    // Generate goto to skip to else block, if false
                    let goto_false_jump = GlobalManager::get_next_pos();
                    goto_if_false_quad
                        .release(Quadruple::goto_false(&condition_id, goto_false_jump));

                    block.generate();

                    // Update goto to skip else block
                    let end_pos = GlobalManager::get_next_pos();
                    goto_end_block.release(Quadruple::goto(end_pos));
                } else {
                    // Update goto to skip if false
                    let end_pos = GlobalManager::get_next_pos();
                    goto_if_false_quad.release(Quadruple::goto_false(&condition_id, end_pos));
                }
            }
            Statement::For {
                iterator_id,
                range,
                block,
            } => {
                let (start, end, step) = range;
                let start_dt = start.data_type();
                let end_dt = start.data_type();
                let step_dt = if let Some(step) = step {
                    Some(step.data_type())
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

                let start = start.reduce();
                let end = end.reduce();
                let step = if let Some(step) = step {
                    Some(step.reduce())
                } else {
                    None
                };

                let iterator_address = GlobalManager::get()
                    .get_env_mut()
                    .add_var(iterator_id, &DataType::Int, &Dimension::new_scalar(), true)
                    .to_string();

                GlobalManager::emit(Quadruple::unary(
                    Operator::Assign,
                    start.as_str(),
                    iterator_address.as_str(),
                ));

                let check_tmp = GlobalManager::new_temp(&DataType::Bool).to_string();

                let return_position = GlobalManager::get_next_pos();

                GlobalManager::emit(Quadruple::operation(
                    Operator::LessThan,
                    iterator_address.as_str(),
                    end.as_str(),
                    check_tmp.as_str(),
                ));

                // Quadruple::goto_false(check, position)
                let mut goto_false_hold = QuadrupleHold::new();

                block.generate();

                if let Some(step) = step {
                    GlobalManager::emit(Quadruple::operation(
                        Operator::Add,
                        iterator_address.as_str(),
                        step.as_str(),
                        iterator_address.as_str(),
                    ));
                } else {
                    let increment_tmp = GlobalManager::new_constant(
                        &DataType::Int,
                        &Const::new("1", DataType::Int),
                    )
                    .to_string();

                    GlobalManager::emit(Quadruple::operation(
                        Operator::Add,
                        iterator_address.as_str(),
                        increment_tmp.as_str(),
                        iterator_address.as_str(),
                    ));
                }

                let to_start_pos_quadruple = Quadruple::goto(return_position);
                GlobalManager::emit(Quadruple::goto(return_position.clone()));

                let end_pos = GlobalManager::get_next_pos();

                let to_end_pos_quadruple = Quadruple::goto_false(check_tmp.as_str(), end_pos);
                goto_false_hold.release(to_end_pos_quadruple.clone());

                GlobalManager::resolve_context(&ExitStatement::Continue, to_start_pos_quadruple);
                GlobalManager::resolve_context(&ExitStatement::Break, to_end_pos_quadruple);

                GlobalManager::get().get_env_mut().del_var(iterator_id);
                GlobalManager::emit(Quadruple::new("free", "", "", &iterator_address.as_str()));
            }
            Statement::While { condition, block } => {
                let start_pos = GlobalManager::get_next_pos();

                // Temporal storing condition value
                let mut condition_id = condition.reduce();
                let condition_dt = condition.data_type();
                if condition_dt != DataType::Bool {
                    condition_id = GlobalManager::emit_cast(&DataType::Bool, condition_id.as_str());
                }

                // Goto instruction to exit the loop
                let mut goto_false_cond = QuadrupleHold::new();

                block.generate();

                // Emit instruction to return to condition evaluation
                let to_start_pos_quadruple = Quadruple::goto(start_pos);
                GlobalManager::emit(to_start_pos_quadruple.clone());

                let end_pos = GlobalManager::get_next_pos();

                let to_end_pos_quadruple = Quadruple::goto(end_pos);

                // Emit instruction to return to condition evaluation
                goto_false_cond.release(Quadruple::goto_false(&condition_id, end_pos));

                GlobalManager::resolve_context(&ExitStatement::Continue, to_start_pos_quadruple);
                GlobalManager::resolve_context(&ExitStatement::Break, to_end_pos_quadruple);
            }
            Statement::FunctionDeclaration(func) => func.generate(),
            Statement::Return(ret) => {
                let mut return_item = ret.reduce();
                let manager = GlobalManager::get();
                let context = manager.get_env().current_env();
                assert_eq!(context.is_global, false);
                let return_type = context.return_type.clone().unwrap();
                drop(manager);
                if return_type != ret.data_type() {
                    return_item = GlobalManager::emit_cast(&return_type, return_item.as_str());
                }

                GlobalManager::emit(Quadruple::new_return(return_item.as_str()));
            }
            Statement::Break => GlobalManager::prepare_exit_stmt(&ExitStatement::Break),
            Statement::Continue => GlobalManager::prepare_exit_stmt(&ExitStatement::Continue),
            Statement::VoidReturn => GlobalManager::emit(Quadruple::void_return()),
        }
    }

    fn reduce(&self) -> String {
        todo!("reduce statement");
    }
}

impl Node for Function {
    fn generate(&mut self) -> () {
        let mut manager = GlobalManager::get();
        let next_position = manager.get_next_id();

        manager.update_func_position(&self.signature.id, next_position);
        manager.get_env_mut().switch(&self.signature.id);

        drop(manager);

        self.block.generate();

        GlobalManager::emit(Quadruple::end_func());

        GlobalManager::get()
            .get_env_mut()
            .switch(&String::from("global"));
        // GlobalManager::get().get_env().drop_env(&self.signature.id);
    }

    fn reduce(&self) -> String {
        todo!("Function reduce!");
    }
}

impl Node for Expression {
    fn dimensionality(&self) -> Vec<usize> {
        match &self {
            Expression::Const(constant) => constant.dimensionality(),
            Expression::Op(operation) => operation.dimensionality(),
            Expression::Access(access) => access.dimensionality(),
            Expression::Id(id) => id.dimensionality(),
            Expression::Call(call) => call.dimensionality(),
            Expression::Not(expr) => expr.dimensionality(),
            Expression::Negative(expr) => expr.dimensionality(),
        }
    }
    fn data_type(&self) -> DataType {
        match &self {
            Expression::Const(constant) => constant.dtype.clone(),
            Expression::Op(operation) => operation.data_type(),
            Expression::Access(access) => access.id.data_type(),
            Expression::Id(id) => id.data_type(),
            Expression::Call(call) => call.data_type(),
            Expression::Not(expr) => expr.data_type(),
            Expression::Negative(expr) => expr.data_type(),
        }
    }

    fn generate(&mut self) -> () {
        match self {
            Expression::Const(constant) => constant.generate(),
            Expression::Op(operation) => operation.generate(),
            Expression::Access(access) => access.generate(),
            Expression::Id(id) => id.generate(),
            Expression::Call(call) => call.generate(),
            Expression::Not(not) => not.generate(),
            Expression::Negative(_) => todo!(),
        }
    }

    fn reduce(&self) -> String {
        match self {
            Expression::Const(constant) => constant.reduce(),
            Expression::Op(operation) => operation.reduce(),
            Expression::Access(access) => access.reduce(),
            Expression::Id(id) => id.reduce(),
            Expression::Call(call) => call.reduce(),
            Expression::Not(not) => {
                let mut to_negate = not.reduce();
                let expr_type =
                    MemoryResolver::get_type_from_address(to_negate.parse().unwrap()).unwrap();

                if DataType::equivalent(expr_type, &DataType::Bool).is_err() {
                    panic!("Expression can't be casted to boolean");
                }
                if *expr_type != DataType::Bool {
                    to_negate = GlobalManager::emit_cast(&DataType::Bool, &to_negate);
                }

                let dest = GlobalManager::new_temp(&DataType::Bool).to_string();
                GlobalManager::emit(Quadruple::unary(
                    Operator::Not,
                    to_negate.as_str(),
                    dest.as_str(),
                ));

                dest
            }
            Expression::Negative(expr) => {
                let addr = expr.reduce();
                let expr_dt = expr.data_type();
                let new_addr = GlobalManager::new_temp(&expr_dt).to_string();

                GlobalManager::emit(Quadruple::unary(Operator::Neg, &addr, &new_addr));

                new_addr
            }
        }
    }
}

impl Node for Index {
    fn reduce(&self) -> String {
        match self {
            Self::Simple(idx) => idx.reduce(),
            Self::Range(_, _) => panic!("Range not supported"), // TODO
        }
    }
}

trait Pipe {
    fn resolve_pipe_type(&self) -> DataType {
        todo!()
    }

    fn resolve_pipe(&self) -> Box<Expression> {
        todo!()
    }
}

impl Pipe for Operation {
    fn resolve_pipe_type(&self) -> DataType {
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
                        let call_param = op.resolve_pipe();
                        let call = Call::new(&access.id.id, vec![call_param]);
                        call.data_type()
                    }
                    _ => {
                        let call = Call::new(&access.id.id, vec![Box::new(Expression::Op(op))]);
                        call.data_type()
                    }
                },
                _ => panic!(),
            }
        } else {
            panic!()
        }
    }

    fn resolve_pipe(&self) -> Box<Expression> {
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
                        let call_param = op.resolve_pipe();
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
    fn dimensionality(&self) -> Vec<usize> {
        if self.operator == Operator::Pipe {
            let new_tree = self.resolve_pipe();
            new_tree.dimensionality()
        } else {
            self.left.dimensionality()
        }
    }

    fn data_type(&self) -> DataType {
        match self.operator {
            Operator::Pipe => self.resolve_pipe_type(),
            _ => SemanticRules::match_type(
                self.operator,
                self.left.data_type(),
                self.right.data_type(),
            ),
        }
    }

    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        if self.operator == Operator::Pipe {
            let new_tree = self.resolve_pipe();
            return new_tree.reduce();
        }
        let left_dims = self.left.dimensionality();
        let right_dims = self.right.dimensionality();
        if left_dims != right_dims {
            panic!(
                "Can't operate items with dimensions {:#?} and {:#?}",
                left_dims, right_dims
            )
        }

        let mut left = self.left.reduce();
        let left_dt = self.left.data_type();
        let mut right = self.right.reduce();
        let right_dt = self.right.data_type();

        let dt = self.data_type();

        match dt {
            DataType::Int | DataType::Float | DataType::String | DataType::Pointer => {
                if left_dt != dt {
                    let new_left = GlobalManager::new_temp(&dt).to_string();
                    GlobalManager::emit(Quadruple::type_cast(
                        &dt,
                        left.as_str(),
                        new_left.as_str(),
                    ));

                    left = new_left
                }

                if right_dt != dt {
                    let new_right = GlobalManager::new_temp(&dt).to_string();
                    GlobalManager::emit(Quadruple::type_cast(
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
                            left = GlobalManager::emit_cast(&DataType::Bool, left.as_str());
                        }
                        if right_dt != DataType::Bool {
                            right = GlobalManager::emit_cast(&DataType::Bool, left.as_str());
                        }
                    }
                    OperatorType::Comparison => {
                        if left_dt != right_dt {
                            let max_dt = DataType::max(&left_dt, &right_dt);
                            if max_dt != left_dt {
                                left = GlobalManager::emit_cast(&max_dt, left.as_str());
                            } else {
                                right = GlobalManager::emit_cast(&max_dt, right.as_str());
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

        let mut manager = GlobalManager::get();

        let tmp = manager.new_temp_address(&dt).to_string();

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
    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        let const_address = GlobalManager::new_constant(&self.dtype, self);
        return const_address.to_string();
    }
}

impl Node for Id {
    fn dimensionality(&self) -> Vec<usize> {
        let mut man = GlobalManager::get();
        if let Some(id) = man.get_env_mut().get_var(&self.id) {
            return id.dimension.shape.clone();
        }
        panic!("id {} is not defined", self.id);
    }

    fn reduce(&self) -> String {
        self.address().to_string()
    }

    fn data_type(&self) -> DataType {
        match &self.dtype {
            Some(dtype) => dtype.clone(),
            _ => {
                let mut man = GlobalManager::get();
                if let Some(id) = man.get_env_mut().get_var(&self.id) {
                    return id.data_type.clone();
                }
                panic!("id {} is not defined", self.id);
            }
        }
    }

    fn address(&self) -> MemAddress {
        if let Some(var_entry) = GlobalManager::get().get_env_mut().get_var(&self.id) {
            return var_entry.address;
        } else {
            panic!("Cannot find id {} in scope", self.id);
        }
    }
}

trait ImmutableVar {
    fn is_immutable(&self) -> bool {
        todo!()
    }
}

impl ImmutableVar for Access {
    fn is_immutable(&self) -> bool {
        let id_var = GlobalManager::get()
            .get_env_mut()
            .get_var(&self.id.id)
            .cloned();

        if let Some(id_var) = id_var {
            id_var.immutable
        } else {
            panic!("Variable {} is not defined", self.id.id);
        }
    }
}

impl Node for Access {
    fn dimensionality(&self) -> Vec<usize> {
        let to_access_shape = self.id.dimensionality();
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

    fn data_type(&self) -> DataType {
        return self.id.data_type();
    }

    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        let access_item: SymbolEntry;
        let id_var = GlobalManager::get()
            .get_env_mut()
            .get_var(&self.id.id)
            .cloned();
        if let Some(entry) = id_var {
            access_item = entry;
        } else {
            panic!("Item {} does not exist!", self.id.id);
        }

        if self.indexing.len() == 0 {
            return self.id.address().to_string();
        } else if access_item.dimension.size == 1 {
            panic!("Can't index scalar value {}", self.id.id);
        }

        // Address to values used to index the array
        let indexing_addresses = self
            .indexing
            .iter()
            .map(|index| index.reduce())
            .collect::<Vec<String>>();

        if indexing_addresses.len() > access_item.dimension.dimensions as usize {
            panic!("Incompatible index!");
        }

        let shape_cp = access_item.dimension.shape.clone();
        let mut array_shape = shape_cp.iter();
        let acc_tmp = GlobalManager::new_temp(&DataType::Pointer).to_string();
        let mut first_run = true;

        zip(&indexing_addresses, &access_item.dimension.acc_size).for_each(|(index, dim_size)| {
            if let Some(dim) = array_shape.next() {
                GlobalManager::emit(Quadruple::verify(index.as_str(), dim.to_string().as_str()))
            }

            let dim_const = GlobalManager::new_constant(
                &DataType::Pointer,
                &Const::new(dim_size.to_string().as_str(), DataType::Pointer),
            );
            let dim_const = dim_const.to_string();

            if first_run {
                GlobalManager::emit(Quadruple::operation(
                    Operator::Mul,
                    index.as_str(),
                    dim_const.as_str(),
                    acc_tmp.as_str(),
                ));
                first_run = false;
            } else {
                let tmp = GlobalManager::new_temp(&DataType::Pointer);
                let tmp_str = tmp.to_string();

                GlobalManager::emit(Quadruple::operation(
                    Operator::Mul,
                    index.as_str(),
                    dim_const.as_str(),
                    tmp_str.as_str(),
                ));

                GlobalManager::emit(Quadruple::operation(
                    Operator::Add,
                    acc_tmp.as_str(),
                    tmp_str.as_str(),
                    acc_tmp.as_str(),
                ));
            }
        });

        let access_tmp = GlobalManager::new_temp(&DataType::Pointer);

        GlobalManager::emit(Quadruple::operation(
            Operator::Add,
            access_item.address.to_string().as_str(),
            acc_tmp.as_str(),
            access_tmp.to_string().as_str(),
        ));

        // let dump_address = GlobalManager::new_temp(&self.id.data_type()).to_string();

        // GlobalManager::emit(Quadruple::operation(
        //     Operator::Assign,
        //     format!("*{}", access_tmp).as_str(),
        //     "",
        //     dump_address.as_str(),
        // ));

        if self.indexing.len() == access_item.dimension.dimensions as usize {
            format!("*{}", access_tmp)
        } else {
            format!("{}", access_tmp)
        }

        // dump_address
    }
}

impl Node for Call {
    fn data_type(&self) -> DataType {
        if let Some(data_type) = NativeFunctions::data_type(&self.id) {
            data_type
        } else {
            GlobalManager::get().get_func(&self.id).return_type.clone()
        }
    }

    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        if let Some(return_value) = NativeFunctions::call_reduce(self) {
            return return_value;
        }

        let man = GlobalManager::get();
        let func = man.get_func(&self.id).clone();
        let return_type = func.return_type.clone();
        let param_defintions = func.params.clone();
        drop(man);
        let target_params_len = param_defintions.len();
        if self.params.len() != target_params_len {
            panic!(
                "Params size do not match {} {} - {}",
                self.id,
                self.params.len(),
                target_params_len
            );
        }

        GlobalManager::emit(Quadruple::era(self.id.as_str()));

        for (index, param) in self.params.iter().enumerate() {
            let (_, def_param_data_type, _) = param_defintions.get(index).unwrap();

            if def_param_data_type == &DataType::Pointer && param.dimensionality().len() > 0 {
                let param_address = param.reduce();
                GlobalManager::emit(Quadruple::param(param_address.as_str(), index));
                continue;
            }

            let mut param_address = param.reduce();
            let param_data_type = param.data_type();

            assert!(
                DataType::equivalent(&param_data_type, def_param_data_type).is_ok(),
                "Data type {:?} cannot be assigned to a variable {:?}.",
                param_data_type,
                def_param_data_type
            );

            // TODO: Refactor type casting instruction into func
            if param_data_type != *def_param_data_type {
                let value_temp = GlobalManager::new_temp(&def_param_data_type).to_string();

                GlobalManager::emit(Quadruple::type_cast(
                    &def_param_data_type,
                    param_address.as_str(),
                    value_temp.as_str(),
                ));

                param_address = value_temp.clone();
            }

            GlobalManager::emit(Quadruple::param(param_address.as_str(), index));
        }

        GlobalManager::emit(Quadruple::go_sub(self.id.as_str()));

        let mut manager = GlobalManager::get();
        if let Some(address) = manager.get_func_return(&self.id) {
            let return_value = manager.new_temp_address(&return_type).to_string();
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
