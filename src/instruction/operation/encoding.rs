use std::io::Read;
use thiserror::Error;
use crate::instruction::operation::Operation;
use crate::math::dynamic_number;
use crate::math::dynamic_number::Unsigned;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to read operation specifier code")]
    DynamicNumber(#[source] dynamic_number::unprefixed::DecodeError)
}

impl Operation {
    /// Get the number of inputs a specific operation requires based on its code.
    /// 
    /// # Result
    /// [None] is returned if the code is invalid.
    fn input_count(code: u16) -> Option<u8> {
        Some(Self::OPERATIONS.get(code as usize)?.input_count)
    }

    /// Get whether the destination operand is present for an operation based on its code.
    ///
    /// # Result
    /// [None] is returned if the code is invalid.
    fn has_destination(code: u16) -> Option<bool> {
        Some(Self::OPERATIONS.get(code as usize)?.has_destination)
    }
    
    fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let code = Unsigned::decode_unprefixed(input).map_err(DecodeError::DynamicNumber)?;
        dbg!(code);
        todo!()
    }
}