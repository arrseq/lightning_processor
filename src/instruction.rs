use std::io;
use std::io::{Read, Write};
use crate::dynamic_number;
use crate::instruction::operand::dynamic::Dynamic;
use crate::instruction::operand::Meta;
use self::operand::Operands;
use self::operation::Operation;
use self::prefix::Prefixes;

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
pub enum EncodeError {
    Write(io::Error),
    Operands(operand::EncodeError),
    
    /// The instruction was set to execute synchronously but there was no address operand. Synchronous execution is used
    /// to access a memory address without a race condition.
    SynchronizedWithNoAddress
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
    
    pub fn encode(self, output: &mut impl Write) -> Result<(), EncodeError> {
        let encoded_operation = self.operation.encode();
        let operation_escape = Self::get_operation_escape(encoded_operation);
        
        let prefixes = Prefixes {
            escape: operation_escape,
            execution: self.execution,
            branch_likely_taken: self.branch_likely_taken
        };
        
        // Synchronized instructions must have the dynamic operand point to an address.
        if
            // Has an execution mode override.
            let Some(execution) = self.execution
            // The execution mode is synchronous.
            && let prefix::Execution::Synchronize = execution
            // The dynamic addressing mode is not address.
            && !matches!(self.operands.dynamic, Dynamic::Address(_)) { return Err(EncodeError::SynchronizedWithNoAddress) }
        
        prefixes
            .encode(output)
            .map_err(EncodeError::Write)?;
        Self::encode_operation(output, encoded_operation, operation_escape).map_err(EncodeError::Write)?;
        self.operands.encode(output).map_err(EncodeError::Operands)?;
        
        Ok(())
    }
    
    pub fn get_operation_escape(operation: u16) -> prefix::Escape {
        if operation > u8::MAX as u16 { return prefix::Escape::Word };
        prefix::Escape::Byte
    }
    
    pub fn encode_operation(output: &mut impl Write, operation: u16, escape: prefix::Escape) -> Result<(), io::Error> {
        match escape {
            prefix::Escape::Byte => {
                let buffer = [operation as u8];
                output.write_all(&buffer)?;
            },
            prefix::Escape::Word => {
                let buffer = operation.to_le_bytes();
                output.write_all(&buffer)?;
            }
        }
        
        Ok(())
    }
}