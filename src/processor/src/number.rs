//! Unsized absolute number. 
//! While Rust has u8, u16... for absolute values, it does not have a simple enum for variable length
//! absolute integers. 

// Constants

use crate::instruction::operand::{IMMEDIATE_EXPONENT_BYTE, IMMEDIATE_EXPONENT_DUAL, IMMEDIATE_EXPONENT_QUAD, IMMEDIATE_EXPONENT_WORD};

pub const BYTE_SIZE: u8 = 1;
pub const WORD_SIZE: u8 = 2;
pub const DUAL_SIZE: u8 = 4;
pub const QUAD_SIZE: u8 = 8;

// Implementations

/// Absolute modes.
/// Base type variants for representing an absolute value.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Type {
    #[default]
    Byte,
    Word,
    Dual,
    Quad
}

impl Type {
    /// Number of bytes the current variant holds. The uint size.
    pub fn size(&self) -> u8 {
        match self {
            Self::Byte => BYTE_SIZE,
            Self::Word => WORD_SIZE,
            Self::Dual => DUAL_SIZE,
            Self::Quad => QUAD_SIZE
        }
    }
}

impl From<Data> for Type {
    fn from(value: Data) -> Self {
        match value {
            Data::Byte(_) => Self::Byte,
            Data::Word(_) => Self::Word,
            Data::Dual(_) => Self::Dual,
            Data::Quad(_) => Self::Quad
        }
    }
}

impl Type {
    /// Create from a number of bytes.
    pub fn from_bytes(bytes: u8) -> Option<Self> {
        Some(match bytes {
            BYTE_SIZE => Type::Byte,
            WORD_SIZE => Type::Word,
            DUAL_SIZE => Type::Dual,
            QUAD_SIZE => Type::Quad,
            _ => return None
        })
    }

    /// Create from an exponent of 2. The maximum supported exponent is 3.
    pub fn from_exponent(exponent: u8) -> Option<Self> {
        Self::from_bytes(2u8.pow(exponent as u32))
    }

    pub fn exponent(&self) -> u8 {
        match self {
            Self::Byte => IMMEDIATE_EXPONENT_BYTE,
            Self::Word => IMMEDIATE_EXPONENT_WORD,
            Self::Dual => IMMEDIATE_EXPONENT_DUAL,
            Self::Quad => IMMEDIATE_EXPONENT_QUAD
        }
    }
}

/// Variable absolute data type.
/// Complete variants that annotate numbers with their type in the same enum allowing for the data type to be changed
/// during runtime.
#[derive(Debug, Clone, Eq)]
pub enum Data {
    Byte(u8),
    Word(u16),
    Dual(u32),
    Quad(u64)
}

impl Data {
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            Self::Byte(number) => bytes.extend(number.to_le_bytes()),
            Self::Word(number) => bytes.extend(number.to_le_bytes()),
            Self::Dual(number) => bytes.extend(number.to_le_bytes()),
            Self::Quad(number) => bytes.extend(number.to_le_bytes()),
        }

        bytes
    }

    pub fn exponent(self) -> u8 {
        Type::from(self).exponent()
    }

    /// Get the data as a quad sized uint.
    pub fn quad(&self) -> u64 {
        match *self {
            Self::Byte(value) => value as u64,
            Self::Word(value) => value as u64,
            Self::Dual(value) => value as u64,
            Self::Quad(value) => value as u64
        }
    }

    /// Try to fit a 64-bit number into the smallest division variant of this type.
    pub fn from_quad_selecting(quad: u64) -> Self {
        if quad <= u8::MAX as u64 { return Self::Byte(quad as u8) }
        if quad <= u16::MAX as u64 { return Self::Word(quad as u16) }
        if quad <= u32::MAX as u64 { return Self::Dual(quad as u32) }
        Self::Quad(quad)
    }
}

// region: Converting numbers to data instances
impl From<u8> for Data {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

impl From<u16> for Data {
    fn from(value: u16) -> Self {
        Self::Word(value)
    }
}

impl From<u32> for Data {
    fn from(value: u32) -> Self {
        Self::Dual(value)
    }
}

impl From<u64> for Data {
    fn from(value: u64) -> Self {
        Self::Quad(value)
    }
}
// endregion

impl From<Type> for Data {
    fn from(value: Type) -> Self {
        match value {
            Type::Byte => Self::Byte(0),
            Type::Word => Self::Word(0),
            Type::Dual => Self::Dual(0),
            Type::Quad => Self::Quad(0)
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.quad() == other.quad()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractError {
    /// The start byte index is out of bounds.
    StartOutOfBounds,
    /// The selection size is too large for the start.
    EndOutOfBounds
}