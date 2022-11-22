use std::str::FromStr;

use memory::types::DataType;
use parser::{
    expressions::call::Call,
    functions::{FunctionParam, FunctionSignature},
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
                    NativeFunction::Random => (DataType::Float, vec![]),
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
                _ => None,
            }
        } else {
            None
        }
    }
}
