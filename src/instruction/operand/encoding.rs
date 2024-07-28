#[cfg(test)]
mod test;

use std::io;
use std::io::{Read, Write};
use thiserror::Error;
use crate::instruction::operand::{ConstantMode, EncodedModes, EncodedRegisters, Mode, Operand, RegisterMode, SecondMode};
use crate::math::dynamic_number::{DynamicNumber, Size};

/// # Usage
/// This can be used to decode the first and second mode byte because they both follower the same field types.
#[derive(Debug, Clone, Copy, PartialEq)]
struct ModeByte { register: u8, mode: u8, size: u8 }

impl ModeByte {
    fn encode(self) -> u8 {
        let mut encoded = self.register << 4;
        encoded |= (self.mode & 0b000000_11) << 2;
        encoded |= self.size & 0b000000_11;
        encoded
    }

    fn decode(encoded: u8) -> Self {
        let register = (encoded & 0xF0) >> 4;
        let mode = (encoded & 0b0000_11_00) >> 2;
        let size = encoded & 0b000000_11;
        Self { register, mode, size }
    }

    fn decode_size(self) -> Size {
        Size::from_power(self.size)
    }
}

#[derive(Debug, Error)]
pub enum DecodeIoError {
    #[error("Mandatory first mode byte was expected")]
    FirstModeByte,
    #[error("Second mode byte was expected")]
    SecondModeByte,
    #[error("{count} bytes expected for the constant")]
    Constant { count: u8 }
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Lock instruction used without memory operands")]
    InvalidLock,
    #[error("Could not decode due to IO Error")]
    Io { #[source] io: io::Error, decoder: DecodeIoError },
}

impl Operand {
    fn encode(self, output: &mut impl Write) -> io::Result<()> {
        let registers = self.mode.registers().unwrap_or(EncodedRegisters(0, Some(0)));
        let modes = match self.mode {
            Mode::Register { mode, .. } => match mode {
                RegisterMode::Register => EncodedModes(Mode::REGISTER_MODE, None),
                RegisterMode::Dereference => EncodedModes(Mode::DEREFERENCE_REGISTER_MODE, None)
            },
            Mode::Constant { .. } => EncodedModes(Mode::CONSTANT_MODE, None),
            Mode::Second { mode, .. } => match mode {
                SecondMode::Array => EncodedModes(Mode::SECOND_MODE, Some(SecondMode::ARRAY_ADDRESSING_MODE)),
                SecondMode::ConstantBased { mode, .. } => match mode {
                    ConstantMode::Constant => EncodedModes(Mode::SECOND_MODE, Some(SecondMode::CONSTANT_MODE)),
                    ConstantMode::Relative => EncodedModes(Mode::SECOND_MODE, Some(SecondMode::RELATIVE_MODE)),
                    ConstantMode::ArrayInObject => EncodedModes(Mode::SECOND_MODE, Some(SecondMode::ARRAY_IN_OBJECT_MODE))
                }
            }
        };

        // Write the first mode byte.
        let byte = ModeByte { register: registers.0, mode: modes.0, size: self.data_size.to_power() };
        output.write_all(&[byte.encode()])?;

        self.encode_second_mode_byte(output)?;

        Ok(())
    }

    fn encode_second_mode_byte(self, output: &mut impl Write) -> io::Result<()> {
        // Get the constant and write the second mode byte if applicable.
        let constant = if let Mode::Second { mode, index_register, .. } = self.mode {
            let constant = if let SecondMode::ConstantBased { constant, .. } = mode { Some(constant) }
            else { None };
            let constant_size = if let Some(constant) = constant { Size::from(constant).to_power() }
            else { 0 };
            
            let second_mode = match mode {
                SecondMode::Array => SecondMode::ARRAY_ADDRESSING_MODE,
                SecondMode::ConstantBased { mode, .. } => match mode {
                    ConstantMode::Constant => SecondMode::CONSTANT_MODE,
                    ConstantMode::Relative => SecondMode::RELATIVE_MODE,
                    ConstantMode::ArrayInObject => SecondMode::ARRAY_IN_OBJECT_MODE
                }
            };

            let byte = ModeByte { register: index_register, mode: second_mode, size: constant_size };
            output.write_all(&[byte.encode()])?;
            constant
        } else if let Mode::Constant { constant } = self.mode { Some(DynamicNumber::with_size_u64(self.data_size, constant)) }
        else { None };

        if let Some(constant) = constant { Self::encode_constant(output, constant)?; }

        Ok(())
    }

