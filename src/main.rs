use moris_lang::vm::test::Inspector;

fn main() {
    let inspector = Inspector::new("samples/fibonacci.mo");

    println!("i = {:#?}", inspector.get("i"));
    println!("mem = {:#?}", inspector.get("mem"));
    println!("y = {:#?}", inspector.get("y"));
}
