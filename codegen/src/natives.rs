use std::str::FromStr;

use memory::types::DataType;
use parser::{
    expressions::call::Call,
    functions::{FunctionParam, FunctionSignature},
    types::Operator,
};
use strum::{Display, EnumIter, EnumProperty, EnumString, EnumVariantNames, IntoEnumIterator};

use crate::{manager::Manager, node::ExpressionNode, quadruples::Quadruple};

/// Defines all the native functions whose implementation is in the side
/// of the virtual machine
#[derive(Debug, PartialEq, EnumString, EnumVariantNames, Display, EnumIter, EnumProperty)]
#[strum(serialize_all = "snake_case")]
pub enum NativeFunction {
    #[strum(props(check_params = "false"))]
    Print,
    #[strum(props(check_params = "false"))]
    Println,
    #[strum(props(check_params = "false"))]
    Read,
    Zeros,
    Random,
    RandomFill,
    ScalarMul,
    ReadCsv,
    Select,
    PrintNames,
    Scatter,
    ToCsv,
    SetCaption,
    SetXTitle,
    SetYTitle,
    SetXBounds,
    SetYBounds,
    SetPlotOut,
    Describe,
    Mean,
    Median,
    Std,
    Sum,
    Var,
}

fn ptr_param(name: &str) -> FunctionParam {
    FunctionParam::new_scalar(name, DataType::Pointer)
}

fn int_param(name: &str) -> FunctionParam {
    FunctionParam::new_scalar(name, DataType::Int)
}

fn str_param(name: &str) -> FunctionParam {
    FunctionParam::new_scalar(name, DataType::String)
}

fn float_param(name: &str) -> FunctionParam {
    FunctionParam::new_scalar(name, DataType::Float)
}

fn df_param(name: &str) -> FunctionParam {
    FunctionParam::new_scalar(name, DataType::DataFrame)
}

fn series_param(name: &str) -> FunctionParam {
    FunctionParam::new_scalar(name, DataType::Series)
}

impl NativeFunction {
    pub fn data_type(id: &String, manager: &mut Manager) -> Option<DataType> {
        if let Ok(function_id) = NativeFunction::from_str(id.as_str()) {
            if let Some(_) = function_id.get_str("check_params") {
                Some(DataType::Void)
            } else {
                Some(manager.get_func(&id).return_type.clone())
            }
        } else {
            None
        }
    }

    /// Returns the function definitions for all native functions.
    /// This ensures validation of proper argument count and type checking for
    /// these functions.
    pub fn get_function_definitions() -> Vec<FunctionSignature> {
        NativeFunction::iter()
            .filter(|func| {
                if let Some(_) = func.get_str("check_params") {
                    false
                } else {
                    true
                }
            })
            .map(|func| {
                let (data_type, params): (DataType, Vec<FunctionParam>) = match func {
                    NativeFunction::Zeros => (DataType::Void, vec![ptr_param("arr")]),
                    NativeFunction::RandomFill => (
                        DataType::Void,
                        vec![ptr_param("arr"), int_param("min"), int_param("max")],
                    ),
                    NativeFunction::ScalarMul => (
                        DataType::Void,
                        vec![ptr_param("arr"), float_param("factor")],
                    ),
                    NativeFunction::ReadCsv => (DataType::DataFrame, vec![str_param("file_path")]),
                    NativeFunction::Select => {
                        (DataType::Series, vec![df_param("df"), str_param("col")])
                    }
                    NativeFunction::Scatter => {
                        (DataType::Void, vec![series_param("x"), series_param("y")])
                    }
                    NativeFunction::SetCaption => (DataType::Void, vec![str_param("caption")]),
                    NativeFunction::SetXTitle => (DataType::Void, vec![str_param("title")]),
                    NativeFunction::SetYTitle => (DataType::Void, vec![str_param("title")]),
                    NativeFunction::SetXBounds => {
                        (DataType::Void, vec![float_param("min"), float_param("max")])
                    }
                    NativeFunction::SetYBounds => {
                        (DataType::Void, vec![float_param("min"), float_param("max")])
                    }
                    NativeFunction::SetPlotOut => (DataType::Void, vec![str_param("path")]),
                    NativeFunction::PrintNames => (DataType::Void, vec![df_param("df")]),
                    NativeFunction::ToCsv => (DataType::Void, vec![df_param("df")]),
                    NativeFunction::Describe => (DataType::Void, vec![df_param("df")]),
                    NativeFunction::Random => (DataType::Float, vec![]),
                    NativeFunction::Mean
                    | NativeFunction::Median
                    | NativeFunction::Std
                    | NativeFunction::Sum
                    | NativeFunction::Var => (DataType::Float, vec![]), // Params are checked in custom reduce
                    _ => panic!(),
                };

                FunctionSignature {
                    id: func.to_string(),
                    params,
                    data_type,
                    is_native: true,
                }
            })
            .collect::<Vec<FunctionSignature>>()
    }

    /// Defines custom reduce logic for Call nodes.
    /// If None is returned, the native function call will be treated as
    /// every other function.
    pub fn call_reduce(ctx: &Call, manager: &mut Manager) -> Option<String> {
        let id = ctx.id.as_str();
        if let Ok(function_id) = NativeFunction::from_str(id) {
            match function_id {
                NativeFunction::Print | NativeFunction::Println => {
                    ctx.params.iter().for_each(|param| {
                        let value = param.reduce(manager);
                        manager.emit(Quadruple::new(
                            NativeFunction::Print.to_string().as_str(),
                            "",
                            "",
                            value.as_str(),
                        ));
                    });
                    if function_id == NativeFunction::Println {
                        manager.emit(Quadruple::new(
                            NativeFunction::Print.to_string().as_str(),
                            "",
                            "",
                            "\n",
                        ));
                    }
                    Some(String::from("VOID"))
                }
                NativeFunction::Read => {
                    manager.emit(Quadruple::era(id));

                    ctx.params.iter().enumerate().for_each(|(index, param)| {
                        match **param {
                            parser::expressions::Expression::Access(_) => (),
                            _ => panic!("Can only read values from variables"),
                        };

                        let value_addr = param.reduce(manager);

                        manager.emit(Quadruple::param(value_addr.as_str(), index));
                    });

                    manager.emit(Quadruple::go_sub(id));

                    Some(String::from("VOID"))
                }
                NativeFunction::Mean
                | NativeFunction::Median
                | NativeFunction::Std
                | NativeFunction::Var => {
                    manager.emit(Quadruple::era(id));

                    if ctx.params.len() != 1 {
                        panic!(
                            "Function {id} takes one parameter, {} were provided",
                            ctx.params.len()
                        );
                    }

                    let param = ctx.params.get(0).unwrap();
                    let param_dt = param.data_type(manager);

                    if param_dt != DataType::Series && param.dimensionality(manager).len() == 0 {
                        panic!("Function {id} does not accept scalar values");
                    }

                    let param_tmp = param.reduce(manager);

                    manager.emit(Quadruple::param(param_tmp.as_str(), 0));

                    manager.emit(Quadruple::go_sub(id));

                    if let Some(func_return_address) = manager.get_func_return(&String::from(id)) {
                        let return_value = manager.new_temp(&DataType::Float).to_string();

                        manager.emit(Quadruple::unary(
                            Operator::Assign,
                            func_return_address.to_string().as_str(),
                            return_value.as_str(),
                        ));

                        Some(return_value)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
