use crate::instruction::register::Register;
use crate::num;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    CopyRegister,
    CopyAddressed,
    Acquire
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Absolute,
    Relative
}

pub const BASE_IMMEDIATE_OFFSET_MASK: u16 = 0x0C;
pub type BaseImmediateOffset = num::MaskedU16<BASE_IMMEDIATE_OFFSET_MASK>;
pub type BaseIndexImmediateOffset = u8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndexMode {
    Offset(BaseIndexImmediateOffset),
    RegisterOffset(Register)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaseMode {
    Offset(BaseImmediateOffset),
    RegisterOffset(Register),
    Index {
        mode: IndexMode,
        index: Register }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Address {
    Immediate {
        mode: Mode,
        immediate: u16 },
    Register {
        mode: Mode,
        register: Register },
    Base {
        mode: BaseMode,
        base: Register }
}