use moris_lang::{self, parser::grammar::ExprParser};

fn test_expr(parser: &ExprParser, in_str: &str, test_str: &str) {
    assert_eq!(&format!("{:?}", parser.parse(in_str).unwrap()), test_str);
}

#[test]
fn grammar_test() {
    // let target = "26 * (55.2 - 3 / 4) + varname - 54 |> function |> forward function2 |> f3 && 54 > 57 && (33 / 32 - 34) * 17 > 544 - 2 && true";
    // let target = "[43, 31, 4,4, 4] * 2";
    let target = "[43.3, 43,varid] |> func |> 34";
    let parser = ExprParser::new();

    // Constants

    test_expr(&parser, "54", "54");
    test_expr(&parser, "54.3", "54.3");
    test_expr(&parser, "true", "true");
    test_expr(&parser, "[32, 34]", "[32, 34]");
    test_expr(&parser, r#""some string""#, "some string");


    test_expr(&parser, "[32, 34]", "[32,34]");

}
