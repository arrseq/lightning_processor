#[cfg(test)]
mod test;

use std::io;
use std::io::{Read, Write};
use thiserror::Error;
use crate::instruction::{operand, operation};
use crate::instruction::operand::Operand;
use crate::instruction::operation::{OperandCategory, DestinationAndDualInput, DestinationAndInput, DualInput, Input, Operation, Destination, VectorComponent};
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
    const fn code(self) -> u16 {
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
    const fn code(self) -> u16 {
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
    const fn code(self) -> u16 {
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
    const fn code(self) -> u16 {
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
    pub const fn code(self) -> u16 {
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
    Operand { #[source] source: operand::encoding::Error, error: OperandError },
    #[error("Failed to read from input")]
    Io { #[source] source: io::Error, error: IoError }
}

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Failed to write encoded form of map vector modifier")]
    MapVector
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("Failed to write operation specifier code")]
    Chain { #[source] source: io::Error },
    #[error("Failed to encode operand")]
    Operand { #[source] source: operand::encoding::Error, error: OperandError },
    #[error("Failed to write to output")]
    Io { #[source] source: io::Error, error: IoError }
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
            OperandCategory::Destination => Self::Destination {
                operation: Destination::from_code(code).unwrap(),
                destination: Self::decode_operand(input, OperandError::Destination)?
            },
            OperandCategory::Input => Self::Input {
                operation: Input::from_code(code).unwrap(),
                input: Self::decode_operand(input, OperandError::Input { nth: 0 })?
            },
            OperandCategory::DestinationAndInput => Self::DestinationAndInput {
                operation: DestinationAndInput::from_code(code).unwrap(),
                destination: Self::decode_operand(input, OperandError::Destination)?,
                input: Self::decode_operand(input, OperandError::Input { nth: 0 })?
            },
            OperandCategory::DualInput => Self::DualInput {
                operation: DualInput::from_code(code).unwrap(),
                inputs: [
                    Self::decode_operand(input, OperandError::Input { nth: 0 })?,
                    Self::decode_operand(input, OperandError::Input { nth: 1 })?
                ]
            },
            OperandCategory::DestinationAndDualInput => Self::DestinationAndDualInput {
                operation: DestinationAndDualInput::from_code(code).unwrap(),
                destination: Self::decode_operand(input, OperandError::Destination)?,
                inputs: [
                    Self::decode_operand(input, OperandError::Input { nth: 0 })?,
                    Self::decode_operand(input, OperandError::Input { nth: 1 })?
                ]
            }
        }); }

        if code == Self::MAP_VECTOR_CODE {
            let map_vector = Self::decode_map_vector(input)?;
            return Ok(Self::MapVector {
                mappings: map_vector.1,
                operand: map_vector.0
            });
        };

        Self::from_no_operand_code(code).ok_or(DecodeError::InvalidOperation)
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

    pub(super) const fn code(self) -> u16 {
        match self {
            Self::None => Self::NONE.code,
            Self::Lock => Self::LOCK.code,
            Self::VectorOperands => Self::VECTOR_OPERANDS.code,
            Self::MapVector { .. } => Self::MAP_VECTOR.code,
            Self::OverrideBranch => Self::OVERRIDE_BRANCH.code,
            Self::Destination { operation, .. } => operation.code(),
            Operation::Input { operation, .. } => operation.code(),
            Operation::DestinationAndInput { operation, .. } => operation.code(),
            Operation::DualInput { operation, .. } => operation.code(),
            Operation::DestinationAndDualInput { operation, .. } => operation.code(),
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

    fn optional_vector_component_from_code(code: u8) -> Option<VectorComponent> {
        Some(match code {
            1 => VectorComponent::X0,
            2 => VectorComponent::X1,
            3 => VectorComponent::X2,
            4 => VectorComponent::X3,
            _ => return None
        })
    }

    fn optional_vector_component_code(vector_component: Option<VectorComponent>) -> u8 {
        if let Some(component) = vector_component { match component {
            VectorComponent::X0 => 1,
            VectorComponent::X1 => 2,
            VectorComponent::X2 => 3,
            VectorComponent::X3 => 4
        }} else { 0 }
    }
    
    fn encode_map_vector(output: &mut impl Write, operand: u8, mappings: [Option<VectorComponent>; 4]) -> Result<(), EncodeError> {
        // todo: fix comments
        let mut encoded = operand << 6;
        encoded |= (Self::optional_vector_component_code(mappings[0]) & 0b00000_111) << 3;
        encoded |= Self::optional_vector_component_code(mappings[1]) & 0b00000_111;

        let mut second_encoded = (Self::optional_vector_component_code(mappings[2]) & 0b00000_111) << 5;
        second_encoded |= (Self::optional_vector_component_code(mappings[3]) & 0b00000_111) << 2;

        output.write_all(&[ encoded, second_encoded ]).map_err(|source| EncodeError::Io { source, error: IoError::MapVector })
    }

    fn decode_map_vector(input: &mut impl Read) -> Result<(u8, [Option<VectorComponent>; 4]), DecodeError> {
        let mut buffer = [0u8; 2];
        input.read_exact(&mut buffer).map_err(|source| DecodeError::Io { source, error: IoError::MapVector })?;

        let operand = (buffer[0] & 0b11_000_000) >> 6;
        let mapping_0 = (buffer[0] & 0b00_111_000) >> 3;
        let mapping_1 = buffer[0] & 0b00_000_111;
        let mapping_2 = (buffer[1] & 0b111_000_00) >> 5;
        let mapping_3 = (buffer[1] & 0b000_111_00) >> 2;

        Ok((operand, [
            Self::optional_vector_component_from_code(mapping_0),
            Self::optional_vector_component_from_code(mapping_1),
            Self::optional_vector_component_from_code(mapping_2),
            Self::optional_vector_component_from_code(mapping_3)
        ]))
    }
    
    pub(crate) fn encode(self, output: &mut impl Write) -> Result<(), EncodeError> {
        let operation = self.code();
        Unsigned::new(operation as u64)
            .encode_chain(output, Some(Self::MAX_OPERATION_LENGTH as u64))
            .map_err(|source| EncodeError::Chain { source })?;
        
        match self {
            Self::MapVector { mappings, operand } => Self::encode_map_vector(output, operand, mappings)?,
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