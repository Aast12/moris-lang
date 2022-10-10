use moris_lang::ast::node::Node;

pub mod ast;
pub mod symbols;
pub mod parser;
pub mod semantics;

use moris_lang::parser::grammar::PProgramParser as Parser;


// #[cfg(not(test))]
fn main() {
    
    use moris_lang::ast::{quadruples::Manager, expressions::id::Id, types::DataType};

    let m = Manager::new();

    let mut id = Id::new("dx", Some(DataType::Float));
    id.set_manager(&m);
    let mut id2 = Id::new("dx2", Some(DataType::Int));
    id2.set_manager(&m);


    println!("{:#?}", id.reduce().dump());
    
    // print!("{:#?}", Parser::new().parse("for + 5;").unwrap());
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
