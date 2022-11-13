use crate::{
    codegen::{manager::GlobalManager, quadruples::Quadruple},
    memory::types::DataType,
};

use super::{node::Node, statements::Block, types::Variable};

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub id: String,
    pub data_type: DataType,
    pub params: Vec<FunctionParam>,
}

#[derive(Debug, Clone)]
pub struct FunctionParam(pub Variable);

#[derive(Debug)]
pub struct Function {
    pub signature: FunctionSignature,
    pub block: Block,
}

impl Function {
    pub fn new(signature: FunctionSignature, block: Block) -> Function {
        Function { signature, block }
    }
}

impl Node for Function {
    fn generate(&mut self) -> () {
        let mut manager = GlobalManager::get();
        let next_position = manager.get_next_id();

        manager.update_func_position(&self.signature.id, next_position);
        manager.get_env().switch(&self.signature.id);

        drop(manager);

        self.block.generate();

        GlobalManager::emit(Quadruple::new_coded("endfunc"));

        GlobalManager::get()
            .get_env()
            .switch(&String::from("global"));
        // GlobalManager::get().get_env().drop_env(&self.signature.id);
    }

    fn reduce(&self) -> String {
        todo!("Function reduce!");
    }
}
