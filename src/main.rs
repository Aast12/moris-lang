use std::fs::{self, File};
use std::path::Path;

use moris_lang::ast::node::Node;

use moris_lang::ast::statements::Program;
use moris_lang::codegen::manager::{MANAGER, GlobalManager};
use moris_lang::codegen::meta::ProgramMeta;
use moris_lang::codegen::quadruples::Quadruple;
use moris_lang::parser::grammar::PProgramParser as Parser;
use moris_lang::vm::virtual_machine::VirtualMachine;

fn try_file(path: &str) -> Program {
    match fs::read_to_string(path) {
        Ok(file_content) => Parser::new().parse(file_content.as_str()).unwrap(),
        Err(error) => panic!("path: {} -> {}", path, error),
    }
}
fn fib(x: i32) -> i32 {
    if (x <= 2) {
        return x;
    }

    return fib(x - 1) + fib(x - 2);
}



fn main() {
    // let reader = File::open("program.o").unwrap();
    // let data: ProgramMeta = serde_pickle::from_reader(reader, Default::default()).unwrap();

    // println!("{:#?}", data);

    let path_buf = Path::new(env!("CARGO_MANIFEST_DIR")).join("./samples/fibonacci.mo");
    let path = path_buf.to_str().unwrap();

    let mut test_program = try_file(path);

    // print!("{:#?}", test_program);

    test_program.generate();

    // println!("meta \n{:#?}", GlobalManager::get().env);

    GlobalManager::get().dump();

    let mut vm = VirtualMachine::load("program.o");
    
    println!("META \n{:#?}", vm.data);
    println!("Memory \n{:#?}", vm.memory);

    println!("Quadruples");
    println!("{:#?}", GlobalManager::get().quadruples);
    let mut i = 0;
    println!();
    for quad in GlobalManager::get().quadruples.iter() {
        let Quadruple(fst, snd, trd, fth) = quad;
        println!("{}.\t{}\t{}\t{}\t{}", i, fst, snd, trd, fth);
        i += 1;
    }
    // println!("Starting execution");
    vm.execute();

    println!("Memory \n{:#?}", vm.memory);
    
    println!("FIB {}", fib(10));
    
    // let m = MANAGER.lock().unwrap();

    // let mut i = 0;
    // println!();
    // for quad in m.quadruples.iter() {
    //     let Quadruple(fst, snd, trd, fth) = quad;
    //     println!("|{}|{}|{}|{}|{}|", i, fst, snd, trd, fth);
    //     i += 1;
    // }
    // println!();
    // print!("{:#?}", m);
}
