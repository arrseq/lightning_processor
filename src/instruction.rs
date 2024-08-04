use crate::instruction::operation::{Operation, VectorComponent};

pub mod encoding;
pub mod operation;
pub mod operand;

#[cfg(test)]
mod test;

pub type VectorMapping = [Option<VectorComponent>; 4];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub operation: Operation,
    pub lock: bool,
    pub vector_operands: bool,
    /// List of vector remapping modifiers.
    ///
    /// [None] as an element specifies that the operand does not have a modifier.
    pub vector_mappings: [Option<VectorMapping>; 4],
    pub branch_override: Option<bool>,
}

impl Instruction {
    /// Create a new instruction with no modifiers.
    pub const fn new(operation: &Operation) -> Self {
        Self {
            operation: *operation,
            lock: false,
            vector_operands: false,
            vector_mappings: [None; 4],
            branch_override: None
        }
    }
}
