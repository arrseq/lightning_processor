use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterMode {
    Register,
    Dereference,
    Relative
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantBasedMode {
    Constant,
    Relative,
    ArrayInObject
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecondMode {
    Array,
    ConstantBased { mode: ConstantBasedMode, constant: u64, mask: u64 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Register { register: u8, mode: RegisterMode },
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
pub struct EncodedMode(u8, Option<u8>);

#[derive(Debug, Error)]
#[error("The operand mode code is invalid")]
pub struct InvalidCodeError;

impl Mode {
    pub const REGISTER_MODE               : u8 = 0;
    pub const DEREFERENCE_REGISTER_MODE   : u8 = 1;
    pub const RELATIVE_REGISTER_MODE      : u8 = 2;
    
    pub const CONSTANT_MODE               : u8 = 3;
    pub const ARRAY_ADDRESSING_MODE       : u8 = 4;
    
    pub const CONSTANT_SECOND_MODE       : u8 = 5;
    pub const RELATIVE_SECOND_MODE       : u8 = 6;
    pub const ARRAY_IN_OBJECT_SECOND_MODE: u8 = 7;

    /// Encode this operand based on its mode.
    pub fn encode_mode(self) -> EncodedMode {
        match self {
            Mode::Register { mode, .. } => match mode {
                RegisterMode::Register => Self::REGISTER_MODE,
                RegisterMode::Dereference => Self::DEREFERENCE_REGISTER_MODE,
                RegisterMode::Relative => Self::RELATIVE_REGISTER_MODE
            },
            Mode::Constant { .. } => Self::CONSTANT_MODE, 
            Mode::Second { mode, .. } => match mode {
                SecondMode::Array => Self::ARRAY_ADDRESSING_MODE,
                SecondMode::ConstantBased { mode, .. } => match mode {
                    ConstantBasedMode::Constant => Self::CONSTANT_SECOND_MODE,
                    ConstantBasedMode::Relative => Self::RELATIVE_SECOND_MODE,
                    ConstantBasedMode::ArrayInObject => Self::ARRAY_IN_OBJECT_SECOND_MODE
                }
            }
        }
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