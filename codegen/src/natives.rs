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
pub enum NativeFunctions {
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
}

impl NativeFunctions {
    pub fn data_type(id: &String, manager: &mut Manager) -> Option<DataType> {
        if let Ok(function_id) = NativeFunctions::from_str(id.as_str()) {
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
        NativeFunctions::iter()
            .filter(|func| {
                if let Some(_) = func.get_str("check_params") {
                    false
                } else {
                    true
                }
            })
            .map(|func| {
                let (data_type, params): (DataType, Vec<FunctionParam>) = match func {
                    NativeFunctions::Zeros => (
                        DataType::Void,
                        vec![FunctionParam::new_scalar("arr", DataType::Pointer)],
                    ),
                    NativeFunctions::RandomFill => (
                        DataType::Void,
                        vec![
                            FunctionParam::new_scalar("arr", DataType::Pointer),
                            FunctionParam::new_scalar("min", DataType::Int),
                            FunctionParam::new_scalar("max", DataType::Int),
                        ],
                    ),
                    NativeFunctions::ScalarMul => (
                        DataType::Void,
                        vec![
                            FunctionParam::new_scalar("arr", DataType::Pointer),
                            FunctionParam::new_scalar("factor", DataType::Float),
                        ],
                    ),
                    NativeFunctions::ReadCsv => (
                        DataType::DataFrame,
                        vec![FunctionParam::new_scalar("file_path", DataType::String)],
                    ),
                    NativeFunctions::Select => (
                        DataType::Series,
                        vec![
                            FunctionParam::new_scalar("df", DataType::DataFrame),
                            FunctionParam::new_scalar("col", DataType::String),
                        ],
                    ),
                    NativeFunctions::Scatter => (
                        DataType::Void,
                        vec![
                            FunctionParam::new_scalar("x", DataType::Series),
                            FunctionParam::new_scalar("y", DataType::Series),
                        ],
                    ),
                    NativeFunctions::PrintNames => (
                        DataType::Void,
                        vec![FunctionParam::new_scalar("df", DataType::DataFrame)],
                    ),
                    NativeFunctions::ToCsv => (
                        DataType::Void,
                        vec![FunctionParam::new_scalar("df", DataType::DataFrame)],
                    ),
                    NativeFunctions::Random => (DataType::Float, vec![]),
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
        if let Ok(function_id) = NativeFunctions::from_str(id) {
            match function_id {
                NativeFunctions::Print | NativeFunctions::Println => {
                    ctx.params.iter().for_each(|param| {
                        let value = param.reduce(manager);
                        manager.emit(Quadruple::new(
                            NativeFunctions::Print.to_string().as_str(),
                            "",
                            "",
                            value.as_str(),
                        ));
                    });
                    if function_id == NativeFunctions::Println {
                        manager.emit(Quadruple::new(
                            NativeFunctions::Print.to_string().as_str(),
                            "",
                            "",
                            "\n",
                        ));
                    }
                    Some(String::from("VOID"))
                }
                NativeFunctions::Read => {
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
