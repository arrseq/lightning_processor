use strum_macros::FromRepr;
use instruction::operand::SizedOperand;
use number::high::HighNumber;
use utility::ToCode;
use crate::number::Number;

use super::register::Register;

pub const MODE_BITS: u8 = 5;
pub const MODES: u8 = 2u8.pow(MODE_BITS as u32);

#[derive(Debug, Clone, Copy)]
pub struct Dual {
    pub constant: Number,
    pub offset: Register
}

#[derive(Debug, Clone, Copy)]
pub enum Address {
    Register,
    Constant(Number),
    /// Addressing mode where the register value and constant are added before being used to dereferencing memory.
    Add(Dual),
    /// Addressing mode where the constant is subtracted from the register value before being used to dereference 
    /// memory.
    Subtract(Dual)
}

#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u8)]
pub enum Code {
    Register,
    Constant,
    AddressRegister,
    AddressConstantByte,
    AddressConstantWord,
    AddressConstantDual,
    AddressConstantQuad,
    AddressAddByte,
    AddressAddWord,
    AddressAddDual,
    AddressAddQuad,AddressSubtractByte,
    AddressSubtractWord,
    AddressSubtractDual,
    AddressSubtractQuad
}

impl From<&Dynamic> for Code {
    fn from(value: &Dynamic) -> Self {
        match value {
            Dynamic::Register(_) => Self::Register,
            Dynamic::Constant(_) => Self::Constant,
            Dynamic::Address(address) => match address {
                Address::Register => Self::AddressRegister,
                Address::Constant(number) => match number {
                    Number::Byte(_) => Self::AddressConstantByte,
                    Number::Word(_) => Self::AddressConstantWord,
                    Number::Dual(_) => Self::AddressConstantDual,
                    Number::Quad(_) => Self::AddressConstantQuad
                },
                Address::Add(add) => match add.constant {
                    Number::Byte(_) => Self::AddressAddByte,
                    Number::Word(_) => Self::AddressAddWord,
                    Number::Dual(_) => Self::AddressAddDual,
                    Number::Quad(_) => Self::AddressAddQuad
                },
                Address::Subtract(subtract) => match subtract.constant {
                    Number::Byte(_) => Self::AddressSubtractByte,
                    Number::Word(_) => Self::AddressSubtractWord,
                    Number::Dual(_) => Self::AddressSubtractDual,
                    Number::Quad(_) => Self::AddressSubtractQuad
                }
            }
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