use std::io::Read;
use thiserror::Error;
use crate::instruction::operation::Operation;

pub mod operation;
pub mod operand;
mod modifier;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub operation: Operation,
    pub lock: bool,
    pub vector_operands: bool,
    pub vector_mapping: bool,
    pub branch_override: Option<bool>,
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to decode the operation")]
    Operation { #[source] source: operation::encoding::DecodeError }
}

impl Instruction {
    pub fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(Self {
            operation: Operation::decode(input).map_err(|source| DecodeError::Operation { source })?,
            lock: false,
            vector_operands: false,
            vector_mapping: false,
            branch_override: None
        })
    }
}