    fn encode_constant(output: &mut impl Write, constant: DynamicNumber) -> io::Result<()> {
        match constant {
            DynamicNumber::U8(value) => output.write_all(&[value])?,
            DynamicNumber::U16(value) => output.write_all(&value.to_le_bytes())?,
            DynamicNumber::U32(value) => output.write_all(&value.to_le_bytes())?,
            DynamicNumber::U64(value) => output.write_all(&value.to_le_bytes())?
        }

        Ok(())
    }

    fn decode(self, input: &mut impl Read) -> Result<Self, DecodeError> {
        let mut buffer = [0u8; 1];
        input.read_exact(&mut buffer).map_err(|io| DecodeError::Io { io, decoder: DecodeIoError::FirstModeByte })?;

        let first_mode_byte = ModeByte::decode(buffer[0]);
        let data_size = first_mode_byte.decode_size();

        // It is impossible for the mode decoded by the ModeByte to be invalid. That is why unreachable!() statements
        // are okay here.
        let mode = match first_mode_byte.mode {
            Mode::REGISTER_MODE => Mode::Register { mode: RegisterMode::Register, register: first_mode_byte.register },
            Mode::DEREFERENCE_REGISTER_MODE => Mode::Register { mode: RegisterMode::Dereference, register: first_mode_byte.register },
            Mode::CONSTANT_MODE => {
                let constant = Self::decode_constant(input, data_size)
                    .map_err(|io| DecodeError::Io { io, decoder: DecodeIoError::Constant { count: data_size.size() }})?;
                Mode::Constant { constant: u64::from(constant) }
            },
            Mode::SECOND_MODE => {
                input.read_exact(&mut buffer).map_err(|io| DecodeError::Io { io, decoder: DecodeIoError::SecondModeByte })?;
                let second_mode_byte = ModeByte::decode(buffer[0]);

                let second_mode = match second_mode_byte.mode {
                    SecondMode::ARRAY_ADDRESSING_MODE => SecondMode::Array,
                    SecondMode::CONSTANT_MODE
                    | SecondMode::RELATIVE_MODE
                    | SecondMode::ARRAY_IN_OBJECT_MODE => {
                        let constant_size = second_mode_byte.decode_size();
                        let constant = Self::decode_constant(input, constant_size)
                            .map_err(|io| DecodeError::Io { io, decoder: DecodeIoError::Constant { count: constant_size.size() }})?;

                        let mode = match second_mode_byte.mode {
                            SecondMode::CONSTANT_MODE => ConstantMode::Constant,
                            SecondMode::RELATIVE_MODE => ConstantMode::Relative,
                            SecondMode::ARRAY_ADDRESSING_MODE => ConstantMode::ArrayInObject,
                            _ => unreachable!()
                        };

                        SecondMode::ConstantBased { mode, constant }
                    },
                    _ => unreachable!()
                };

                Mode::Second { mode: second_mode, base_register: first_mode_byte.register, index_register: second_mode_byte.register }
            },
            _ => unreachable!()
        };

        Ok(Self { mode, data_size })
    }

    fn decode_constant(input: &mut impl Read, size: Size) -> io::Result<DynamicNumber> {
        Ok(match size {
            Size::U8 => {
                let mut buffer = [0u8; 1];
                input.read_exact(&mut buffer)?;
                DynamicNumber::U8(buffer[0])
            },
            Size::U16 => {
                let mut buffer = [0u8; (u16::BITS / 8) as usize];
                input.read_exact(&mut buffer)?;
                DynamicNumber::U16(u16::from_le_bytes(buffer))
            },
            Size::U32 => {
                let mut buffer = [0u8; (u32::BITS / 8) as usize];
                input.read_exact(&mut buffer)?;
                DynamicNumber::U32(u32::from_le_bytes(buffer))
            },
            Size::U64 => {
                let mut buffer = [0u8; (u64::BITS / 8) as usize];
                input.read_exact(&mut buffer)?;
                DynamicNumber::U64(u64::from_le_bytes(buffer))
            },
        })
    }
}
