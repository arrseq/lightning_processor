pub mod encoding;

use thiserror::Error;
use crate::math::dynamic_number::{DynamicNumber, Size};

/// Modes for dereferencing with a register from the first mode byte.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterMode {
    Register,
    Dereference
}

/// Modes for dereferencing with a constant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantMode {
    Constant,
    Relative,
    ArrayInObject
}

/// Mode for the second mode byte. It specifies more modes for the operand to be evaluated as.
///
/// # Usage
/// Used for more complex encoding.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecondMode {
    Array,
    ConstantBased { mode: ConstantMode, constant: DynamicNumber }
}

impl SecondMode {
    pub const ARRAY_ADDRESSING_MODE: u8 = 0;
    pub const CONSTANT_MODE        : u8 = 1;
    pub const RELATIVE_MODE        : u8 = 2;
    pub const ARRAY_IN_OBJECT_MODE : u8 = 3;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrayAddressing {
    Array,
    Offsetted
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComplexAddressing {
    Base,
    BasePlusOffset  { offset: DynamicNumber },
    ArrayAddressing { mode: ArrayAddressing, index_register: u8, offset: DynamicNumber }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexAddressingFieldRequirements {
    pub specifier_code: u8,
    pub requires_offset: bool,
    pub requires_index_register: bool
}

impl ComplexAddressing {
    pub const BASE_CODE: u8 = 0;
    pub const BASE_PLUS_OFFSET_CODE: u8 = 1;
    pub const ARRAY_CODE: u8 = 2;
    pub const OFFSETTED_ARRAY_CODE: u8 = 3;
    
    pub const BASE            : ComplexAddressingFieldRequirements = ComplexAddressingFieldRequirements { specifier_code: Self::BASE_CODE,             requires_offset: false, requires_index_register: false };
    pub const BASE_PLUS_OFFSET: ComplexAddressingFieldRequirements = ComplexAddressingFieldRequirements { specifier_code: Self::BASE_PLUS_OFFSET_CODE, requires_offset: true,  requires_index_register: false };
    pub const ARRAY           : ComplexAddressingFieldRequirements = ComplexAddressingFieldRequirements { specifier_code: Self::ARRAY_CODE,            requires_offset: true,  requires_index_register: true };
    pub const OFFSETTED_ARRAY : ComplexAddressingFieldRequirements = ComplexAddressingFieldRequirements { specifier_code: Self::OFFSETTED_ARRAY_CODE,  requires_offset: true,  requires_index_register: true };
}

/// How the operand should evaluate its value.
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
    pub const REGISTER_MODE            : u8 = 0;
    pub const DEREFERENCE_REGISTER_MODE: u8 = 1;
    pub const CONSTANT_MODE            : u8 = 2;
    pub const SECOND_MODE              : u8 = 3;
}

/// The registers of the 2 encoded bytes.
///
/// # Registers
/// The first register is mandatory and the second is present and used depending on conditions of the second byte which
/// is also optional.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EncodedRegisters(pub u8, pub Option<u8>);

/// # Reason
/// Used when a field is attempted to be accessed, but it isn't supported for the specific addressing mode.
#[derive(Debug, Error)]
#[error("The field accessed is not available on this mode")]
pub struct UnsupportedModeField;

impl Mode {
    pub fn registers(self) -> Result<EncodedRegisters, UnsupportedModeField> {
        let register = match self {
            Mode::Register { register, .. } => Some((register, None)),
            Mode::Constant { .. } => None,
            Mode::Second { base_register, index_register, .. } => Some((base_register, Some(index_register)))
        };

        if let Some(register) = register { return Ok(EncodedRegisters(register.0, register.1)) }
        Err(UnsupportedModeField)
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
    pub data_size: Size
}
