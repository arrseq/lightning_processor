use std::io::Read;
use arrseq_memory::dynamic_number;
use crate::operand::register::Register;

/// A tuple containing a register and a constant which will be operated on and then used to address memory.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Calculated {
    pub register: Register,
    pub base: dynamic_number::Unsigned
}

/// A dynamic address dereferencing source target.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Address {
    /// Address memory with the dynamic operand's register field.
    Register(Register),

    /// Address memory with the instruction constant field.
    Constant(dynamic_number::Unsigned),

    /// Address memory with the sum of the dynamic operand's register field.
    Add(Calculated),

    /// Address memory with the difference of the dynamic operand's register field.
    Subtract(Calculated)
}

impl Address {
    /// Whether this contains a constant to operate in its current state.
    pub fn contains_constant(self) -> bool {
        match self {
            Self::Constant(_)
                | Self::Add(_)
                | Self::Subtract(_) => true,
            Self::Register(_) => false
        }
    }
}

/// A dynamic source operand.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dynamic {
    Register(Register),
    Constant(dynamic_number::Unsigned),
    Address(Address)
}

impl Dynamic {
    pub const REGISTER: u8 = 0;
    pub const CONSTANT: u8 = 1;
    
    pub const REGISTER_ADDRESS: u8 = 2;

    pub const CONSTANT_BYTE_ADDRESS: u8 = 3;
    pub const CONSTANT_WORD_ADDRESS: u8 = 4;
    pub const CONSTANT_DOUBLE_WORD_ADDRESS: u8 = 5;
    pub const CONSTANT_QUAD_WORD_ADDRESS: u8 = 6;
    
    pub const ADD_BYTE_ADDRESS: u8 = 7;
    pub const ADD_WORD_ADDRESS: u8 = 8;
    pub const ADD_DOUBLE_WORD_ADDRESS: u8 = 9;
    pub const ADD_QUAD_WORD_ADDRESS: u8 = 10;
    
    pub const SUBTRACT_BYTE_ADDRESS: u8 = 11;
    pub const SUBTRACT_WORD_ADDRESS: u8 = 12;
    pub const SUBTRACT_DOUBLE_WORD_ADDRESS: u8 = 13;
    pub const SUBTRACT_QUAD_WORD_ADDRESS: u8 = 14;
    
    /// Encode this dynamic operand into a 4 bit code.
    pub fn encode(self) -> u8 {
        match self {
            Self::Register(_) => Self::REGISTER,
            Self::Constant(_) => Self::CONSTANT,
            Self::Address(address) => match address {
                Address::Register(_) => Self::REGISTER_ADDRESS,
                Address::Constant(size) => match size {
                    dynamic_number::Unsigned::Byte(_) => Self::CONSTANT_BYTE_ADDRESS,
                    dynamic_number::Unsigned::Word(_) => Self::CONSTANT_WORD_ADDRESS,
                    dynamic_number::Unsigned::DoubleWord(_) => Self::CONSTANT_DOUBLE_WORD_ADDRESS,
                    dynamic_number::Unsigned::QuadWord(_) => Self::CONSTANT_QUAD_WORD_ADDRESS
                },
                Address::Add(add) => match add.base {
                    dynamic_number::Unsigned::Byte(_) => Self::ADD_BYTE_ADDRESS,
                    dynamic_number::Unsigned::Word(_) => Self::ADD_WORD_ADDRESS,
                    dynamic_number::Unsigned::DoubleWord(_) => Self::ADD_DOUBLE_WORD_ADDRESS,
                    dynamic_number::Unsigned::QuadWord(_) => Self::ADD_QUAD_WORD_ADDRESS
                },
                Address::Subtract(subtract) => match subtract.base {
                    dynamic_number::Unsigned::Byte(_) => Self::SUBTRACT_BYTE_ADDRESS, 
                    dynamic_number::Unsigned::Word(_) => Self::SUBTRACT_WORD_ADDRESS,
                    dynamic_number::Unsigned::DoubleWord(_) => Self::SUBTRACT_DOUBLE_WORD_ADDRESS,
                    dynamic_number::Unsigned::QuadWord(_) => Self::SUBTRACT_QUAD_WORD_ADDRESS
                }
            }
        }
    }
    
    pub fn decode(input: u8) -> Self {
        todo!()
    }
    
    /// Whether this dynamic operand contains a constant in its current state.
    pub fn contains_constant(self) -> bool {
        match self {
            Self::Constant(_) => true,
            Self::Address(address) => address.contains_constant(),
            Self::Register(_) => false
        }
    }
}