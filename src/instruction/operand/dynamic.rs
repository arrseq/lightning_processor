use std::io::Read;
use strum_macros::FromRepr;
use instruction::operand::{DualDecodeError, SizedOperand};
use number;
use number::Size;
use utility::{ToCode};
use crate::number::Number;

use super::register::Register;

pub const MODE_BITS: u8 = 5;
pub const MODES: u8 = 2u8.pow(MODE_BITS as u32);

#[derive(Debug, Clone, Copy)]
pub struct Dual {
    pub offset: Number,
    pub base: Register
}

#[derive(Debug, Clone, Copy)]
pub enum Address {
    Register(Register),
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
    AddressAddQuad,
    AddressSubtractByte,
    AddressSubtractWord,
    AddressSubtractDual,
    AddressSubtractQuad
}

impl Code {
    pub fn requires_constant(self) -> bool {
        matches!(self, Self::Constant)
    }

    pub fn required_address_constant(self) -> bool {
        match self {
            Self::AddressConstantByte
                | Self::AddressConstantWord
                | Self::AddressConstantDual
                | Self::AddressConstantQuad => true,
            Self::Register
                | Self::Constant
                | Self::AddressRegister
                | Self::AddressAddByte
                | Self::AddressAddWord
                | Self::AddressAddDual
                | Self::AddressAddQuad
                | Self::AddressSubtractByte
                | Self::AddressSubtractWord
                | Self::AddressSubtractDual
                | Self::AddressSubtractQuad => false
        }
    }

    pub fn address_constant_size(self) -> Option<number::Size> {
        Some(match self {
            Self::AddressConstantByte
                | Self::AddressAddByte
                | Self::AddressSubtractByte => number::Size::Byte,
            Self::AddressConstantWord
                | Self::AddressAddWord
                | Self::AddressSubtractWord => number::Size::Word,
            Self::AddressConstantDual
                | Self::AddressAddDual
                | Self::AddressSubtractDual => number::Size::Dual,
            Self::AddressConstantQuad
                | Self::AddressAddQuad
                | Self::AddressSubtractQuad => number::Size::Quad,
            Self::Register
                | Self::Constant
                | Self::AddressRegister => return None
        })
    }

    pub fn requires_generic_constant(self) -> bool {
        self.required_address_constant() || self.requires_constant()
    }
}

impl From<Dynamic> for Code {
    fn from(value: Dynamic) -> Self {
        match value {
            Dynamic::Register(_) => Self::Register,
            Dynamic::Constant(_) => Self::Constant,
            Dynamic::Address(address) => match address {
                Address::Register(_) => Self::AddressRegister,
                Address::Constant(number) => match number {
                    Number::Byte(_) => Self::AddressConstantByte,
                    Number::Word(_) => Self::AddressConstantWord,
                    Number::Dual(_) => Self::AddressConstantDual,
                    Number::Quad(_) => Self::AddressConstantQuad
                },
                Address::Add(add) => match add.offset {
                    Number::Byte(_) => Self::AddressAddByte,
                    Number::Word(_) => Self::AddressAddWord,
                    Number::Dual(_) => Self::AddressAddDual,
                    Number::Quad(_) => Self::AddressAddQuad
                },
                Address::Subtract(subtract) => match subtract.offset {
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

impl Dynamic {
    pub fn get_register(self) -> Option<Register> {
        Some(match self {
            Self::Register(x) => x,
            Self::Address(address) => match address {
                Address::Register(x) => x,
                Address::Add(add) => add.base,
                Address::Subtract(subtract) => subtract.base,
                Address::Constant(_) => return None
            },
            Self::Constant(_) => return None
        })
    }
    
    /// Get the constant field used by the address dynamic mode. 
    pub fn get_address_constant(self) -> Option<Number> {
        if let Self::Address(address) = self {
            return Some(match address {
                Address::Constant(x) => x,
                Address::Add(dual)
                    | Address::Subtract(dual) => dual.offset,
                Address::Register(_) => return None
            })
        }
        
        None
    }
    
    /// Get the constant field from the constant addressing mode.
    pub fn get_constant(self) -> Option<Number> {
        if let Self::Constant(x) = self { Some(x) } else { None }
    }

    pub fn decode(code: Code, constant: Number, dynamic_register: Register) -> Self {
        match code {
            Code::Register => Self::Register(dynamic_register),
            Code::Constant => Self::Constant(constant),
            Code::AddressRegister => Self::Address(Address::Register(dynamic_register)),
            Code::AddressConstantByte => Self::Address(Address::Constant(constant.resize(Size::Byte))),
            Code::AddressConstantWord => Self::Address(Address::Constant(constant.resize(Size::Word))),
            Code::AddressConstantDual => Self::Address(Address::Constant(constant.resize(Size::Dual))),
            Code::AddressConstantQuad => Self::Address(Address::Constant(constant.resize(Size::Quad))),
            Code::AddressAddByte => Self::Address(Address::Add(Dual { base: dynamic_register, offset: constant.resize(Size::Byte) })),
            Code::AddressAddWord => Self::Address(Address::Add(Dual { base: dynamic_register, offset: constant.resize(Size::Word) })),
            Code::AddressAddDual => Self::Address(Address::Add(Dual { base: dynamic_register, offset: constant.resize(Size::Dual) })),
            Code::AddressAddQuad => Self::Address(Address::Add(Dual { base: dynamic_register, offset: constant.resize(Size::Quad) })),
            Code::AddressSubtractByte => Self::Address(Address::Subtract(Dual { base: dynamic_register, offset: constant.resize(Size::Byte) })),
            Code::AddressSubtractWord => Self::Address(Address::Subtract(Dual { base: dynamic_register, offset: constant.resize(Size::Word) })),
            Code::AddressSubtractDual => Self::Address(Address::Subtract(Dual { base: dynamic_register, offset: constant.resize(Size::Dual) })),
            Code::AddressSubtractQuad => Self::Address(Address::Subtract(Dual { base: dynamic_register, offset: constant.resize(Size::Quad) }))
        }
    }
}

impl ToCode for Dynamic {
    type Code = u8;

    fn to_code(&self) -> Self::Code {
        Code::from(*self) as Self::Code
    }
}

pub type SizedDynamic = SizedOperand<Dynamic>;

impl SizedDynamic {
    pub fn encode(&self) -> u8 {
        self.encode_operand_properties(None, Some(self.operand))
    }

    pub fn decode<Input: Read>(input: &mut Input) -> Result<Self, DualDecodeError> {
        let meta = Self::decode_operand_properties(input).map_err(DualDecodeError::SizedOperand)?;

        Ok(Self {
            data_size: meta.3,
            operand: meta.1.ok_or(DualDecodeError::MissingDynamic)?
        })
    }
}