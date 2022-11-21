use memory::{resolver::MemAddress, types::DataType};

use crate::manager::Manager;

/// Trait for all nodes representing accessable items (variables).
pub trait AccessNode {
    /// Returns the address of the current item
    fn address(&self, _: &mut Manager) -> MemAddress {
        todo!();
    }
}

/// Trait for all nodes representing expressions
pub trait ExpressionNode {
    /// Returns the dimensionality/shape of an item returned
    /// by a node (if applicable), e.g. expressions.
    fn dimensionality(&self, _: &mut Manager) -> Vec<usize> {
        vec![]
    }

    /// Returns de data type of the node evaluated item,
    /// if applicable
    fn data_type(&self, _: &mut Manager) -> DataType {
        todo!()
    }

    /// Generates the quadruples to execute the code of the
    /// current node. Returns the address where the evaluation
    /// of the code from this node will be stored.
    fn reduce(&self, _: &mut Manager) -> String;
}

/// Trait to represent nodes of an Abstract Syntax Tree.
///
/// Implements needed methods for each node to generate
/// code for its children. A manager to generate queadruples
/// is propagated to each method.
///
pub trait Node {
    /// Generates the quadruples to execute the code of the
    /// current node.
    fn generate(&mut self, manager: &mut Manager) -> ();
}
