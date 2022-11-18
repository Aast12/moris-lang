use moris_lang::vm::{test::Inspector, memory_manager::Item};

fn main() {
    println!("samples/arrays.mo");
    let inspector = Inspector::new("samples/arrays.mo");
    println!("x = {:#?}", inspector.get("x"));
    println!("y = {:#?}", inspector.get("y"));
    println!("z = {:#?}", inspector.get("z"));
    // println!("mem = {:#?}", inspector.get("mem"));
    // assert_eq!(inspector.get("x"), Item::Int(125));

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
#[cfg(test)]
mod tests {
    use moris_lang::vm::test::Inspector;

    #[test]
    fn test_arrays() {
        println!("samples/arrays.mo");
        let inspector = Inspector::new("samples/arrays.mo");
        println!("x = {:#?}", inspector.get("x"));
        println!("mem = {:#?}", inspector.get("mem"));
        // assert_eq!(inspector.get("x"), Item::Int(125));
    }

    #[test]
    fn test_pipe() {
        println!("samples/pipe.mo");
        let inspector = Inspector::new("samples/pipe.mo");
        println!("input = {:#?}", inspector.get("input"));
        println!("res = {:#?}", inspector.get("res"));
    }

    #[test]
    fn test_fibonacci_memo() {
        println!("samples/fibonacci_memo.mo");
        let inspector = Inspector::new("samples/fibonacci_memo.mo");
        println!("i = {:#?}", inspector.get("i"));
        println!("mem = {:#?}", inspector.get("mem"));
        println!("y = {:#?}", inspector.get("y"));
    }

    #[test]
    fn test_fibonacci() {
        println!("samples/fibonacci.mo");
        let inspector = Inspector::new("samples/fibonacci.mo");

        println!("x = {:#?}", inspector.get("x"));
        println!("y = {:#?}", inspector.get("y"));
        println!("z = {:#?}", inspector.get("z"));
        println!("result = {:#?}", inspector.get("result"));
    }

    #[test]
    fn test_expressions() {
        println!("samples/expressions.mo");
        let inspector = Inspector::new("samples/expressions.mo");

        println!("x = {:#?}", inspector.get("x"));
        println!("y = {:#?}", inspector.get("y"));
        println!("s = {:#?}", inspector.get("s"));
        println!("q = {:#?}", inspector.get("q"));
    }
}
