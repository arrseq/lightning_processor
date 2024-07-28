pub mod unprefixed;
pub mod arithmetic;
pub mod size;

use std::io;
use thiserror::Error;

/// # Power
/// The power is a representation of this primitive data type which when set to the power of 2 gives the size in bytes.
/// The power only has its 2 least significant bits used and the rest are discarded.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    U8,
    U16,
    U32,
    U64
}

impl Size {
    /// Construct this enum from an exponent of the power of 2.
    pub fn from_power(size: u8) -> Self {
        match size & 0b000000_11 {
            0 => Self::U8,
            1 => Self::U16,
            2 => Self::U32,
            3 => Self::U64,
            _ => unreachable!()
        }
    }

    /// Convert this enum representation to a power of 2.
    pub fn to_power(self) -> u8 {
        match self {
            Self::U8 => 0,
            Self::U16 => 1,
            Self::U32 => 2,
            Self::U64 => 3
        }
    }
}

impl From<DynamicNumber> for Size {
    fn from(value: DynamicNumber) -> Self {
        match value {
            DynamicNumber::U8(_) => Self::U8,
            DynamicNumber::U16(_) => Self::U16,
            DynamicNumber::U32(_) => Self::U32,
            DynamicNumber::U64(_) => Self::U64
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DynamicNumber {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64)
}

impl From<DynamicNumber> for u8 {
    fn from(value: DynamicNumber) -> Self {
        match value {
            DynamicNumber::U8(value) => value,
            DynamicNumber::U16(value) => value as u8,
            DynamicNumber::U32(value) => value as u8,
            DynamicNumber::U64(value) => value as u8,
        }
    }
}

impl From<DynamicNumber> for u16 {
    fn from(value: DynamicNumber) -> Self {
        match value {
            DynamicNumber::U8(value) => value as u16,
            DynamicNumber::U16(value) => value,
            DynamicNumber::U32(value) => value as u16,
            DynamicNumber::U64(value) => value as u16,
        }
    }
}

impl From<DynamicNumber> for u32 {
    fn from(value: DynamicNumber) -> Self {
        match value {
            DynamicNumber::U8(value) => value as u32,
            DynamicNumber::U16(value) => value as u32,
            DynamicNumber::U32(value) => value,
            DynamicNumber::U64(value) => value as u32,
        }
    }
}

impl From<DynamicNumber> for u64 {
    fn from(value: DynamicNumber) -> Self {
        match value {
            DynamicNumber::U8(value) => value as u64,
            DynamicNumber::U16(value) => value as u64,
            DynamicNumber::U32(value) => value as u64,
            DynamicNumber::U64(value) => value
        }
    }
}