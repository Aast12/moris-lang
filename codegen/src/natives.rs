use std::str::FromStr;

use memory::types::DataType;
use parser::{
    expressions::call::Call,
    functions::{FunctionParam, FunctionSignature},
};
use strum::{Display, EnumIter, EnumProperty, EnumString, EnumVariantNames, IntoEnumIterator};

use crate::{manager::GlobalManager, node::Node, quadruples::Quadruple};

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
    ReadCsv,
    Select,
    ToCsv,
}

impl NativeFunctions {
    pub fn data_type(id: &String) -> Option<DataType> {
        if let Ok(function_id) = NativeFunctions::from_str(id.as_str()) {
            if let Some(_) = function_id.get_str("check_params") {
                Some(DataType::Void)
            } else {
                Some(GlobalManager::get().get_func(&id).return_type.clone())
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
                    NativeFunctions::ReadCsv => (DataType::DataFrame, vec![]),
                    NativeFunctions::Select => (DataType::Series, vec![]),
                    NativeFunctions::ToCsv => (
                        DataType::Void,
                        vec![FunctionParam::new_scalar("df", DataType::DataFrame)],
                    ),
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

    pub fn call_reduce(ctx: &Call) -> Option<String> {
        let id = ctx.id.as_str();
        if let Ok(function_id) = NativeFunctions::from_str(id) {
            match function_id {
                NativeFunctions::Print | NativeFunctions::Println => {
                    ctx.params.iter().for_each(|param| {
                        let value = param.reduce();
                        GlobalManager::emit(Quadruple::new(
                            NativeFunctions::Print.to_string().as_str(),
                            "",
                            "",
                            value.as_str(),
                        ));
                    });
                    if function_id == NativeFunctions::Println {
                        GlobalManager::emit(Quadruple::new(
                            NativeFunctions::Print.to_string().as_str(),
                            "",
                            "",
                            "\n",
                        ));
                    }
                    Some(String::from("VOID"))
                }
                NativeFunctions::Read => {
                    GlobalManager::emit(Quadruple::era(id));

                    ctx.params.iter().enumerate().for_each(|(index, param)| {
                        let value = param.reduce();
                        GlobalManager::emit(Quadruple::param(value.as_str(), index));
                    });

                    GlobalManager::emit(Quadruple::go_sub(id));

                    Some(String::from("VOID"))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
