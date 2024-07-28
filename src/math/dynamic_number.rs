pub mod unprefixed;
pub mod arithmetic;
pub mod size;

use std::io;
use thiserror::Error;
use crate::math::dynamic_number::size::Size;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DynamicNumber {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64)
}

#[derive(Debug, Error)]
pub enum DecodeUnprefixedError {
    #[error("Overflow occurred when adding to summation buffer")]
    Overflow,
    #[error("Failed to read next byte")]
    Io(#[source] io::Error)
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