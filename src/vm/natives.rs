use codegen::natives::*;
use memory::{
    resolver::MemoryResolver,
    types::{DataType, FloatType, IntType},
};
use polars::prelude::{AnyValue, CsvReader, SerReader};
use rand::Rng;
use std::{io, iter::zip};

use crate::plots::{backend::TextDrawingBackend, context::PlotContext};

use super::{
    memory_manager::{Item, MemoryManager},
    utils::*,
};

pub fn run_native(
    plot_ctx: &mut PlotContext,
    native_func: NativeFunction,
    memory: &mut MemoryManager,
) -> Option<(NativeFunction, Item)> {
    let mut return_value: Option<(NativeFunction, Item)> = None;
    let mut rng = rand::thread_rng();

    match native_func {
        NativeFunction::Zeros => {
            let params = memory.pop_params();
            let array_pointer = params.get(0).unwrap();

            if let Item::Pointer(array_address) = array_pointer {
                memory.alter_array(array_address, |memory, (next_address, _)| {
                    memory.update(next_address, Item::Int(0));
                });
            }
        }
        NativeFunction::RandomFill => {
            let params = memory.pop_params();
            let array_pointer = params.get(0).unwrap();
            let min = params.get(1).unwrap().to_owned();
            let min = min.unwrap_int();
            let max = params.get(2).unwrap().to_owned();
            let max = max.unwrap_int();

            if let Item::Pointer(array_address) = array_pointer {
                let array_type = MemoryResolver::get_type_from_address(*array_address).unwrap();

                memory.alter_array(array_address, |memory, (next_address, _)| {
                    match array_type {
                        DataType::Int => memory.update(
                            next_address,
                            Item::Int(rand::thread_rng().gen_range(min..max)),
                        ),
                        DataType::Float => memory.update(
                            next_address,
                            Item::Float(
                                rand::thread_rng().gen_range(min as FloatType..max as FloatType),
                            ),
                        ),
                        _ => panic!(
                            "Can't fill array of type {:#?} with random numbers!",
                            array_type
                        ),
                    };
                });
            }
        }
        NativeFunction::ScalarMul => {
            let params = memory.pop_params();
            let array_pointer = params.get(0).unwrap();
            let factor = params.get(1).unwrap().to_owned();
            let factor = factor.unwrap_float();

            if let Item::Pointer(array_address) = array_pointer {
                let array_type = MemoryResolver::get_type_from_address(*array_address).unwrap();

                memory.alter_array(array_address, |memory, (next_address, value)| {
                    if let Some(value) = value {
                        match array_type {
                            DataType::Int => memory.update(
                                next_address,
                                Item::Int(value.unwrap_int() * factor as IntType),
                            ),
                            DataType::Float => memory
                                .update(next_address, Item::Float(value.unwrap_float() * factor)),
                            _ => panic!(
                                "Can't fill array of type {:#?} with random numbers!",
                                array_type
                            ),
                        };
                    } else {
                        panic!("Undefined element in array multiplication!")
                    }
                });
            }
        }
        NativeFunction::Read => {
            let params = memory.pop_params_address();

            let mut in_line = String::new();

            io::stdin()
                .read_line(&mut in_line)
                .expect("failed to readline");

            while in_line.ends_with('\n') || in_line.ends_with('\n') {
                in_line.pop();
            }

            let inputs: Vec<&str> = in_line.split(' ').collect();

            zip(inputs, params).for_each(|(input, param_addr)| {
                let input_type = MemoryResolver::get_type_from_address(param_addr).unwrap();
                let item = match input_type {
                    DataType::Int => match input.parse::<IntType>() {
                        Ok(parsed) => Item::Int(parsed),
                        Err(err) => panic!("{:#?}", err),
                    },
                    DataType::Float => match input.parse::<FloatType>() {
                        Ok(parsed) => Item::Float(parsed),
                        Err(err) => panic!("{:#?}", err),
                    },
                    DataType::String => Item::String(input.to_string()),
                    _ => panic!("Type {:#?} can't be parsed from a string", input_type),
                };

                memory.update(param_addr, item)
            });
        }
        NativeFunction::ReadCsv => {
            let params = memory.pop_params();
            let file_path = params.get(0).unwrap();

            if let Item::String(file_path) = file_path {
                let csv = CsvReader::from_path(file_path.as_str());
                if let Ok(df) = csv {
                    let df_result = df.with_ignore_parser_errors(true).finish();
                    if let Ok(df) = df_result {
                        return_value = Some((NativeFunction::ReadCsv, Item::DataFrame(df)));
                    } else {
                        panic!(
                            "Could not read file {file_path} -> {}",
                            df_result.unwrap_err()
                        );
                    }
                } else {
                    panic!("Could not read file {file_path}");
                }
            }
        }
        NativeFunction::Select => {
            let params = memory.pop_params();
            let df = params.get(0).unwrap().to_owned();
            let df = df.unwrap_data_frame();
            let select_col = params.get(1).unwrap().to_owned();
            let select_col = select_col.unwrap_string();

            let selected = df.column(select_col.as_str());
            if let Ok(selected) = selected.cloned() {
                return_value = Some((NativeFunction::Select, Item::Series(selected)));
            } else {
                panic!("Can't select column {} from DataFrame!", select_col);
            }
        }
        NativeFunction::PrintNames => {
            let params = memory.pop_params();
            let df = unwrap_df_param(&params, 0);

            df.get_columns().iter().for_each(|col| {
                println!("{:#?} - {:#?}", col.name(), col.dtype());
            })
        }
        NativeFunction::SetPlotOut => {
            let params = memory.pop_params();
            let path = unwrap_str_param(&params, 0);

            plot_ctx.set_output_path(path);
        }
        NativeFunction::SetCaption => {
            let params = memory.pop_params();
            let caption = unwrap_str_param(&params, 0);

            plot_ctx.set_caption(caption);
        }
        NativeFunction::SetXTitle => {
            let params = memory.pop_params();
            let caption = unwrap_str_param(&params, 0);

            plot_ctx.set_x_label(caption);
        }
        NativeFunction::SetYTitle => {
            let params = memory.pop_params();
            let caption = unwrap_str_param(&params, 0);

            plot_ctx.set_y_label(caption);
        }
        NativeFunction::SetXBounds => {
            let params = memory.pop_params();
            let min = unwrap_float_param(&params, 0);
            let max = unwrap_float_param(&params, 1);

            plot_ctx.set_x_bounds((min, max));
        }
        NativeFunction::SetYBounds => {
            let params = memory.pop_params();
            let min = unwrap_float_param(&params, 0);
            let max = unwrap_float_param(&params, 1);

            plot_ctx.set_y_bounds((min, max));
        }
        NativeFunction::Scatter => {
            let params = memory.pop_params();
            let x_series = unwrap_series_param(&params, 0);
            let y_series = unwrap_series_param(&params, 1);

            plot_ctx
                .draw_scatter::<TextDrawingBackend>(&x_series, &y_series)
                .unwrap();

            plot_ctx.reset_context();
        }
        NativeFunction::Random => {
            memory.pop_params();
            return_value = Some((NativeFunction::Random, Item::Float(rng.gen_range(0.0..1.0))))
        }
        NativeFunction::Describe => {
            let params = memory.pop_params();
            let df = unwrap_df_param(&params, 0);
            println!("{:#?}", df.describe(None));
        }
        NativeFunction::Sum
        | NativeFunction::Mean
        | NativeFunction::Median
        | NativeFunction::Std
        | NativeFunction::Var => {
            let params = memory.pop_params();
            let target = params.first().unwrap();

            let value: FloatType;

            if target.is_pointer() {
                let start_address = target.clone().unwrap_pointer();
                let start_item = memory.resolved_get(start_address.clone());
                let array = memory.get_array(&start_address);
                let filtered = array.iter().filter(|item| item.is_some());

                let mut items = match start_item {
                    Item::Int(_) => filtered
                        .map(|item| item.clone().unwrap().unwrap_int() as FloatType)
                        .collect::<Vec<FloatType>>(),
                    Item::Float(_) => filtered
                        .map(|item| item.clone().unwrap().unwrap_float())
                        .collect::<Vec<FloatType>>(),
                    _ => panic!("Can't calculate {} of the item type.", native_func),
                };

                value = match native_func {
                    NativeFunction::Sum => items.iter().sum(),
                    NativeFunction::Mean => {
                        items.iter().sum::<FloatType>() / items.len() as FloatType
                    }
                    NativeFunction::Median => {
                        items.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        let half = items.len() / 2;

                        if items.len() % 2 == 0 {
                            (*items.get(half - 1).unwrap() + *items.get(half).unwrap())
                                / 2 as FloatType
                        } else {
                            *items.get(half + 1).unwrap()
                        }
                    }
                    NativeFunction::Std => {
                        let size = items.len() as FloatType;
                        let mean: FloatType = items.iter().sum::<FloatType>() / size;
                        let sum = items
                            .iter()
                            .map(|item| (item - mean).powi(2))
                            .sum::<FloatType>();

                        (sum / size).sqrt()
                    }
                    NativeFunction::Var => {
                        let size = items.len() as FloatType;
                        let mean: FloatType = items.iter().sum::<FloatType>() / size;
                        let sum = items
                            .iter()
                            .map(|item| (item - mean).powi(2))
                            .sum::<FloatType>();

                        sum / size
                    }
                    _ => panic!(),
                }
            } else {
                let target = target.to_owned().unwrap_series();
                value = match native_func {
                    NativeFunction::Sum => target.sum().unwrap(),
                    NativeFunction::Mean => target.mean().unwrap(),
                    NativeFunction::Median => target.median().unwrap(),
                    NativeFunction::Std => {
                        if let AnyValue::Float64(item) = target
                            .std_as_series(0)
                            .cast(&polars::prelude::DataType::Float64)
                            .unwrap()
                            .get(0)
                        {
                            item
                        } else {
                            panic!()
                        }
                    }
                    NativeFunction::Var => {
                        if let AnyValue::Float64(item) = target
                            .var_as_series(0)
                            .cast(&polars::prelude::DataType::Float64)
                            .unwrap()
                            .get(0)
                        {
                            item
                        } else {
                            panic!()
                        }
                    }
                    _ => panic!(),
                }
            }
            return_value = Some((native_func, Item::Float(value)));
        }
        NativeFunction::ToCsv => todo!(),
        _ => todo!(),
    }

    return_value
}
