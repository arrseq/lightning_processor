use proc_bitfield::{bitfield, ConvRaw};
use crate::instruction::Scale;

#[derive(Debug, Clone, Copy, PartialEq, Default, ConvRaw)]
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