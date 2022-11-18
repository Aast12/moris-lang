use moris_lang::vm::test::Inspector;

fn main() {
    println!("samples/pipe.mo");
    let inspector = Inspector::new("samples/pipe.mo");
    println!("input = {:#?}", inspector.get("input"));
    println!("res = {:#?}", inspector.get("res"));

    println!("samples/fibonacci_memo.mo");
    let inspector = Inspector::new("samples/fibonacci_memo.mo");

    println!("i = {:#?}", inspector.get("i"));
    println!("mem = {:#?}", inspector.get("mem"));
    println!("y = {:#?}", inspector.get("y"));

    println!("samples/fibonacci.mo");
    let inspector = Inspector::new("samples/fibonacci.mo");

    println!("x = {:#?}", inspector.get("x"));
    println!("y = {:#?}", inspector.get("y"));
    println!("z = {:#?}", inspector.get("z"));
    println!("result = {:#?}", inspector.get("result"));

    println!("samples/expressions.mo");
    let inspector = Inspector::new("samples/expressions.mo");

    println!("x = {:#?}", inspector.get("x"));
    println!("y = {:#?}", inspector.get("y"));
    println!("s = {:#?}", inspector.get("s"));
    println!("q = {:#?}", inspector.get("q"));
}
