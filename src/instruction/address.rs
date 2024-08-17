use crate::instruction::RegisterCode;
use crate::num::{MaskedU16, MaskedU32, MaskedU8};

pub type LargeImmediate = MaskedU32<0x1FFFF>;
pub type ScaleCode = MaskedU8<0x3>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Absolute,
    Relative
}

pub type MediumImmediate = MaskedU16<0x1FFF>;
pub type ShortImmediate = MaskedU16<0x1FF>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndexedBaseOffsetMode {
    Immediate(ShortImmediate),
    Register(RegisterCode)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaseMode {
    Offset(MediumImmediate),
    RegisterOffset(RegisterCode),
    Indexed {
        index: RegisterCode,
        offset: IndexedBaseOffsetMode
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Address {
    Immediate {
        mode: Mode,
        immediate: LargeImmediate
    },
    Register {
        mode: Mode,
        register: RegisterCode
    },
    Base {
        mode: BaseMode,
        base: RegisterCode
    }
}