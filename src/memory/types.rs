#[derive(Eq, Debug, PartialEq, Hash, Clone, Copy)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
    DataFrame,
}
