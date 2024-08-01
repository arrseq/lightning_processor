use std::io::Read;
use thiserror::Error;
use crate::instruction::operation::Operation;

pub mod operation;
pub mod operand;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub operation: Operation
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to decode the operation")]
    Operation { #[source] source: operation::encoding::DecodeError }
}

impl Instruction {
    pub fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        Ok(Self {
            operation: Operation::decode(input).map_err(|source| DecodeError::Operation { source })?
        })
    }
}