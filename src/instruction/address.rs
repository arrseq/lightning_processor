use crate::instruction::{ScaleCode};
use crate::num::MaskedU32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccessMode {
    Read,
    Write
}

pub type LargeOffset = MaskedU32<0xFFFFF>;
pub type SmallOffset = MaskedU32<0x3FFF>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meta {
    pub relative: bool,
    pub access_mode: AccessMode,
    pub scale: ScaleCode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationWithBase {}