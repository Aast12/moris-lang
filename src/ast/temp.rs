use super::{node::Node, types::DataType};

pub struct Temp {
    pub id: i32,
    pub data_type: DataType,
}

impl Temp {
    pub fn new(id: i32, data_type: DataType) -> Temp {
        Temp { id, data_type }
    }
}

impl<'m> Node<'m> for Temp {
    fn generate(&mut self) -> () {
        panic!()
    }

    fn reduce(&self) -> String {
        return format!("tmp{}", self.id.to_string());
    }
}
