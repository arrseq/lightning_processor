use std::io::Read;
use thiserror::Error;
use crate::instruction::operation::{Operation, VectorComponent};

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
    pub vector_mapping: [Option<VectorMapping>; 4],
    pub branch_override: Option<bool>,
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to decode an operation")]
    Operation { #[source] source: operation::encoding::DecodeError },
}

impl Instruction {
    pub fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let mut lock = false;
        let mut vector_operands = false;
        let mut vector_mapping = [None; 4];
        let mut branch_override = None;

        Ok(Self {
            operation: loop {
                let operation = Operation::decode(input).map_err(|source| DecodeError::Operation { source })?;

                match operation {
                    Operation::Lock => lock = true,
                    Operation::VectorOperands => vector_operands = true,
                    Operation::MapVector { mappings, operand } => vector_mapping[operand as usize] = Some(mappings),
                    Operation::TakeBranch => branch_override = Some(true),
                    Operation::IgnoreBranch => branch_override = Some(false),
                    _ => break operation
                }
            },
            lock, vector_operands, vector_mapping, branch_override
        })
    }
}