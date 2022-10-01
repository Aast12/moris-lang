use moris_lang;

#[test]
fn grammar_test() {
    let target = "26 * (55.2 - 3 / 4) + varname - 54 |> function |> forward function2 |> f3 && 54 > 57 && (33 / 32 - 34) * 17 > 544 - 2";
    let factor_target = "5 |> function";

    let expr = moris_lang::parser::grammar::ExprParser::new()
        .parse(&target)
        .unwrap();

    let strexpr = moris_lang::parser::grammar::ExprParser::new()
        .parse(r#" "sierl ero" + "sun eeii ""#)
        .unwrap();

    // assert_eq!(&format!("{:?}", strexpr), "faiñ");

    assert_eq!(&format!("{:?}", expr), &target);

    // let expr = grammar::FactorParser::new().parse(&factor_target).unwrap();
    // assert_eq!(&format!("{:?}", expr), &factor_target)

    // println!("{:?}", grammar::NumParser::new().parse("12355").unwrap());

    // assert_eq!(&format!("{:?}", expr), &target);
    // println!("{:?}", expr);
}
