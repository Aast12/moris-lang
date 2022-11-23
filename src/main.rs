use moris_lang::vm::runner::Runner;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let mut runner = Runner::new(path).unwrap();
    runner.compile_and_run();
}
