use strum_macros::FromRepr;
use utility::{FromCode, MaxCode, MaxWithBits};
use crate::utility::{ToCode, TryCoded, TryFromCode};

/// There are 15 different registers supported.
pub const INDEX_BITS: u8 = 4;
pub const MAX_INDEX: usize = (INDEX_BITS as usize).max_with_bits().unwrap();

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum General {
    A0,
    B0,
    C0,
    D0,
    E0,
    F0,
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    A2,
    B2,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Pointer {
    Base,
    Stack
}

#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u8)]
pub enum Code {
    GeneralA0,
    GeneralB0,
    GeneralC0,
    GeneralD0,
    GeneralE0,
    GeneralF0,
    GeneralA1,
    GeneralB1,
    GeneralC1,
    GeneralD1,
    GeneralE1,
    GeneralF1,
    GeneralA2,
    GeneralB2,
    PointerBase,
    PointerStack
}

impl From<&Register> for Code {
    fn from(value: &Register) -> Self {
        match value {
            Register::General(general) => match general {
                General::A0 => Self::GeneralA0,
                General::B0 => Self::GeneralB0,
                General::C0 => Self::GeneralC0,
                General::D0 => Self::GeneralD0,
                General::E0 => Self::GeneralE0,
                General::F0 => Self::GeneralF0,
                General::A1 => Self::GeneralA1,
                General::B1 => Self::GeneralB1,
                General::C1 => Self::GeneralC1,
                General::D1 => Self::GeneralD1,
                General::E1 => Self::GeneralE1,
                General::F1 => Self::GeneralF1,
                General::A2 => Self::GeneralA2,
                General::B2 => Self::GeneralB2
            },
            Register::Pointer(pointer) => match pointer {
                Pointer::Base => Self::PointerBase,
                Pointer::Stack => Self::PointerStack
            }
        }
    }
}

/// A valid register target. 
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    General(General),
    Pointer(Pointer)
}

// region: Validity
impl TryFromCode for Register {
    type Code = u8;

    fn try_from_code(code: Self::Code) -> Option<Self> {
        let code = Code::from_repr(code)?;
        Some(Self::from(&code))
    }
}

impl From<&Code> for Register {
    fn from(value: &Code) -> Self {
        match value {
            Code::GeneralA0 => Self::General(General::A0),
            Code::GeneralB0 => Self::General(General::B0),
            Code::GeneralC0 => Self::General(General::C0),
            Code::GeneralD0 => Self::General(General::D0),
            Code::GeneralE0 => Self::General(General::E0),
            Code::GeneralF0 => Self::General(General::F0),
            Code::GeneralA1 => Self::General(General::A1),
            Code::GeneralB1 => Self::General(General::B1),
            Code::GeneralC1 => Self::General(General::C1),
            Code::GeneralD1 => Self::General(General::D1),
            Code::GeneralE1 => Self::General(General::E1),
            Code::GeneralF1 => Self::General(General::F1),
            Code::GeneralA2 => Self::General(General::A2),
            Code::GeneralB2 => Self::General(General::B2),
            Code::PointerBase => Self::Pointer(Pointer::Base),
            Code::PointerStack => Self::Pointer(Pointer::Stack)
        }
    }
}

impl ToCode for Register {
    type Code = u8;

    fn to_code(&self) -> Self::Code { Code::from(self) as Self::Code }
}

impl MaxCode for Register {
    type Code = u8;

    fn max_code() -> Self::Code {
        MAX_INDEX as Self::Code
    }

    fn codes() -> Self::Code {
        MAX_INDEX as Self::Code + 1
    }
}

impl TryCoded for Register {
    type Code = u8;
}

impl FromCode for Register {
    type Code = u8;

    /// Try to perform this operation even with an invalid code. Only the least most significant bits will be used. The 
    /// number of bits read is determined by [INDEX_BITS].
    /// ```
    /// use atln_processor::instruction::register::Register;
    /// use atln_processor::utility::{FromCode, ToCode};
    ///
    /// assert_eq!(Register::from_code(0b11111111).to_code(), 0b0000_1111);
    fn from_code(mut code: Self::Code) -> Self {
        code &= MAX_INDEX as u8;
        let code_enum = Code::from_repr(code).unwrap();
        Self::from(&code_enum)
    }
}
// endregion