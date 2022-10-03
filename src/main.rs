pub mod ast;

pub mod parser;


#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
    println!("{:#?}", parser::grammar::ProgramParser::new().parse(r#"
        fn myFunc(x: int, y: float): bool {
            5;
            callFn();
            let x: int = 5;
        }

        let x: float;
        let y: int = 7;
    "#).unwrap());

    println!("{:#?}", parser::grammar::ProgramParser::new().parse(r#"
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
    "#).unwrap());

    // println!("{:#?}", parser::grammar::GlobalStatementParser::new().parse(r#"
    //     let x: int = 2;
    // "#).unwrap());

    // println!("{:#?}", parser::grammar::GlobalStatementParser::new().parse(r#"
    //     if (x) {
    //         c = 5;
    //     }
    // "#).unwrap());

    // println!("{:#?}", parser::grammar::GlobalStatementParser::new().parse(r#"
    //     fn myFunc(x: int, y: float): bool {
    //         5;
    //         callFn();
    //         let x: int = 5;
    //         x = 7;
    //         for (iter in iteration) {
    //             x = 7;
    //             df |> myFunc;
    //         }
    //     }
    // "#).unwrap());
}
