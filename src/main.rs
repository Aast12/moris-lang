use std::path::Path;
use std::{fs, path};

use moris_lang::ast::node::Node;

use moris_lang::ast::quadruples::MANAGER;
use moris_lang::ast::statements::Program;
use moris_lang::parser::grammar::PProgramParser as Parser;

fn try_file(path: &str) -> Program {
    match fs::read_to_string(path) {
        Ok(file_content) => Parser::new().parse(file_content.as_str()).unwrap(),
        Err(error) => panic!("path: {} -> {}", path, error),
    }
}

fn main() {
    let path_buf = Path::new(env!("CARGO_MANIFEST_DIR")).join("./samples/while.mo");
    let path = path_buf.to_str().unwrap();

    let mut test_program = try_file(path);

    print!("{:#?}", test_program);

    test_program.generate();

    let m = MANAGER.lock().unwrap();

    let mut i = 0;
    println!();
    for quad in m.quadruples.iter() {
        println!("{}: {:#?}", i, quad);
        i += 1;
    }
    println!();
    print!("{:#?}", m);
}
