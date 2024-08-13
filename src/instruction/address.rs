use crate::instruction::RegisterCode;
use crate::num::{MaskedU16, MaskedU32, MaskedU8};

pub type Immediate = MaskedU32<0x1FFFF>;
pub type ScaleCode = MaskedU8<0x3>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Absolute,
    Relative
}

pub type BaseOffset = MaskedU16<0x1FFF>;
pub type IndexedBaseOffset = MaskedU16<0x1FF>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndexedBaseOffsetMode {
    Immediate(IndexedBaseOffset),
    Register(RegisterCode)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaseMode {
    Offset(BaseOffset),
    RegisterOffset(RegisterCode),
    Indexed {
        index: RegisterCode,
        offset: IndexedBaseOffsetMode
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Address {
    Immediate {
        immediate: Immediate,
        mode: Mode
    },
    Register {
        register: RegisterCode,
        mode: Mode
    },
    Base {
        base: RegisterCode,
        mode: BaseMode
    }
}