use moris_lang::ast::node::Node;

use moris_lang::ast::quadruples::MANAGER;
// use moris_lang::ast::types::DataType;
use moris_lang::memory::resolver::{MemoryResolver, MemoryScope, SCOPE_OFFSETS, TYPE_OFFSETS};
use moris_lang::memory::types::DataType;
use moris_lang::parser::grammar::PProgramParser as Parser;

// #[cfg(not(test))]
fn main() {
    let test_program = Parser::new().parse(
        "
        fn myFunc(x: int, y: float): bool {
            5;
            callFn();
            let z:bool=7;
            return z;
        }

        let x: float;
        let y: int = 7;

    ",
    );

    let mut program_node = test_program.unwrap();
    print!("{:#?}", program_node);

    program_node.generate();

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
