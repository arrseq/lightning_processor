use proc_bitfield::{bitfield};
use crate::instruction::Scale;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ReadOperation(pub u32): Debug, FromRaw, IntoRaw { 
        pub relative: bool @ 5,
        pub scale: u8 [unsafe! Scale] @ 6..=7,
        pub offset: u16 @ 8..=21,
        pub base: u8 @ 22..=26,
        pub destination: u8 @ 27..=31
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct WriteOperation(pub u32): Debug, FromRaw, IntoRaw { 
        pub relative: bool @ 5,
        pub scale: u8 [unsafe! Scale] @ 6..=7,
        pub offset: u16 @ 8..=21,
        pub base: u8 @ 22..=26,
        pub source: u8 @ 27..=31
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct StackOperation(pub u32): Debug, FromRaw, IntoRaw { 
        pub vec: bool @ 5,
        pub scale: u8 [unsafe! Scale] @ 6..=7,
        pub source: u8 @ 27..=31
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct UnStackOperation(pub u32): Debug, FromRaw, IntoRaw { 
        pub vec: bool @ 5,
        pub scale: u8 [unsafe! Scale] @ 6..=7,
        pub destination: u8 @ 27..=31
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct LockOperation(pub u32): Debug, FromRaw, IntoRaw { 
        pub relative: bool @ 5,
        pub scale: u8 [unsafe! Scale] @ 6..=7,
        pub offset: u32 @ 8..=26,
        pub base: u8 @ 27..=31
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(u8)]
pub enum BranchMode {
    #[default]
    Basic,
    CallStack,
    Demote
}

impl From<u8> for BranchMode {
    fn from(code: u8) -> Self {
        match code {
            0 => Self::Basic,
            1 => Self::CallStack,
            2 => Self::Demote,
            _ => Self::default()
        }
    }
}

impl From<BranchMode> for u8 {
    fn from(mode: BranchMode) -> Self {
        mode as Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(u8)]
pub enum BranchCondition {
    #[default]
    Zero,
    Negative,
    Parity,
    Regrouping,
    Overflow
}

impl From<u8> for BranchCondition {
    fn from(code: u8) -> Self {
        match code {
            0 => Self::Zero,
            1 => Self::Negative,
            2 => Self::Parity,
            3 => Self::Regrouping,
            4 => Self::Overflow,
            _ => Self::default(), // Default case for invalid values
        }
    }
}

impl From<BranchCondition> for u8 {
    fn from(condition: BranchCondition) -> Self {
        condition as Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BranchHint(pub Option<bool>);

impl From<u8> for BranchHint {
    fn from(value: u8) -> Self {
        Self(Some(match value {
            1 => false,
            2 => true,
            _ => return Self(None)
        }))
    }
}

impl From<BranchHint> for u8 {
    fn from(value: BranchHint) -> Self {
        match value.0 {
            None => 0,
            Some(taken) => if taken { 1 } else { 0 } 
        }
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct BranchOperation(pub u32): Debug, FromRaw, IntoRaw { 
        pub mode: u8 [unsafe! BranchMode] @ 5..=6,
        pub relative: bool @ 7,
        pub condition: u8 [unsafe! BranchCondition] @ 8..=10,
        pub hint: u8 [unsafe! BranchHint] @ 11..=12,
        pub offset: u16 @ 13..=26,
        pub base: u8 @ 27..=31
    }
}