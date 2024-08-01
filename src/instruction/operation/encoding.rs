#[cfg(test)]
mod test;

use std::io;
use std::io::{Read, Write};
use thiserror::Error;
use crate::instruction::{operand, operation};
use crate::instruction::operand::Operand;
use crate::instruction::operation::{OperandCategory, DestinationAndDualInput, DestinationAndInput, DualInput, Input, Operation, Destination};
use crate::math::dynamic_number;
use crate::math::dynamic_number::Unsigned;

impl Destination {
    // Implement from_code to convert a code into a Destination, returning None if it doesn't match.
    const fn from_code(code: u16) -> Option<Self> {
        Some(match code {
            Self::UNSTACK => Self::Unstack,
            _ => return None,
        })
    }

    // Implement to_code to convert a Destination into its corresponding code.
    const fn to_code(self) -> u16 {
        match self {
            Self::Unstack => Self::UNSTACK,
        }
    }
}

impl Input {
    // Implement from_code to convert a code into an Input, returning None if it doesn't match.
    const fn from_code(code: u16) -> Option<Self> {
        Some(match code {
            Self::STACK => Self::Stack,
            _ => return None,
        })
    }

    // Implement to_code to convert an Input into its corresponding code.
    const fn to_code(self) -> u16 {
        match self {
            Self::Stack => Self::STACK,
        }
    }
}

impl DestinationAndInput {
    // Implement from_code to convert a code into a DestinationAndInput, returning None if it doesn't match.
    const fn from_code(code: u16) -> Option<Self> {
        Some(match code {
            Self::COPY => Self::Copy,
            _ => return None,
        })
    }

    // Implement to_code to convert a DestinationAndInput into its corresponding code.
    const fn to_code(self) -> u16 {
        match self {
            Self::Copy => Self::COPY,
        }
    }
}

impl DualInput {
    // Implement from_code to convert a code into a DualInput, returning None if it doesn't match.
    const fn from_code(code: u16) -> Option<Self> {
        Some(match code {
            Self::COMPARE => Self::Compare,
            Self::SIGNED_COMPARE => Self::SignedCompare,
            _ => return None,
        })
    }

    // Implement to_code to convert a DualInput into its corresponding code.
    const fn to_code(self) -> u16 {
        match self {
            Self::Compare => Self::COMPARE,
            Self::SignedCompare => Self::SIGNED_COMPARE,
        }
    }
}

impl DestinationAndDualInput {
    // Implement from_code to convert a code into a DestinationAndDualInput, returning None if it doesn't match.
    pub const fn from_code(code: u16) -> Option<Self> {
        Some(match code {
            Self::ADD => Self::Add,
            Self::FLOATING_ADD => Self::FloatingAdd,
            Self::SUBTRACT => Self::Subtract,
            Self::FLOATING_SUBTRACT => Self::FloatingSubtract,
            Self::MULTIPLY => Self::Multiply,
            Self::FLOATING_MULTIPLY => Self::FloatingMultiply,
            Self::DIVIDE => Self::Divide,
            Self::FLOATING_DIVIDE => Self::FloatingDivide,
            _ => return None,
        })
    }

    // Implement to_code to convert a DestinationAndDualInput into its corresponding code
    pub const fn to_code(self) -> u16 {
        match self {
            Self::Add => Self::ADD,
            Self::FloatingAdd => Self::FLOATING_ADD,
            Self::Subtract => Self::SUBTRACT,
            Self::FloatingSubtract => Self::FLOATING_SUBTRACT,
            Self::Multiply => Self::MULTIPLY,
            Self::FloatingMultiply => Self::FLOATING_MULTIPLY,
            Self::Divide => Self::DIVIDE,
            Self::FloatingDivide => Self::FLOATING_DIVIDE,
        }
    }
}

