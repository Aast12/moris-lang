use moris_lang::vm::inspector::Inspector;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let inspector = Inspector::new(format!("examples/{path}").as_str());
    // inspector.debug();
}
