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
    fn input_count(code: u16) -> u8 {
        match code {
            Self::STACK.code => {},    
            Self::UNSTACK => {}, 
            Self::COPY => {},     
            Self::COMPARE => {}, 
            Self::SIGNED_COMPARE => {},
            Self::ADD => {},     
            Self::SUBTRACT => {}, 
            Self::MULTIPLY => {},
            Self::DIVIDE => {}   
        }
    }
    
    fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let code = Unsigned::decode_unprefixed(input).map_err(DecodeError::DynamicNumber)?;
        dbg!(code);
        todo!()
    }
}