#[cfg(test)]
mod test;

use std::io;
use std::io::Write;
use thiserror::Error;
use crate::instruction::operand;
use crate::instruction::operand::{ConstantMode, EncodedModes, EncodedRegisters, InvalidModeError, Mode, Operand, RegisterMode, SecondMode};

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
                SecondMode::Array => EncodedModes(Self::SECOND_MODE, Some(Self::ARRAY_ADDRESSING_SECOND_MODE)),
                SecondMode::ConstantBased { mode, .. } => match mode {
                    ConstantMode::Constant => EncodedModes(Self::SECOND_MODE, Some(Self::CONSTANT_SECOND_MODE)),
                    ConstantMode::Relative => EncodedModes(Self::SECOND_MODE, Some(Self::RELATIVE_SECOND_MODE)),
                    ConstantMode::ArrayInObject => EncodedModes(Self::SECOND_MODE, Some(Self::ARRAY_IN_OBJECT_SECOND_MODE))
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

    pub fn decode_constant_mode(second_mode: u8, constant: u64, constant_mask: u64, base_register: u8, index_register: u8) -> Result<Self, InvalidModeError> {
        let mode_variant = match second_mode {
            Self::CONSTANT_SECOND_MODE => ConstantMode::Constant,
            Self::RELATIVE_SECOND_MODE => ConstantMode::Relative,
            Self::ARRAY_IN_OBJECT_SECOND_MODE => ConstantMode::ArrayInObject,
            _ => return Err(InvalidModeError)
        };

        Ok(Self::Second {
            base_register, index_register,
            mode: SecondMode::ConstantBased { mode: mode_variant, constant, mask: constant_mask }
        })
    }
}

/// # Result
/// - If the mask is [u64::MAX] then 3 is returned.
/// - If the mask is [u32::MAX] then 2 is returned.
/// - If the mask is [u16::MAX] then 1 is returned.
/// - If the mask is [u8::MAX] or anything else, then 0 is returned.  
pub fn mask_to_operand_size(mask: u64) -> u8 {
    const U16_MAX: u64 = u16::MAX as u64;
    const U32_MAX: u64 = u32::MAX as u64;

    match mask {
        U16_MAX => 1,
        U32_MAX => 2,
        u64::MAX => 3,
        _ => 0
    }
}

/// # Parameters
/// - The 4 least significant bits are used from the register.
/// - The first 2 least significant bits are used from the first_mode and operand size.
pub fn encode_first_mode_byte(register: u8, mut first_mode: u8, mut operand_size: u8) -> u8 {
    let mut encoded = register << 4;

    first_mode &= 0b000000_11;
    first_mode <<= 2;
    encoded |= first_mode;

    operand_size &= 0b000000_11;
    encoded |= operand_size;

    encoded
}

impl Operand {
    pub fn encode(self, output: &mut impl Write) -> io::Result<()> {
        let registers = self.mode.registers().unwrap_or(EncodedRegisters(0, Some(0)));
        let modes = self.mode.encode_mode();
        
        let buffer = [encode_first_mode_byte(registers.0, modes.0, mask_to_operand_size(self.data_mask))];
        output.write_all(&buffer)?;
        
        Ok(())
    }
}
