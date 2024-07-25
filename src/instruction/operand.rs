use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constant {
    pub value: u64,
    pub mask: u64
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressingMode {
    Register,
    Constant,
    Array,
    ArrayInObject
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Addressing {
    pub mode: AddressingMode,
    pub index_register: u8
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Register,
    Constant,
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
    
    pub fn encode(self) -> u8 {
        match self {
            Self::Register => Self::REGISTER_MODE,
            Self::Constant => Self::CONSTANT_MODE,
            Self::Addressing(addressing) => match addressing.mode {
                AddressingMode::Register => Self::REGISTER_ADDRESSING_MODE,
                AddressingMode::Constant => Self::CONSTANT_ADDRESSING_MODE,
                AddressingMode::Array => Self::ARRAY_ADDRESSING_MODE,
                AddressingMode::ArrayInObject => Self::ARRAY_IN_OBJECT_ADDRESSING_MODE
            }
        }
    }
    
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
    pub data_mask: u64,
    pub constant: Constant
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operands {
    pub destination: Option<Operand>,
    pub operands: [Option<Operand>; 3]
}