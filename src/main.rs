pub mod ast;
pub mod symbols;

pub mod parser;

#[cfg(not(test))]
fn main() {
    // print!("{:#?}", parser::grammar::ProgramParser::new().parse("for + 5;").unwrap());
    print!(
        "{:#?}",
        parser::grammar::ProgramParser::new()
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
        parser::grammar::ProgramParser::new()
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
        parser::grammar::ProgramParser::new()
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
        parser::grammar::ProgramParser::new()
            .parse(
                r#"
        let x: int = 2;
    "#
            )
            .unwrap()
    );

    println!(
        "{:#?}",
        parser::grammar::ProgramParser::new()
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
        parser::grammar::ProgramParser::new()
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
