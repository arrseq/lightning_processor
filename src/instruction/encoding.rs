use std::io::{Read, Write};
use thiserror::Error;
use crate::instruction::{Instruction, operation};
use crate::instruction::operation::Operation;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to decode an operation")]
    Operation { #[source] source: operation::encoding::DecodeError }
    // FIXME: Do we need more errors here? 
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("Failed to encode an operation")]
    Operation { #[source] source: operation::encoding::EncodeError }
    // FIXME: Do we need more errors here? 
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
    
    pub fn encode(&self, output: &mut impl Write) -> Result<(), EncodeError> {
        // Encode modifier operations.
        if self.lock { Operation::Lock
                .encode(output)
                .map_err(|source| EncodeError::Operation { source })?; }
        
        Ok(())
    }
}