use std::env;
use moris_lang::vm::inspector::Inspector;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let inspector = Inspector::new(format!("samples/{path}").as_str());
    inspector.debug();
}
