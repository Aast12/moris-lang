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
    let z: int = 0;
    let y: float = 7;

    while (z < 10) {
        z = z + 1;
        y = y + 2;
        if (z / 7 == 0) {
            continue;
        }
        if (y / 7 == 0) {
            break;
        }
    }

    ",
    );

    // let test_program = Parser::new().parse(
    //     "
    // let x: int = 5;
    // let z: int = 7 + 2 / x;

    // while (x < 7 && x > 2) {
    //     z = 8;
    //     z = 9;
    // }
    // z = 10;
    // ",
    // );

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
    return;
    // program_node.generate();
    // print!("{:#?}", program_nodse);
    print!(
        "{:#?}",
        Parser::new()
            .parse(
                "if(x == 2){
        x = 3;
        fncall();
    }else if(x == 5){
        x = 4;
    } else {
        x + 7;   
    }"
            )
            .unwrap()
    );

    println!(
        "{:#?}",
        Parser::new()
            .parse(
                r#"
        fn myFunc(x: int, y: float): bool {
            5;
            callFn();
            let x:int=7;
            return x;
        }

        let x: float;
        let y: int = 7;
    "#
            )
            .unwrap()
    );

    println!(
        "{:#?}",
        Parser::new()
            .parse(
                r#"
        fn myFunc(x: int, y: float): bool {
            5;
            callFn();
            let x: int = 5;
        }

        let x: float;
        let y: int = 7;

        if(x) {
            if (y) {
                x = 6;
            }
        } else{
            x = 7;
        }
    "#
            )
            .unwrap()
    );

    println!(
        "{:#?}",
        Parser::new()
            .parse(
                r#"
        let x: int = 2;
    "#
            )
            .unwrap()
    );

    println!(
        "{:#?}",
        Parser::new()
            .parse(
                r#"
        if (x) {
            c = 5;
        }
    "#
            )
            .unwrap()
    );

    println!(
        "{:#?}",
        Parser::new()
            .parse(
                r#"
        fn myFunc(x: int, y: float): bool {
            5;
            callFn();
            let x: int = 5;
            x = 7;
            for (iter in iteration) {
                x = 7;
                df |> myFunc;
            }
        }
    "#
            )
            .unwrap()
    );
}