#[derive(Debug, Error)]
pub enum OperandError {
    #[error("Failed to perform operation on destination")]
    Destination,
    #[error("Failed to perform operation on input {nth}")]
    Input { nth: u8 }
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to read operation specifier code")]
    Chain { #[source] source: io::Error },
    #[error("The operation code was not recognized")]
    InvalidOperation,
    #[error("Failed to retrieve operand")]
    Operand { #[source] source: operand::encoding::Error, error: OperandError }
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("Failed to write operation specifier code")]
    Chain { #[source] source: io::Error },
    #[error("Failed to encode operand")]
    Operand { #[source] source: operand::encoding::Error, error: OperandError }
}

impl Operation {
    /// The maximum number of bytes an operation can be in the chain length encoding.
    pub const MAX_OPERATION_LENGTH: u8 = 2;
    
    pub(crate) fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let code = Unsigned::decode_chain(input, Some(Self::MAX_OPERATION_LENGTH as u64)).map_err(|source| DecodeError::Chain { source })?.value as u16;
        let category = Self::OPERATIONS.get(code as usize).ok_or(DecodeError::InvalidOperation)?.category;
        
        // The ends of the statements are marked unreachable in the match because the codes will always be valid for 
        // their operand types.
        if let Some(category) = category { return Ok(match category {
            OperandCategory::Destination => Operation::Destination {
                operation: Destination::from_code(code).unwrap(),
                destination: Self::decode_operand(input, OperandError::Destination)?
            },
            OperandCategory::Input => Operation::Input {
                operation: Input::from_code(code).unwrap(),
                input: Self::decode_operand(input, OperandError::Input { nth: 0 })?
            },
            OperandCategory::DestinationAndInput => Operation::DestinationAndInput {
                operation: DestinationAndInput::from_code(code).unwrap(),
                destination: Self::decode_operand(input, OperandError::Destination)?,
                input: Self::decode_operand(input, OperandError::Input { nth: 0 })?
            },
            OperandCategory::DualInput => Operation::DualInput {
                operation: DualInput::from_code(code).unwrap(),
                inputs: [
                    Self::decode_operand(input, OperandError::Input { nth: 0 })?,
                    Self::decode_operand(input, OperandError::Input { nth: 1 })?
                ]
            },
            OperandCategory::DestinationAndDualInput => Operation::DestinationAndDualInput {
                operation: DestinationAndDualInput::from_code(code).unwrap(),
                destination: Self::decode_operand(input, OperandError::Destination)?,
                inputs: [
                    Self::decode_operand(input, OperandError::Input { nth: 0 })?,
                    Self::decode_operand(input, OperandError::Input { nth: 1 })?
                ]
            }
        }); }
        
        Ok(Self::from_no_operand_code(code).ok_or(DecodeError::InvalidOperation)?)
    }
    
    const fn from_no_operand_code(code: u16) -> Option<Self> {
        Some(match code {
            Self::NONE_CODE => Self::None,
            _ => return None
        })
    }
    
    fn decode_operand(input: &mut impl Read, error: OperandError) -> Result<Operand, DecodeError> {
        Operand::decode(input).map_err(|source| DecodeError::Operand { source, error })
    }
    
    const fn encode_operation(self) -> u16 {
        match self {
            Self::None => Self::NONE.code,
            Self::Destination { operation, .. } => operation.to_code(),
            Operation::Input { operation, .. } => operation.to_code(),
            Operation::DestinationAndInput { operation, .. } => operation.to_code(),
            Operation::DualInput { operation, .. } => operation.to_code(),
            Operation::DestinationAndDualInput { operation, .. } => operation.to_code(),
        }
    }

    fn encode_operand(output: &mut impl Write, operand: Operand, error: OperandError) -> Result<(), EncodeError> {
        operand.encode(output).map_err(|source| EncodeError::Operand { source, error })
    }
    
    fn encode_inputs<const Count: usize>(output: &mut impl Write, inputs: [Operand; Count]) -> Result<(), EncodeError> {
        for (nth, input) in inputs.iter().enumerate() { Self::encode_operand(output, *input, OperandError::Input { nth: nth as u8 })?; }
        Ok(())
    }
    
    fn encode_destination(output: &mut impl Write, destination: Operand) -> Result<(), EncodeError> {
        Self::encode_operand(output, destination, OperandError::Destination)
    }
    
    pub(crate) fn encode(self, output: &mut impl Write) -> Result<(), EncodeError> {
        let operation = self.encode_operation();
        Unsigned::new(operation as u64)
            .encode_chain(output, true)
            .map_err(|source| EncodeError::Chain { source })?;
        
        match self {
            Self::Destination { destination, .. } => Self::encode_destination(output, destination)?,
            Self::Input { input, .. } => Self::encode_inputs(output, [ input ])?,
            Self::DestinationAndInput { destination, input, .. } => {
                Self::encode_destination(output, destination)?;
                Self::encode_inputs(output, [ input ])?;
            },
            Self::DualInput { inputs, .. } => Self::encode_inputs(output, inputs)?,
            Self::DestinationAndDualInput { destination, inputs, .. } => {
                Self::encode_destination(output, destination)?;
                Self::encode_inputs(output, inputs)?;
            },
            _ => {}
        };
        
        Ok(())
    }
}