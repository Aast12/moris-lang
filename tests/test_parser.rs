use moris_lang::parser;

fn get_parser() -> parser::grammar::ProgramParser {
    parser::grammar::ProgramParser::new()
}

fn expect_fail(in_str: &str) {
    let parser = get_parser();
    assert!(std::panic::catch_unwind(|| parser.parse(in_str).unwrap()).is_err());
}

fn expect_success(in_str: &str) {
    let parser = get_parser();
    assert!(std::panic::catch_unwind(|| parser.parse(in_str).unwrap()).is_ok());
}

#[test]
fn test_expressions() {
    expect_success("5 + 2 - 3 * (5 + 10) / 3;");
    expect_success("5 + 2- 3 *(5 + 10)/ 3;");
    expect_success("5 |> fn2;");
    expect_success("(fnid(x, y, z) + 5 / 2) |> pipeFn-2;");

    expect_fail("a |> 3;");
    expect_fail("(5 + 3;");
    expect_fail("fn + 5;");
    expect_fail("if + 5;");
    expect_fail("5 * 4 /3 |>for + int;");
}

#[test]
fn test_variables() {
    expect_success("let x:int=5;");
    expect_success("let x:int= 5;");
    expect_success("let x : int = 5;");
    expect_success("let df: DataFrame;");
    expect_success("let vec: int[1][1] = [[a]];");

    expect_fail("5 = 7;");
    expect_fail("let 3:int = 4;");
    expect_fail("var:float=5;");
    expect_fail("var = ;");
    expect_fail("let vec: int[1][1][3] = [[a]];");
    expect_fail("let vec: int[2][];");
    expect_fail("let vec: bool[][3];");
    expect_fail("let vec: bool[];");
}

#[test]
fn test_if() {
    expect_success(
        "
        if (x == 2) {
            x = 3;
            fncall();
        }
    ",
    );
    expect_success(
        "
        if(x == 2){
            x = 3;
            fncall();
        }else{
            x = 4;
        }
    ",
    );
    expect_success(
        "
        if(x == 2){
            x = 3;
            fncall();
        }else if(x == 4){
            x = 4;
        }else{
            x = 7;
        }
    "
    );
    expect_success(
        " 
        if (x == 2) {
            x = 3;
            fncall();
        } else if (x == 4) {
            x = 4;
        } else {
            x = 7;
        }
    "
    );

    expect_fail("if(x == 2) x = 3;");
    expect_fail(
        " 
        if (x == 2) {
            x = 3;
            fncall();
        } elseif (x == 4) {
            x = 4;
        } else {
            x = 7;
        }
    "
    );
    expect_fail(
        " 
        if (x == 2) {
            x = 3;
            fncall();
        }
        x = 4;
        else {
            x = 7;
        }
    "
    );
    expect_fail(
        " 
        if (x == 2) {
            x = 3;
            fncall();
        }
        else x = 7;
    "
    );
    expect_fail(
        " 
        if (x == 2) {
            let y:int = 4;
        }
    "
    );
}

#[test]
fn test_for() {
    expect_success("for (it in iterable) it = it + 1;");
    expect_success("for (it in iterable) {
        it = it + 1;    
    }");
    expect_success("for(it in iterable){
        it = it + 1;    
    }");
    expect_success("for(it in iterable) {}");


    expect_fail("in + 2;");
    expect_fail("in + 2;");
    expect_fail("for(it in iterable){
        let x:int = 3;
    }");
    expect_fail("for(let it in iterable){}");
    expect_fail("for(it initerable){}");
}
