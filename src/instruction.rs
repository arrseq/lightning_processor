use std::io;
use std::io::Read;
use crate::dynamic_number;
use crate::operand::Operands;
use crate::operation::Operation;
use crate::prefix::Prefixes;

pub mod operand;
pub mod operation;
pub mod prefix;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub execution: Option<prefix::Execution>,
    pub branch_likely_taken: Option<bool>,
    pub operands: Operands,
    pub operation: Operation
}

#[derive(Debug)]
pub enum DecodeError {
    Prefix(prefix::DecodeError),
    Operands(operand::DecodeError),
    Operation(OperationError)
}

#[derive(Debug)]
pub enum OperationError {
    Stream(io::Error),
    Operation(operation::InvalidCodeError)
}

impl Instruction {
    pub fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let prefixes = Prefixes::decode(input).map_err(DecodeError::Prefix)?;
        let operation = Self::decode_operation(input, prefixes.escape).map_err(DecodeError::Operation)?;
        let operands = Operands::decode(input).map_err(DecodeError::Operands)?;
        
        Ok(Self {
            operands, operation,
            execution: prefixes.execution,
            branch_likely_taken: prefixes.branch_likely_taken
        })
    }
    
    pub fn decode_operation(input: &mut impl Read, escape: prefix::Escape) -> Result<Operation, OperationError> {
        Ok(match escape {
            prefix::Escape::Byte => {
                let mut buffer = [0u8; 1];
                input.read_exact(&mut buffer).map_err(OperationError::Stream)?;
                Operation::decode(buffer[0] as u16).map_err(OperationError::Operation)?
            },
            prefix::Escape::Word => {
                let mut buffer = [0u8; dynamic_number::Size::WORD_BYTES];
                input.read_exact(&mut buffer).map_err(OperationError::Stream)?;
                Operation::decode(u16::from_le_bytes(buffer)).map_err(OperationError::Operation)?
            }
        })
    }
}