use crate::num::{MaskedU32, MaskedU8};

pub type Code = MaskedU8<0x07>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flag {
    Zero,
    Negative,
    Overflow,
    Regrouping,
    Parity
}

pub type LargeOffset = MaskedU32<0x1FFFFFF>;
pub type SmallOffset = MaskedU32<0xFFFFF>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meta {
    pub relative: bool,
    pub call: bool,
    pub demote: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationWithBase {}