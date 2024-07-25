use std::io;
use std::io::{Read, Write};
use thiserror::Error;
use crate::instruction_lg::operand::{Operand, register};
use crate::instruction_lg::operand::register::Register;
use crate::math::dynamic_number;

#[cfg(test)]
mod test;

/// Named of the 3 supported operands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Destination {
    /// Register only operands.
    Register,

    /// Dynamically addressed operand. This operand could potentially refer to one of many things.
    Dynamic,

    External(Operand)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operands {
    /// The size of the data that the operands refer to.
    pub size: dynamic_number::Size,
    pub destination: Destination,
    pub register: Register,
    pub dynamic: Operand,

    /// Whether to use as a vector.
    pub segmented: bool
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("")]
    InvalidDynamicCode(#[source] super::InvalidCodeError),

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
        let destination = if (meta_byte >> 5) & 0b0000000_1 == 1 { Destination::Dynamic } else { Destination::Register };
        let dynamic_code = (meta_byte & 0b000_1111_0) >> 1;
        let external_destination = meta_byte & 0b0000000_1 == 1;

        // Decode the actual operands.
        let registers = register::Dual::decode(buffer[1]);

        let dynamic = Self::decode_dynamic(input, dynamic_code, registers.second, meta_size)?;

        let external_destination = if !external_destination { None }
        else {
            // Decode an external destination. This involves it own dynamic code and register.
            let mut buffer = [0u8; 1];
            input.read_exact(&mut buffer).map_err(DecodeError::Read)?;
            let dynamic_code = (buffer[0] & 0b1111_0000) >> 4;

            // Unwrapping is safe here because all valid register codes are 4 bits.
            let register = Register::decode(buffer[0] & 0b0000_1111).unwrap();
            let dynamic = Self::decode_dynamic(input, dynamic_code, register, meta_size)?;

            Some(dynamic)
        };

        Ok(Self {
            size: meta_size,
            destination: if let Some(external_destination) = external_destination { Destination::External(external_destination) } else { destination },
            register: registers.first,
            dynamic, segmented
        })
    }

    fn decode_dynamic(input: &mut impl Read, dynamic_code: u8, register: Register, meta_size: dynamic_number::Size) -> Result<Operand, DecodeError> {
        Ok(match Operand::requirement(dynamic_code).unwrap() {
            super::Requirement::Register => Operand::decode_register(dynamic_code, register).map_err(DecodeError::InvalidDynamicCode)?,
            super::Requirement::Constant(size) => {
                let constant = Self::decode_constant(input, size.unwrap_or(meta_size)).map_err(DecodeError::Read)?;
                Operand::decode_constant(dynamic_code, constant).map_err(DecodeError::InvalidDynamicCode)?
            },
            // There is no dynamic operand mode with this requirement that uses [None] for its constant size here. It is
            // acceptable to unwrap here.
            super::Requirement::RegisterAndConstant(size) => {
                let calculated = super::Calculated {
                    base: register,
                    offset: Self::decode_constant(input, size.unwrap_or(meta_size)).map_err(DecodeError::Read)?
                };

                Operand::decode_calculated(dynamic_code, calculated).map_err(DecodeError::InvalidDynamicCode)?
            }
        })
    }

    fn decode_constant(input: &mut impl Read, size: dynamic_number::Size) -> Result<dynamic_number::Unsigned, io::Error> {
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
        encoded_meta |= (matches!(self.destination, Destination::Dynamic) as u8) << 5;
        encoded_meta |= self.dynamic.encode() << 1;
        encoded_meta |= matches!(self.destination, Destination::External(_)) as u8;

        // Encode the actual operands.
        let registers = register::Dual {
            first: self.register,
            second: self.dynamic.register().unwrap_or_default()
        };

        let buffer = [encoded_meta, registers.encode()];
        output.write_all(&buffer).map_err(EncodeError::Write)?;

        if let Ok(constant) = self.dynamic.constant() { Self::encode_constant(output, constant).map_err(EncodeError::Write)?; }

        if let Destination::External(external_destination) = self.destination {
            let mut encoded_meta = external_destination.encode() << 4;
            encoded_meta |= external_destination.register().unwrap_or(Register::default()).encode();
            output.write_all(&[encoded_meta]).map_err(EncodeError::Write)?;

            if let Ok(constant) = self.dynamic.constant() { Self::encode_constant(output, constant).map_err(EncodeError::Write)? }
        }
        Ok(())
    }

    fn encode_constant(output: &mut impl Write, constant: dynamic_number::Unsigned) -> Result<(), io::Error> {
        let bytes = constant.to_le_bytes();
        let buffer = bytes.as_slice();
        output.write_all(buffer)
    }
}
