use proc_bitfield::{bitfield};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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

impl From<u8> for Mode {
    fn from(code: u8) -> Self {
        match code {
            0 => Self::And,
            1 => Self::Nand,
            2 => Self::Or,
            3 => Self::Nor,
            4 => Self::Xor,
            5 => Self::XNor,
            6 => Self::Add,
            7 => Self::Subtract,
            8 => Self::Multiply,
            9 => Self::Divide,
            10 => Self::ShiftLeft,
            11 => Self::ShiftRight,
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(u8)]
pub enum RegroupingBinaryMode {
    #[default]
    Add,
    Subtract
}

impl From<bool> for RegroupingBinaryMode {
    fn from(flag: bool) -> Self {
        match flag {
            false => Self::Add,
            true => Self::Subtract,
        }
    }
}

impl From<RegroupingBinaryMode> for bool {
    fn from(mode: RegroupingBinaryMode) -> Self {
        mode as u8 != 0
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(u8)]
pub enum RegroupingQuaternaryMode {
    #[default]
    Multiply,
    Divide
}

impl From<bool> for RegroupingQuaternaryMode {
    fn from(flag: bool) -> Self {
        match flag {
            false => Self::Multiply,
            true => Self::Divide,
        }
    }
}

impl From<RegroupingQuaternaryMode> for bool {
    fn from(mode: RegroupingQuaternaryMode) -> Self {
        mode as u8 != 0
    }
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