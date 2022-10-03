use moris_lang::{self, parser::grammar::{ExprParser, DimensionParser, VarDeclarationParser, VarReferenceParser, VarAssignmentParser, FunctionParamsParser, FnSignatureParser, StatementParser}, ast::{Expr, Variable, Dimension}};
use moris_lang::ast;
fn test_expr_eq(parser: &ExprParser, in_str: &str, test_str: &str) {
    parser.parse(in_str).unwrap();

    // assert_eq!(&format!("{:?}", parser.parse(in_str).unwrap()), test_str);
}
fn test_expr_eq_(parser: &ExprParser, in_str: &str, test_str: &str) {
    // parser.parse(in_str).unwrap();

    assert_eq!(&format!("{:?}", parser.parse(in_str).unwrap()), test_str);
}


fn test_expr_fail(parser: &ExprParser, in_str: &str) {
    assert!(std::panic::catch_unwind(|| parser.parse(in_str).unwrap()).is_err());
}

#[test]
fn grammar_test() {
    let parser = ExprParser::new();
    let x = ast::TypeConst::Bool(false);
    // Constants

    // let dimparser = DimensionParser::new().parse("[5][3 + 2]").unwrap();
    // assert_eq!(&format!("{:?}", dimparser), "2");
    // format!("{:?}", StatementParser::new().parse(r#"x = 7;"#).unwrap());
    
    assert_eq!(&format!("{:?}", StatementParser::new().parse(r#"someFn(5, x) + 5;"#).unwrap()), "55");
    
    assert_eq!(&format!("{:?}", StatementParser::new().parse(r#"for(i in iter){
        if (2){

        }else if(3) {
            if (4) {
                st4;
            }
            st3_4;
        }
    }"#).unwrap()), "55");
    
    assert_eq!(&format!("{:?}", StatementParser::new().parse(r#"while(scope1) {
        if (2){

        } else if(3) {
            if (4) {
                st4;
            }
            st3_4;
        }
    }"#).unwrap()), "55");
    
    assert_eq!(&format!("{:?}", StatementParser::new().parse(r#"if (scope1) {
        if (2){

        } else if(3) {
            if (4) {
                st4;
            }
            st3_4;
        }
    } else {
        else1;       
    }"#).unwrap()), "55");
    // println!("{:?}", parser.parse(r#""5 * 2 && 5" * 45 "#).unwrap());
    // assert_eq!(&format!("{:?}", FnSignatureParser::new().parse("fn somefn(x: int, y: float): bool").unwrap()), "55");
    FnSignatureParser::new().parse("fn somefn(x: int, y: float): bool").unwrap();
    // test_expr_eq(&parser, r#""5 * 2 && 5" * 45 "#, "54");
    // test_expr_eq(FunctionParamsParser::new(), "x: int, y: float", "54");
    // assert_eq!(&format!("{:?}", FunctionParamsParser::new().parse("x: int, y: float").unwrap()), "55");
    FunctionParamsParser::new().parse("x: int, y: float").unwrap();
    

    let varassignparser =VarAssignmentParser::new().parse("x[3] = 5313 * 12414 + 1234124 |> fb3 && false").unwrap();
    // assert_eq!(&format!("{:?}", varassignparser), "2");

    let varassignparser =ExprParser::new().parse("x[3][3] - 5313 * 12414 + 1234124 |> fn6 && false").unwrap();
    // assert_eq!(&format!("{:?}", varassignparser), "2");

    let varrefparser = VarReferenceParser::new().parse("someVarname[2: 5][4 / 32 * 5 |> fn2: 6]").unwrap();
    // assert_eq!(&format!("{:?}", varrefparser), "2");
    
    let varparsed = VarDeclarationParser::new().parse("let myvar: int[2 * 6][7] = 7 * 6 * 2 + 2 > 3 && 5 |> fn3 / 3 || false;").unwrap();
    // assert_eq!(&format!("{:?}", varparsed), "2");
    
    test_expr_eq(&parser, "54", "54");
    test_expr_eq(&parser, "54.3", "54.3");
    test_expr_eq(&parser, "true", "true");
    test_expr_eq(&parser, "[32, 34]", "[32, 34]");
    test_expr_eq(&parser, "[[32, 34], [32, 34]]", "[[32, 34], [32, 34]]");
    test_expr_eq(&parser, r#""some string""#, "some string");
    

    // Pipes
    
    test_expr_eq(
        &parser,
        "[43.3, 43, varid] |> func |> fun2",
        "[43.3, 43, varid] |> func |> fun2",
    );

    test_expr_fail(&parser, "45 |> func |> func3 |> 3");
    test_expr_fail(&parser, "45 |> func |> 3 |> func3");
}
