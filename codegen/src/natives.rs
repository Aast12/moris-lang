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
    Print,
    Println,
    Zeros,
    ReadCsv,
    Select,
    ToCsv,
}

impl NativeFunctions {
    pub fn data_type(id: &String) -> Option<DataType> {
        if let Ok(function_id) = NativeFunctions::from_str(id.as_str()) {
            match function_id {
                NativeFunctions::Print | NativeFunctions::Println => Some(DataType::Void), // Functions with no formal definition e.g. infinite params
                _ => Some(GlobalManager::get().get_func(&id).return_type.clone()),
            }
        } else {
            None
        }
    }

    pub fn get_function_definitions() -> Vec<FunctionSignature> {
        NativeFunctions::iter()
            .filter(|func| match func {
                NativeFunctions::Print | NativeFunctions::Println => false,
                _ => true,
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
                _ => None,
            }
        } else {
            None
        }
    }
}
