#[cfg(test)]
mod test;

use std::io::Read;
use thiserror::Error;
use crate::instruction::operand;
use crate::instruction::operand::Operand;
use crate::instruction::operation::{Category, Operation};
use crate::math::dynamic_number;
use crate::math::dynamic_number::Unsigned;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to read operation specifier code")]
    Chain(#[source] dynamic_number::chain::DecodeError),
    #[error("The operation code was not recognized")]
    InvalidOperation,
    #[error("The operand could not be decoded")]
    Operand(#[source] operand::encoding::DecodeError)
}

impl Operation {
    /// The maximum number of bytes an operation can be in the chain length encoding.
    pub const MAX_OPERATION_LENGTH: u8 = 2;
    
    fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let code = u16::from(Unsigned::decode_chain_length(input, Some(Self::MAX_OPERATION_LENGTH as usize)).map_err(DecodeError::Chain)?);
        let category = Self::OPERATIONS.get(code as usize).ok_or(DecodeError::InvalidOperation)?.category;
        
        let destination = if category.has_destination() { Some(Operand::decode(input).map_err(DecodeError::Operand)?) }
        else { None };
        
        match category {
            Category::Destination => match code {
                
            }
            Category::Input => {}
            Category::InputAndDestination => {}
            Category::DualInput => {}
            Category::DualInputAndDestination => {}
        }
        
        todo!()
    }
}