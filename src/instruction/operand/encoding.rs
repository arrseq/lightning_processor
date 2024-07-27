#[cfg(test)]
mod test;

use std::io;
use std::io::{Read, Write};
use crate::instruction::operand::{ConstantMode, EncodedModes, EncodedRegisters, InvalidModeError, Mode, Operand, RegisterMode, SecondMode};
use crate::math::dynamic_number::{DynamicNumber, Size};

impl SecondMode {
    pub fn encode_mode(self) -> u8 {
        match self {
            Self::Array => Self::ARRAY_ADDRESSING_SECOND_MODE,
            Self::ConstantBased { mode, .. } => match mode {
                ConstantMode::Constant => Self::CONSTANT_SECOND_MODE,
                ConstantMode::Relative => Self::RELATIVE_SECOND_MODE,
                ConstantMode::ArrayInObject => Self::ARRAY_IN_OBJECT_SECOND_MODE
            }
        }
    }
}

impl Mode {
    /// Encode this operand based on its mode.
    pub fn encode_mode(self) -> EncodedModes {
        match self {
            Mode::Register { mode, .. } => match mode {
                RegisterMode::Register => EncodedModes(Self::REGISTER_MODE, None),
                RegisterMode::Dereference => EncodedModes(Self::DEREFERENCE_REGISTER_MODE, None)
            },
            Mode::Constant { .. } => EncodedModes(Self::CONSTANT_MODE, None),
            Mode::Second { mode, .. } => match mode {
                SecondMode::Array => EncodedModes(Self::SECOND_MODE, Some(SecondMode::ARRAY_ADDRESSING_SECOND_MODE)),
                SecondMode::ConstantBased { mode, .. } => match mode {
                    ConstantMode::Constant => EncodedModes(Self::SECOND_MODE, Some(SecondMode::CONSTANT_SECOND_MODE)),
                    ConstantMode::Relative => EncodedModes(Self::SECOND_MODE, Some(SecondMode::RELATIVE_SECOND_MODE)),
                    ConstantMode::ArrayInObject => EncodedModes(Self::SECOND_MODE, Some(SecondMode::ARRAY_IN_OBJECT_SECOND_MODE))
                }
            }
        }
    }

    pub fn decode_register_mode(first_mode: u8, register: u8) -> Result<Self, InvalidModeError> {
        let mode_variant = match first_mode {
            Self::REGISTER_MODE => RegisterMode::Register,
            Self::DEREFERENCE_REGISTER_MODE => RegisterMode::Dereference,
            _ => return Err(InvalidModeError)
        };

        Ok(Self::Register { mode: mode_variant, register })
    }

    pub fn decode_constant_mode(second_mode: u8, constant: DynamicNumber, base_register: u8, index_register: u8) -> Result<Self, InvalidModeError> {
        let mode_variant = match second_mode {
            SecondMode::CONSTANT_SECOND_MODE => ConstantMode::Constant,
            SecondMode::RELATIVE_SECOND_MODE => ConstantMode::Relative,
            SecondMode::ARRAY_IN_OBJECT_SECOND_MODE => ConstantMode::ArrayInObject,
            _ => return Err(InvalidModeError)
        };

        Ok(Self::Second {
            base_register, index_register,
            mode: SecondMode::ConstantBased { mode: mode_variant, constant }
        })
    }
}

/// # Usage
/// This can be used to encode the first and second mode byte because they both follower the same field types.
/// 
/// # Parameters
/// - The 4 least significant bits are used from the register.
/// - The first 2 least significant bits are used from the first_mode and operand size.
fn encode_mode_byte(register: u8, mut first_mode: u8, operand_size: u8) -> u8 {
    let mut encoded = register << 4;

    first_mode &= 0b000000_11;
    first_mode <<= 2;
    encoded |= first_mode;

    encoded |= operand_size & 0b000000_11;
    encoded
}

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
}

impl Operand {
    pub fn encode(self, output: &mut impl Write) -> io::Result<()> {
        let registers = self.mode.registers().unwrap_or(EncodedRegisters(0, Some(0)));
        let modes = self.mode.encode_mode();

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

            let byte = ModeByte { register: index_register, mode: mode.encode_mode(), size: constant_size };
            output.write_all(&[byte.encode()])?;
            constant
        } else if let Mode::Constant { constant } = self.mode { Some(DynamicNumber::with_size(self.data_size, constant)) }
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
    
    pub fn decode(self, input: &mut impl Read) -> io::Result<()> {
        
        Ok(())
    }
}
