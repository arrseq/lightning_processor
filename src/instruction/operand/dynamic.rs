use strum_macros::FromRepr;
use instruction::operand::SizedOperand;
use number::high::HighNumber;
use utility::ToCode;
use crate::number::Number;

use super::register::Register;

impl From<&HighNumber> for u64 {
    fn from(value: &HighNumber) -> Self {
        match value { 
            HighNumber::Dual(v) => *v as u64,
            HighNumber::Quad(v) => *v
        }
    } 
}

#[derive(Debug, Clone, Copy)]
pub struct Added {
    pub constant: HighNumber,
    pub offset: Register
}

#[derive(Debug, Clone, Copy)]
pub enum Address {
    Constant(HighNumber),
    Register,
    /// Address mode where the register value and constant are added before being used to dereferencing memory,
    Added(Added)
}

#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u8)]
pub enum Code {
    Register,
    Constant,
    Address
}

impl From<&Dynamic> for Code {
    fn from(value: &Dynamic) -> Self {
        match value {
            Dynamic::Register(_) => Self::Register,
            Dynamic::Constant(_) => Self::Constant,
            Dynamic::Address(_) => Self::Address
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dynamic {
    Register(Register),
    Constant(Number),
    Address(Address)
}

impl ToCode for Dynamic {
    type Code = u8;

    fn to_code(&self) -> Self::Code {
        Code::from(self) as Self::Code
    }
}

pub type SizedDynamic = SizedOperand<Dynamic>;