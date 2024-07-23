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

/// Metadata for the operand involving the size of the operands, addressing mode, and more.
///
/// Some fields are privately initiated to ensure the validity of the data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meta {
    /// The size of the data being referenced by the operand(s).
    pub size: dynamic_number::Size,

    /// The name of the operand in which to store the result in.
    pub destination: Name,

    /// Whether to override the destination and use an external dynamic operand as the destination.
    pub external_destination: bool,

    /// The encoded code of the dynamic operand. The entire dynamic operand is not stored here to modularize the 
    /// decoding process.
    dynamic_code: u8,
}

/// The result of the meta points to the dynamic operand but the dynamic operand is constant data.
#[derive(Debug, Error)]
#[error("The result points to a constant")]
pub struct ResultToConstantError;

impl Meta {
    /// # Result
    /// Instance of [Self] as long as the dynamic operand isn't constant with the result of this metadata being set to
    /// the dynamic operand.
    pub fn new(size: dynamic_number::Size, destination: Name, custom_data: bool, dynamic: Dynamic) -> Result<Self, ResultToConstantError> {
        if matches!(dynamic, Dynamic::Constant(_)) && matches!(destination, Name::Dynamic) { return Err(ResultToConstantError) }
        Ok(Self {
            size, destination,
            external_destination: custom_data,
            dynamic_code: dynamic.encode()
        })
    }

    pub fn encode(self) -> u8 {
        let mut encoded = self.size.exponent_representation() << 6;
        encoded |= (matches!(self.destination, Name::Dynamic) as u8) << 5;
        encoded |= self.dynamic_code << 1;
        encoded |= self.external_destination as u8;
        encoded
    }

    /// # Result
    /// This function has no error because the dynamic code is never invalid. Valid dynamic codes are 4 bits.
    pub fn decode(encoded: u8) -> Result<Self, dynamic::InvalidCodeError> {
        let size = dynamic_number::Size::from_exponent_representation(encoded >> 6).unwrap();
        let destination = if (encoded >> 5) & 0b0000000_1 == 1 { Name::Dynamic } else { Name::Register };
        let dynamic_code = (encoded & 0b000_1111_0) >> 1;
        if !Dynamic::is_valid(dynamic_code) { return Err(dynamic::InvalidCodeError); }
        let custom_data = encoded & 0b0000000_1 == 1;
        Ok(Self { size, destination, dynamic_code, external_destination: custom_data })
    }

    pub fn dynamic_code(self) -> u8 {
        self.dynamic_code
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operands {
    /// The size of the data that the operands refer to.
    pub size: dynamic_number::Size,
    pub destination: Name,
    pub register: Register,
    pub dynamic: Dynamic,
    pub external_destination: Option<Dynamic>
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("")]
    InvalidDynamicCode(#[source] dynamic::InvalidCodeError),
    
    #[error("")]
    Read(#[source] io::Error)
}

#[derive(Debug)]
pub enum EncodeError {
    Write(io::Error),
    ResultToConstant(ResultToConstantError)
}

impl Operands {
    pub fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        // Contains the meta and registers bytes.
        let mut buffer = [0u8; 2];
        input.read_exact(&mut buffer).map_err(DecodeError::Read)?;

        // Code after this should unwrap on things that return [Err(dynamic::InvalidCodeError)] due to this.
        let meta = Meta::decode(buffer[0]).map_err(DecodeError::InvalidDynamicCode)?;
        let registers = register::Dual::decode(buffer[1]);
        
        let dynamic = match Dynamic::requirement(meta.dynamic_code).unwrap() {
            dynamic::Requirement::Register => Dynamic::decode_register(meta.dynamic_code, registers.second),
            dynamic::Requirement::Constant(size) => Dynamic::decode_constant(meta.dynamic_code, Self::decode_constant(input, size.unwrap_or(meta.size)).map_err(DecodeError::Read)?),
            // There is no dynamic operand mode with this requirement that uses [None] for its constant size here. It is
            // acceptable to unwrap here.
            dynamic::Requirement::RegisterAndConstant(size) => {
                let calculated = dynamic::Calculated {
                    base: registers.second,
                    offset: Self::decode_constant(input, size.unwrap_or(meta.size)).map_err(DecodeError::Read)?
                };
                
                Dynamic::decode_calculated(meta.dynamic_code, calculated)
            }
        }.unwrap();
        
        let external_destination = if !meta.external_destination { None } 
        else {
            todo!() as Option<Dynamic>
        };
        
        Ok(Self {
            size: meta.size,
            destination: meta.destination,
            register: registers.first,
            dynamic,
            external_destination
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
        let meta = Meta::new(self.size, self.destination, self.external_destination.is_some(), self.dynamic).map_err(EncodeError::ResultToConstant)?;
        let registers = register::Dual {
            first: self.register,
            second: self.dynamic.register().unwrap_or_default()
        };
        
        let buffer = [meta.encode(), registers.encode()];
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
