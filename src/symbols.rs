use std::{collections::HashMap};

use moris_lang::ast;

pub enum Symbol {
    Variable(ast::Variable),
    Function(ast::FunctionSignature),
}

pub struct SymbolTable<'parent> {
    parent: Option<&'parent SymbolTable<'parent>>,
    table: HashMap<String, Symbol>,
}

impl<'parent> SymbolTable<'parent> {
    pub fn new() -> SymbolTable<'parent> {
        SymbolTable {
            parent: None,
            table: HashMap::new(),
        }
    }

    pub fn set(&mut self, id: &str, sym: Symbol) {
        if let Some(_) = self.table.get(id) {
            panic!("{} is already defined!", id);
        } else {
            self.table.insert(String::from(id), sym);
        }
    }

    pub fn get(&'parent self, id: &str) -> Option<&'parent Symbol> {
        if let Some(symbol) = self.table.get(id) {
            return Some(symbol);
        } else {
            let mut parent = self.parent;
            while !parent.is_none() {
                if let Some(symbol) = parent.unwrap().get(id) {
                    return Some(symbol);
                }
                parent = parent.unwrap().parent;
            }
            panic!("{} is not defined!", id);
        }
    }
}

impl<'parent> Default for SymbolTable<'parent> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_symbol() {
        let mut table = SymbolTable::new();
        let sym = Symbol::Variable(ast::Variable {
            data_type: ast::DataType::Bool,
            id: String::from("is_cond"),
            dimension: ast::Dimension(0, vec![]),
            value: None,
        });

        table.set("is_cond", sym);
        match table.get("is_cond") {
            Some(Symbol::Variable(var)) => assert_eq!(var.id, "is_cond"),
            _ => (),
        }
    }
}
