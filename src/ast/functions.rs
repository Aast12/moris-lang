use crate::memory::types::DataType;

use super::{node::Node, quadruples::{GlobalManager, Quadruple}, statements::Block};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub id: String,
    pub data_type: DataType,
    pub params: Vec<FunctionParam>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParam(pub String, pub DataType);

#[derive(Debug)]
pub struct Function {
    pub signature: FunctionSignature,
    pub block: Block,
}

impl<'m> Function {
    pub fn new(signature: FunctionSignature, block: Block) -> Function {
        Function { signature, block }
    }
}

impl<'m> Node<'m> for Function {
    fn generate(&mut self) -> () {
        let mut manager = GlobalManager::get();
        let next_position = manager.get_next_id();

        let return_address = match &self.signature.data_type {
            DataType::Void => None,
            _ => Some(manager.new_global(&self.signature.data_type)),
        };

        manager.new_func(self, next_position, return_address);

        drop(manager);

        self.block.generate();

        GlobalManager::emit(Quadruple::new_coded("endfunc"));

        GlobalManager::get()
            .get_env()
            .switch(&String::from("global"));
        GlobalManager::get().get_env().drop_env(&self.signature.id);
    }

    fn reduce(&self) -> String {
        todo!("Function reduce!");
    }
}
