/// Represents the node of an Abstract Syntax Tree
pub trait Node<'m> {
    fn generate(&mut self) -> () {
        self.reduce();
    }

    fn reduce(&self) -> String {
        todo!("reduce raw node");
    }
}
