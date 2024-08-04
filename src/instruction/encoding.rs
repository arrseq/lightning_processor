use std::io::{Read, Write};
use thiserror::Error;
use crate::instruction::{Instruction, operation};
use crate::instruction::operation::Operation;

#[derive(Debug, Error)]
pub enum OperationError {
    #[error("Could not handle lock modifier")]
    Lock,
    #[error("Could not handle vector operands modifier")]
    VectorOperands,
    #[error("Could not handle mapping vector components for operand {operand}")]
    MapVector { operand: usize },
    #[error("Could not set branch override to {r#override}")]
    BranchOverride { r#override: bool },
    #[error("Could not handle the primary operation")]
    Primary
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to decode an operation")]
    Operation { #[source] source: operation::encoding::DecodeError }
    // FIXME: Do we need more errors here? 
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("Failed to encode an operation")]
    Operation { 
        #[source] 
        source: operation::encoding::EncodeError,
        error: OperationError
    }
    // FIXME: Do we need more errors here? 
}

impl Instruction {
    pub fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let mut lock = false;
        let mut vector_operands = false;
        let mut vector_mappings = [None; 4];
        let mut branch_override = None;

        Ok(Self {
            operation: loop {
                let operation = Operation::decode(input).map_err(|source| DecodeError::Operation { source })?;

                match operation {
                    Operation::Lock => lock = true,
                    Operation::VectorOperands => vector_operands = true,
                    Operation::MapVector { mappings, operand } => vector_mappings[operand as usize] = Some(mappings),
                    Operation::TakeBranch => branch_override = Some(true),
                    Operation::IgnoreBranch => branch_override = Some(false),
                    _ => break operation
                }
            },
            lock, vector_operands, vector_mappings, branch_override
        })
    }
    
    pub fn encode(&self, output: &mut impl Write) -> Result<(), EncodeError> {
        // Encode modifier operations.
        if self.lock { Operation::Lock
                .encode(output)
                .map_err(|source| EncodeError::Operation { source, error: OperationError::Lock })?; }
        if self.vector_operands { Operation::VectorOperands
            .encode(output)
            .map_err(|source| EncodeError::Operation { source, error: OperationError::VectorOperands })?; }
        for (operand, mapping) in self.vector_mappings.iter().enumerate() { if let Some(mappings) = mapping {
            Operation::MapVector { mappings: *mappings, operand: operand as u8 }
                .encode(output)
                .map_err(|source| EncodeError::Operation { 
                    source, 
                    error: OperationError::MapVector { operand }
                })?;
        }}
        if let Some(branch_override) = self.branch_override {
            if branch_override {
                Operation::TakeBranch
                    .encode(output)
                    .map_err(|source| EncodeError::Operation { 
                        source, 
                        error: OperationError::BranchOverride { r#override: branch_override }
                    })?;
            } else {
                Operation::IgnoreBranch
                    .encode(output)
                    .map_err(|source| EncodeError::Operation {
                        source,
                        error: OperationError::BranchOverride { r#override: branch_override }
                    })?;
            }
        }
        
        self.operation
            .encode(output)
            .map_err(|source| EncodeError::Operation { 
                source,
                error: OperationError::Primary
            })?;
        
        Ok(())
    }
}