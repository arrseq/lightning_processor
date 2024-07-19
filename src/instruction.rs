use std::io;
use std::io::{Read, Write};
use crate::dynamic_number;
use crate::instruction::operand::dynamic::Dynamic;
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

/// The instruction was set to execute synchronously but there was no address operand. Synchronous execution is used
/// to access a paged address without a race condition.
#[derive(Debug)]
pub struct SynchronizedWithNoAddress;

#[derive(Debug)]
pub enum DecodeError {
    Prefix(prefix::DecodeError),
    Operands(operand::DecodeError),
    Operation(OperationError),
    SynchronizedWithNoAddress(SynchronizedWithNoAddress)
}

#[derive(Debug)]
pub enum EncodeError {
    Write(io::Error),
    Operands(operand::EncodeError),
    SynchronizedWithNoAddress(SynchronizedWithNoAddress)
}

#[derive(Debug)]
pub enum OperationError {
    Stream(io::Error),
    Operation(operation::InvalidCodeError)
}

impl Instruction {
    /// Synchronized instructions must have the dynamic operand point to an address.
    fn check_synchronous_error(execution: Option<prefix::Execution>, dynamic: Dynamic) -> Result<(), SynchronizedWithNoAddress> {
        // Has an execution mode override.
        if let Some(execution) = execution
            && let prefix::Execution::Synchronize = execution
            && !matches!(dynamic, Dynamic::Address(_)) { return Err(SynchronizedWithNoAddress) }
        Ok(())
    }
    
    pub fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let prefixes = Prefixes::decode(input).map_err(DecodeError::Prefix)?;
        let operation = Self::decode_operation(input, prefixes.escape).map_err(DecodeError::Operation)?;
        let operands = Operands::decode(input).map_err(DecodeError::Operands)?;

        Self::check_synchronous_error(prefixes.execution, operands.dynamic).map_err(DecodeError::SynchronizedWithNoAddress)?;
        
        Ok(Self {
            operands, operation,
            execution: prefixes.execution,
            branch_likely_taken: prefixes.branch_likely_taken
        })
    }
    
    fn decode_operation(input: &mut impl Read, escape: prefix::Escape) -> Result<Operation, OperationError> {
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

    /// # Note
    /// The output may be written to before an error is encountered thus leaving incoming encoded bytes on the output.
    pub fn encode(self, output: &mut impl Write) -> Result<(), EncodeError> {
        let encoded_operation = self.operation.encode();
        let operation_escape = Self::get_operation_escape(encoded_operation);

        let prefixes = Prefixes {
            escape: operation_escape,
            execution: self.execution,
            branch_likely_taken: self.branch_likely_taken
        };

        Self::check_synchronous_error(self.execution, self.operands.dynamic).map_err(EncodeError::SynchronizedWithNoAddress)?;

        prefixes
            .encode(output)
            .map_err(EncodeError::Write)?;
        Self::encode_operation(output, encoded_operation, operation_escape).map_err(EncodeError::Write)?;
        self.operands.encode(output).map_err(EncodeError::Operands)?;

        Ok(())
    }

    fn get_operation_escape(operation: u16) -> prefix::Escape {
        if operation > u8::MAX as u16 { return prefix::Escape::Word };
        prefix::Escape::Byte
    }

    fn encode_operation(output: &mut impl Write, operation: u16, escape: prefix::Escape) -> Result<(), io::Error> {
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