use proc_bitfield::{bitfield, ConvRaw};

#[derive(Debug, Clone, Copy, PartialEq, Default, ConvRaw)]
#[repr(u8)]
pub enum Mode {
    #[default]
    And,
    Nand,
    Or,
    Nor,
    Xor,
    XNor,
    Add,
    Subtract,
    Multiply,
    Divide,
    ShiftLeft,
    ShiftRight
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Operation(pub u32): Debug, FromRaw, IntoRaw {
        pub atomic: bool @ 5,
        pub vector: bool @ 6,
        pub mode: u8 [unsafe! Mode] @ 7..=10,
        pub source_0_offset: u8 @ 11..=12,
        pub source_1_offset: u8 @ 13..=14,
        pub destination_offset: u8 @ 15..=16,
        pub source_0: u8 @ 16..=20,
        pub source_1: u8 @ 21..=26,
        pub destination: u8 @ 27..=31
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ConvRaw)]
#[repr(u8)]
pub enum RegroupingBinaryMode {
    Add,
    Subtract
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct RegroupingBinaryOperation(pub u32): Debug, FromRaw, IntoRaw {
        pub atomic: bool @ 5,
        pub mode: bool [unsafe! RegroupingBinaryMode] @ 6,
        pub source_0: u8 @ 16..=20,
        pub source_1: u8 @ 21..=26,
        pub destination: u8 @ 27..=31
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ConvRaw)]
#[repr(u8)]
pub enum RegroupingQuaternaryMode {
    Multiply,
    Divide
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct RegroupingQuaternaryOperation(pub u32): Debug, FromRaw, IntoRaw {
        pub atomic: bool @ 5,
        pub mode: bool [unsafe! RegroupingQuaternaryMode] @ 6,
        pub upper: u8 @ 7..=11,
        pub source_0: u8 @ 12..=16,
        pub source_1: u8 @ 17..=21,
        pub remainder: u8 @ 22..=26,
        pub destination: u8 @ 27..=31
    }
}