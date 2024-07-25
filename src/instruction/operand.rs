use thiserror::Error;
use crate::instruction::operand::addressing::Addressing;

pub mod addressing;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Register,
    Constant { constant: u64 },
    Addressing(Addressing)
}

#[derive(Debug, Error)]
#[error("The operand mode code is invalid")]
pub struct InvalidCodeError;

impl Mode {
    pub const REGISTER_MODE: u8 = 0;
    pub const CONSTANT_MODE: u8 = 1;
    pub const REGISTER_ADDRESSING_MODE: u8 = 2;
    pub const CONSTANT_ADDRESSING_MODE: u8 = 3;
    pub const ARRAY_ADDRESSING_MODE: u8 = 4;
    pub const ARRAY_IN_OBJECT_ADDRESSING_MODE: u8 = 5;

    // TODO: Complete
    // pub fn decode(code: u8) -> Self {
    //     match code {
    //         Self::REGISTER_MODE => Self::Register,
    //         Self::CONSTANT_MODE => Self::Constant,
    //     }
    // }
    // TODO: Complete
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operand {
    pub mode: Mode,
    pub base_register: u8,
    pub data_mask: u64
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operands {
    pub destination: Option<Operand>,
    pub operands: [Option<Operand>; 3]
}