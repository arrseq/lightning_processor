use std::io::Read;
use thiserror::Error;
use crate::instruction::operation::{Operation, VectorComponent};

pub mod operation;
pub mod operand;
mod modifier;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub operation: Operation,
    pub lock: bool,
    pub vector_operands: bool,
    pub vector_mapping: Option<[Option<VectorComponent>; 4]>,
    pub branch_override: Option<bool>,
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to decode the operation")]
    Operation { #[source] source: operation::encoding::DecodeError }
}

impl Instruction {
    pub fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let mut lock = false;
        let mut vector_operands = false;
        let mut vector_mapping = None;
        let mut branch_override = None;
        
        Ok(Self {
            lock, vector_operands, vector_mapping, branch_override,
            operation: Operation::decode(input).map_err(|source| DecodeError::Operation { source })?
        })
    }
}