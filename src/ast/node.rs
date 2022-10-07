use crate::ast::quadruples;

/// Represents the node of an Abstract Syntax Tree
pub trait Node {
    fn set_manager(&self, manager: &quadruples::Manager) -> ();
    fn generate(&self) -> ();
    fn rvalue(&self) -> dyn Node;
    fn lvalue(&self) -> dyn Node;
    fn emit(&self) -> ();
}
