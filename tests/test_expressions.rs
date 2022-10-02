use moris_lang::{self, parser::grammar::{ExprParser, DimensionParser, VarDeclarationParser}, ast::{Expr, Variable, Dimension}};

fn test_expr_eq(parser: &ExprParser, in_str: &str, test_str: &str) {
    assert_eq!(&format!("{:?}", parser.parse(in_str).unwrap()), test_str);
}

fn test_expr_fail(parser: &ExprParser, in_str: &str) {
    assert!(std::panic::catch_unwind(|| parser.parse(in_str).unwrap()).is_err());
}

#[test]
fn grammar_test() {
    let parser = ExprParser::new();

    // Constants

    // let dimparser = DimensionParser::new().parse("[5][3 + 2]").unwrap();
    // assert_eq!(&format!("{:?}", dimparser), "2");

    let varparsed = VarDeclarationParser::new().parse("let myvar: int[2 * 6][7]").unwrap();
    assert_eq!(&format!("{:?}", varparsed), "2");
    
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
