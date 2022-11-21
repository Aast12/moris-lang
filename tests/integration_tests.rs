use moris_lang::vm::{memory_manager::Item, test::Inspector};
use serial_test::file_serial;

fn test_file(file_name: &str) -> Inspector {
    let file_path = build_file_path(file_name);
    let inspector = Inspector::new(file_path.as_str());
    println!("Test file {}:", file_path);
    inspector.debug();
    inspector
}

fn build_file_path(file_name: &str) -> String {
    format!("samples/{file_name}").to_owned()
}

#[test]
#[file_serial]
fn test_pipes() {
    println!("STARTING pipe");
    let data = test_file("pipe.mo");
    assert_eq!(data.get("res"), Item::Int(93712));
    assert_eq!(data.get("input"), Item::Int(1220));
}

#[test]
#[file_serial]
fn test_fibonacci_recursive() {
    println!("STARTING fibonacci");
    let data = test_file("fibonacci.mo");
    assert_eq!(data.get("x"), Item::Int(7));
    assert_eq!(data.get("y"), Item::Float(6.0));
    assert_eq!(data.get("z"), Item::Float(42.0));
    assert_eq!(data.get("result"), Item::Int(89));
}

#[test]
#[file_serial]
fn test_fibonacci_memo() {
    println!("STARTING fibonacci_memo");
    let data = test_file("fibonacci_memo.mo");
    assert_eq!(data.get("i"), Item::Int(500));
    assert_eq!(data.get("recursive_result"), Item::Int(12586269025));
    assert_eq!(data.get("iter_result"), Item::Int(12586269025));
}

#[test]
#[file_serial]
fn test_expressions() {
    println!("STARTING expressions");
    let _ = test_file("expressions.mo");

    // TODO: assert values
}

#[test]
#[file_serial]
fn test_arrays() {
    println!("STARTING arrays");
    let data = test_file("arrays.mo");
    assert_eq!(data.get("i"), Item::Int(10));
    assert_eq!(data.get("j"), Item::Int(10));
    // TODO: assert values
}
