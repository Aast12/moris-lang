use moris_lang::ast::node::Node;

pub mod ast;
pub mod env;
pub mod parser;
pub mod semantics;
pub mod symbols;

use moris_lang::ast::quadruples::MANAGER;
use moris_lang::parser::grammar::PProgramParser as Parser;
use regex::Error;

// #[cfg(not(test))]
fn main() {
    use moris_lang::ast::{expressions::id::Id, quadruples::Manager, types::DataType};

    let m = Manager::new();

    let mut id = Id::new("dx", Some(DataType::Float));
    id.set_manager(&m);
    let mut id2 = Id::new("dx2", Some(DataType::Int));
    id2.set_manager(&m);

    // println!("{:#?}", id.reduce().dump());

    // print!("{:#?}", Parser::new().parse("for + 5;").unwrap());
    let test_program = Parser::new().parse(
        "
    let x: int = 5;


    fn main(y: bool): float {
        let y: float = 7;

        return x  + 2 - 3 * y / 3;
    }
    ",
    );

    let mut program_node = test_program.unwrap();
    print!("{:#?}", program_node);
    program_node.generate();

    let m = MANAGER.lock().unwrap();

    for quad in m.quadruples.iter() {
        println!("{:#?}", quad);
    }
    print!("{:#?}", m);

    
    // program_node.generate();
    // print!("{:#?}", program_nodse);
    return ();
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
