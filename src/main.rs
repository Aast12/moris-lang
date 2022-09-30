#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar);
pub mod ast;

#[test]
fn grammar() {
    let target = "(26 * (55 - 3 / 4) + 326 - 54) > 34 && 54 > 57 && (33 / 32 - 34) * 17 > 544 - 2 ";

    let expr = grammar::ExprParser::new()
        .parse(&target)
        .unwrap();

    assert_eq!(&format!("{:?}", expr), &target);
    // println!("{:?}", expr);
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}