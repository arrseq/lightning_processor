#[cfg(test)]
mod test;

use std::io::{Read, Write};
use thiserror::Error;
use crate::instruction::operand;
use crate::instruction::operand::Operand;
use crate::instruction::operation::{Category, Operation};
use crate::math::dynamic_number;
use crate::math::dynamic_number::Unsigned;

#[derive(Debug, Error)]
pub enum DecodeOperandError {
    #[error("Failed to retrieve destination")]
    Destination,
    #[error("Failed to retrieve input")]
    Input
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to read operation specifier code")]
    Chain { #[source] source: dynamic_number::chain::Error },
    #[error("The operation code was not recognized")]
    InvalidOperation,
    #[error("Failed to retrieve operand")]
    Operand { #[source] source: operand::encoding::DecodeError, error: DecodeOperandError }
}

#[derive(Debug, Error)]
pub enum EncodeError {}

impl Operation {
    /// The maximum number of bytes an operation can be in the chain length encoding.
    pub const MAX_OPERATION_LENGTH: u8 = 2;
    
    fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let code = Unsigned::decode_chain(input, Some(Self::MAX_OPERATION_LENGTH as u64)).map_err(|source| DecodeError::Chain { source })?.value as u16;
        let category = Self::OPERATIONS.get(code as usize).ok_or(DecodeError::InvalidOperation)?.category;
        
        // The ends of the statements are marked unreachable in the match because the codes will always be valid for 
        // their operand types.
        Ok(match category {
            Category::Destination => {
                let destination = Self::decode_operand(input, DecodeOperandError::Destination)?;
                
                let operation = match code {
                    super::Destination::UNSTACK => super::Destination::Unstack,
                    _ => unreachable!()
                };

                Operation::Destination { operation, destination }
            },
            Category::Input => {
                let input = Self::decode_operand(input, DecodeOperandError::Input)?;

                let operation = match code {
                    super::Input::STACK => super::Input::Stack,
                    _ => unreachable!()
                };

                Operation::Input { operation, input }
            },
            Category::DestinationAndInput => {
                let destination = Self::decode_operand(input, DecodeOperandError::Destination)?;
                let input = Self::decode_operand(input, DecodeOperandError::Input)?;
                
                let operation = match code {
                    super::DestinationAndInput::COPY => super::DestinationAndInput::Copy,
                    _ => unreachable!()
                };
                
                Operation::DestinationAndInput { operation, destination, input }
            }
            Category::DualInput => {
                let input_0 = Self::decode_operand(input, DecodeOperandError::Input)?;
                let input_1 = Self::decode_operand(input, DecodeOperandError::Input)?;
                
                let operation = match code {
                    super::DualInput::COMPARE => super::DualInput::Compare,
                    super::DualInput::SIGNED_COMPARE => super::DualInput::SignedCompare,
                    _ => unreachable!()
                };
                
                Operation::DualInput { operation, input: [input_0, input_1] }
            },
            Category::DestinationAndDualInput => {
                let destination = Self::decode_operand(input, DecodeOperandError::Destination)?;
                let input_0 = Self::decode_operand(input, DecodeOperandError::Input)?;
                let input_1 = Self::decode_operand(input, DecodeOperandError::Input)?;

                let operation = match code {
                    super::DestinationAndDualInput::ADD               => super::DestinationAndDualInput::Add,
                    super::DestinationAndDualInput::FLOATING_ADD      => super::DestinationAndDualInput::FloatingAdd,
                    super::DestinationAndDualInput::SUBTRACT          => super::DestinationAndDualInput::Subtract,
                    super::DestinationAndDualInput::FLOATING_SUBTRACT => super::DestinationAndDualInput::FloatingSubtract,
                    super::DestinationAndDualInput::MULTIPLY          => super::DestinationAndDualInput::Multiply,
                    super::DestinationAndDualInput::FLOATING_MULTIPLY => super::DestinationAndDualInput::FloatingMultiply,
                    super::DestinationAndDualInput::DIVIDE            => super::DestinationAndDualInput::Divide,
                    super::DestinationAndDualInput::FLOATING_DIVIDE   => super::DestinationAndDualInput::FloatingDivide,
                    _ => unreachable!()
                };

                Operation::DestinationAndDualInput { operation, destination, input: [input_0, input_1] }
            }
        })
    }
    
    fn decode_operand(input: &mut impl Read, error: DecodeOperandError) -> Result<Operand, DecodeError> {
        Operand::decode(input).map_err(|source| DecodeError::Operand { source, error })
    }
    
    // fn encode(output: &mut impl Write) -> Result<(), EncodeError> {
    //     
    // }
}