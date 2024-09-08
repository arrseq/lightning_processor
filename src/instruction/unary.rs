use proc_bitfield::{bitfield};
use crate::instruction::Scale;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(u8)]
pub enum Mode {
    #[default]
    Invert,
    Increment,
    Decrement,
    ShiftLeft,
    ShiftRight,
    TrailingZeros,
    LeadingZeros,
    CountZeros
}

impl From<u8> for Mode {
    fn from(code: u8) -> Self {
        match code {
            0 => Self::Invert,
            1 => Self::Increment,
            2 => Self::Decrement,
            3 => Self::ShiftLeft,
            4 => Self::ShiftRight,
            5 => Self::TrailingZeros,
            6 => Self::LeadingZeros,
            7 => Self::CountZeros,
            _ => Self::default()
        }
    }
}

impl From<Mode> for u8 {
    fn from(mode: Mode) -> Self {
        mode as Self
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Operation(pub u32): Debug, FromRaw, IntoRaw {
        pub vector: bool @ 5,
        pub atomic: bool @ 6,
        pub mode: u8 [unsafe! Mode] @ 7..=9,
        pub source_offset: u8 @ 10..=15,
        pub destination_offset: u8 @ 16..=21,
        pub source: u8 @ 21..=26,
        pub destination: u8 @ 27..=31
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct NegateOperation(pub u32): Debug, FromRaw, IntoRaw {
        pub vector: bool @ 5,
        pub atomic: bool @ 6,
        pub scale: u8 [unsafe! Scale] @ 7..=8,
        pub source: u8 @ 21..=26,
        pub destination: u8 @ 27..=31
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ExtendSignOperation(pub u32): Debug, FromRaw, IntoRaw {
        pub vector: bool @ 5,
        pub atomic: bool @ 6,
        pub source_scale: u8 [unsafe! Scale] @ 7..=8,
        pub destination_scale: u8 [unsafe! Scale] @ 9..=10,
        pub source: u8 @ 21..=26,
        pub destination: u8 @ 27..=31
    }
}