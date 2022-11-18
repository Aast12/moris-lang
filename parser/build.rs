extern crate lalrpop;

#[allow(unused_must_use)]
fn main() {
    // lalrpop::process_root().unwrap();
    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .process();
}
