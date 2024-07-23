use std::io;
use std::io::{Read, Write};
use thiserror::Error;
use crate::math::dynamic_number;
use super::operand;
use super::operand::dynamic::Dynamic;
use super::operand::register::Register;

pub mod dynamic;
pub mod modifier;
pub mod register;

#[cfg(test)]
mod test;

/// Named of the 2 supported operands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Name {
    /// Register only operands.
    Register,
    
    /// Dynamically addressed operand. This operand could potentially refer to one of many things.
    Dynamic
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operands {
    /// The size of the data that the operands refer to.
    pub size: dynamic_number::Size,
    pub destination: Name,
    pub register: Register,
    pub dynamic: Dynamic,
    pub external_destination: Option<Dynamic>,
    
    /// Whether to use as a vector.
    pub segmented: bool
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("")]
    InvalidDynamicCode(#[source] dynamic::InvalidCodeError),
    
    #[error("")]
    Read(#[source] io::Error)
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("")]
    Write(#[source] io::Error),

    #[error("The result points to a constant")]
    ResultToConstant
}

impl Operands {
    pub fn decode(input: &mut impl Read, segmented: bool) -> Result<Self, DecodeError> {
        // Contains the meta and registers bytes.
        let mut buffer = [0u8; 2];
        input.read_exact(&mut buffer).map_err(DecodeError::Read)?;

        // Decode meta data.
        let meta_byte = buffer[0];
        let meta_size = dynamic_number::Size::from_exponent_representation(meta_byte >> 6).unwrap();
        let destination = if (meta_byte >> 5) & 0b0000000_1 == 1 { Name::Dynamic } else { Name::Register };
        let dynamic_code = (meta_byte & 0b000_1111_0) >> 1;
        if !Dynamic::is_valid(dynamic_code) { return Err(DecodeError::InvalidDynamicCode(dynamic::InvalidCodeError)); }
        let external_destination = meta_byte & 0b0000000_1 == 1;
        
        // Decode the actual operands.
        let registers = register::Dual::decode(buffer[1]);
        
        let dynamic = match Dynamic::requirement(dynamic_code).unwrap() {
            dynamic::Requirement::Register => Dynamic::decode_register(dynamic_code, registers.second),
            dynamic::Requirement::Constant(size) => Dynamic::decode_constant(dynamic_code, Self::decode_constant(input, size.unwrap_or(meta_size)).map_err(DecodeError::Read)?),
            // There is no dynamic operand mode with this requirement that uses [None] for its constant size here. It is
            // acceptable to unwrap here.
            dynamic::Requirement::RegisterAndConstant(size) => {
                let calculated = dynamic::Calculated {
                    base: registers.second,
                    offset: Self::decode_constant(input, size.unwrap_or(meta_size)).map_err(DecodeError::Read)?
                };
                
                Dynamic::decode_calculated(dynamic_code, calculated)
            }
        }.unwrap();
        
        let external_destination = if !external_destination { None } 
        else {
            todo!() as Option<Dynamic>
        };
        
        Ok(Self {
            size: meta_size,
            destination,
            register: registers.first,
            dynamic, external_destination, segmented
        })
    }
    
    pub fn decode_constant(input: &mut impl Read, size: dynamic_number::Size) -> Result<dynamic_number::Unsigned, io::Error> {
        let mut quad_word_buffer = [0u8; dynamic_number::Size::QUAD_WORD_BYTES];
        let buffer = match size {
            dynamic_number::Size::Byte => &mut quad_word_buffer[0..1],
            dynamic_number::Size::Word => &mut quad_word_buffer[0..dynamic_number::Size::WORD_BYTES],
            dynamic_number::Size::DoubleWord => &mut quad_word_buffer[0..dynamic_number::Size::DOUBLE_WORD_BYTES],
            dynamic_number::Size::QuadWord => &mut quad_word_buffer[0..dynamic_number::Size::QUAD_WORD_BYTES]
        };
        
        input.read_exact(buffer)?;
        Ok(dynamic_number::Unsigned::from_le_bytes(buffer).unwrap())
    }
    
    pub fn encode(self, output: &mut impl Write) -> Result<(), EncodeError> {
        // Encode the operand meta data.
        let mut encoded_meta = self.size.exponent_representation() << 6;
        encoded_meta |= (matches!(self.destination, Name::Dynamic) as u8) << 5;
        encoded_meta |= self.dynamic.encode() << 1;
        encoded_meta |= self.external_destination.is_some() as u8;
        
        // Encode the actual operands.
        let registers = register::Dual {
            first: self.register,
            second: self.dynamic.register().unwrap_or_default()
        };
        
        let buffer = [encoded_meta, registers.encode()];
        output.write_all(&buffer).map_err(EncodeError::Write)?;
        
        if let Ok(constant) = self.dynamic.constant() { Self::encode_constant(output, constant).map_err(EncodeError::Write)?; }
        Ok(())
    }
    
    pub fn encode_constant(output: &mut impl Write, constant: dynamic_number::Unsigned) -> Result<(), io::Error> {
        let bytes = constant.to_le_bytes();
        let buffer = bytes.as_slice();
        output.write_all(buffer)
    }
}
