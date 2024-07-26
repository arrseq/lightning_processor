#[cfg(test)]
mod test;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterMode {
    Register,
    Dereference
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantMode {
    Constant,
    Relative,
    ArrayInObject
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecondMode {
    Array,
    ConstantBased { mode: ConstantMode, constant: u64, mask: u64 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Register { mode: RegisterMode, register: u8 },
    Constant { constant: u64 },
    Second { mode: SecondMode, base_register: u8, index_register: u8 }
}

/// The encoded form of the addressing modes for an operand.
/// 
/// # Modes
/// The first mode is mandatory and determines the basic encoding mode. The second encoding mode is required for a 
/// specific mode in the first field which encodes more addressing modes which require more fields in the full encoding
/// of an operand.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EncodedModes(pub u8, pub Option<u8>);

/// # Reason
/// An error used when a mode is invalid or invalid for a specific situation.
#[derive(Debug, Error)]
#[error("The mode identifier code is invalid")]
pub struct InvalidModeError;

impl Mode {
    pub const REGISTER_MODE               : u8 = 0;
    pub const DEREFERENCE_REGISTER_MODE   : u8 = 1;
    pub const CONSTANT_MODE               : u8 = 2;
    pub const SECOND_MODE                 : u8 = 3;

    pub const ARRAY_ADDRESSING_SECOND_MODE: u8 = 0;
    pub const CONSTANT_SECOND_MODE        : u8 = 1;
    pub const RELATIVE_SECOND_MODE        : u8 = 2;
    pub const ARRAY_IN_OBJECT_SECOND_MODE : u8 = 3;

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

/// An operand that can reference one of many source that should be used to supply an operation.
///
/// # Encoding
/// The encoding of an operand is independent and does not depend on any external details of an instruction.
/// 
/// The encoding starts with the first byte which in cases may encode the entire operand. This byte includes the fields
/// named
/// - `first_register`: A register which is used alone, or as a base register for some addressing modes which may be 
///   unused depending on the mode.
/// - `first_addressing_mode`.
/// - `operand_size`.
/// 
/// If the `first_addressing_mode` codes for the second addressing mode to be present, the another byte will be read 
/// with the fields named
/// - `index_register`: Represents the index of an array. This field may be unused depending on the mode.
/// - `second_addressing_mode`.
/// - `constant_size`: Represents the size of the constant in bytes as a power of 2. This may be unused depending on 
///   the mode.
/// 
/// If the second addressing mode codes for a constant to be read, then the final bytes are read to code for the
/// constant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operand {
    pub mode: Mode,
    pub data_mask: u64
}