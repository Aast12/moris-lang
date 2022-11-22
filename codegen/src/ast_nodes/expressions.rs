use std::iter::zip;

use memory::{resolver::{MemoryResolver, MemAddress}, types::DataType};
use parser::{expressions::{Expression, constant::Const, id::{Id, Access}, operation::Operation, call::Call, Index}, types::{Operator, OperatorType}, semantics::SemanticRules};

use crate::{
    manager::Manager,
    node::{AccessNode, ExpressionNode, Node},
    quadruples::Quadruple, symbols::SymbolEntry, natives::NativeFunction,
};

impl AccessNode for Expression {}

impl ExpressionNode for Expression {
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

impl Node for Expression {
    fn generate(&mut self, manager: &mut Manager) -> () {
        match self {
            Expression::Const(constant) => constant.reduce(manager),
            Expression::Op(operation) => operation.reduce(manager),
            Expression::Access(access) => access.reduce(manager),
            Expression::Id(id) => id.reduce(manager),
            Expression::Call(call) => call.reduce(manager),
            Expression::Not(not) => not.reduce(manager),
            Expression::Negative(_) => todo!(),
        };
    }
}

impl ExpressionNode for Index {
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

impl ExpressionNode for Operation {
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

impl ExpressionNode for Const {
    fn reduce(&self, manager: &mut Manager) -> String {
        let const_address = manager.new_constant(&self.dtype, self);
        return const_address.to_string();
    }
}

impl AccessNode for Id {
    fn address(&self, manager: &mut Manager) -> MemAddress {
        if let Some(var_entry) = manager.get_env_mut().get_var(&self.id) {
            return var_entry.address;
        } else {
            panic!("Cannot find id {} in scope", self.id);
        }
    }
}

impl ExpressionNode for Id {
    fn dimensionality(&self, manager: &mut Manager) -> Vec<usize> {
        if let Some(id) = manager.get_env_mut().get_var(&self.id) {
            return id.dimension.shape.clone();
        }
        panic!("id {} is not defined", self.id);
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

    fn reduce(&self, manager: &mut Manager) -> String {
        self.address(manager).to_string()
    }
}

pub trait ImmutableVar {
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

impl ExpressionNode for Access {
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

impl ExpressionNode for Call {
    fn data_type(&self, manager: &mut Manager) -> DataType {
        if let Some(data_type) = NativeFunction::data_type(&self.id, manager) {
            data_type
        } else {
            manager.get_func(&self.id).return_type.clone()
        }
    }

    fn reduce(&self, manager: &mut Manager) -> String {
        if let Some(return_value) = NativeFunction::call_reduce(self, manager) {
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
