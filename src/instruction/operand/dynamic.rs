use crate::dynamic_number;
use super::operand::register::Register;

/// A tuple containing a register and a constant which will be operated on and then used to address memory.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Calculated {
    pub base: Register,
    pub offset: dynamic_number::Unsigned
}

/// A dynamic address dereferencing source target.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Address {
    /// Address memory with the dynamic operand's register field.
    Register(Register),

    /// Address memory with the emulator constant field.
    Constant(dynamic_number::Unsigned),

    /// Address memory with the sum of the dynamic operand's register field.
    Add(Calculated),

    /// Address memory with the difference of the dynamic operand's register field.
    Subtract(Calculated)
}

impl Address {
    pub fn register(self) -> Result<Register, NotIncludedError> {
        Ok(match self {
            Self::Register(register) => register,
            Self::Add(calculated)
            | Self::Subtract(calculated) => calculated.base,
            _ => return Err(NotIncludedError)
        }) 
    }
    
    pub fn constant(self) -> Result<dynamic_number::Unsigned, NotIncludedError> {
        Ok(match self {
            Self::Constant(constant) => constant,
            Self::Add(calculated) 
            | Self::Subtract(calculated) => calculated.offset,
            _ => return Err(NotIncludedError)
        })
    }
}

/// A dynamic source operand.
///
/// The address modes that involve a constant and are designed for a specific sized constant will have the constant
/// be constrained with [dynamic_number::Unsigned::resize] to fit that requirement.
///
/// # Encoding
/// A dynamic code refers to the specific algorithm or mode of the operand. The largest valid code is 14.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dynamic {
    Register(Register),
    Constant(dynamic_number::Unsigned),
    Address(Address)
}

/// Which combination of the register or constant is required.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Requirement {
    /// The operand requires both a register and constant. This includes the size of the constant if determined by the 
    /// dynamic operand.
    RegisterAndConstant(Option<dynamic_number::Size>),

    /// The operand exclusively requires the register.
    Register,

    /// The operand exclusively requires the constant. This includes the size of the constant if determined by the 
    /// dynamic operand.
    Constant(Option<dynamic_number::Size>)
}

impl Requirement {
    pub fn requires_constant(self) -> bool {
        !matches!(self, Self::Register)
    }
}

/// An invalid dynamic code was used for the specific task.
#[derive(Debug)]
pub struct InvalidCodeError;

#[derive(Debug)]
pub struct NotIncludedError;

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
    
    pub fn register(self) -> Result<Register, NotIncludedError> {
        Ok(match self {
            Self::Register(register) => register,
            Self::Address(address) => return address.register(),
            _ => return Err(NotIncludedError)
        })
    }
    
    pub fn constant(self) -> Result<dynamic_number::Unsigned, NotIncludedError> {
        Ok(match self {
            Self::Constant(constant) => constant,
            Self::Address(address) => return address.constant(),
            _ => return Err(NotIncludedError)
        })
    }

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
                Address::Add(add) => match add.offset {
                    dynamic_number::Unsigned::Byte(_) => Self::ADD_BYTE_ADDRESS,
                    dynamic_number::Unsigned::Word(_) => Self::ADD_WORD_ADDRESS,
                    dynamic_number::Unsigned::DoubleWord(_) => Self::ADD_DOUBLE_WORD_ADDRESS,
                    dynamic_number::Unsigned::QuadWord(_) => Self::ADD_QUAD_WORD_ADDRESS
                },
                Address::Subtract(subtract) => match subtract.offset {
                    dynamic_number::Unsigned::Byte(_) => Self::SUBTRACT_BYTE_ADDRESS,
                    dynamic_number::Unsigned::Word(_) => Self::SUBTRACT_WORD_ADDRESS,
                    dynamic_number::Unsigned::DoubleWord(_) => Self::SUBTRACT_DOUBLE_WORD_ADDRESS,
                    dynamic_number::Unsigned::QuadWord(_) => Self::SUBTRACT_QUAD_WORD_ADDRESS
                }
            }
        }
    }

    /// Decode dynamic operands that exclusively contain a register.
    ///
    /// # Result
    /// If an invalid dynamic code or a code that refers to a dynamic operand mode that does not exclusively use a
    /// register or is invalid, then [Err(InvalidCodeError)] is returned.
    pub fn decode_register(encoded: u8, register: Register) -> Result<Self, InvalidCodeError> {
        Ok(match encoded {
            Self::REGISTER => Self::Register(register),
            Self::REGISTER_ADDRESS => Self::Address(Address::Register(register)),
            _ => return Err(InvalidCodeError)
        })
    }

    /// Decode dynamic operand modes that exclusively contain a constant.
    /// 
    /// If the data is part of the address dynamic mode, then the constant will be resized to fit the address mode's 
    /// corresponding size.
    ///
    /// # Result
    /// If the operand type is invalid or doesn't exclusively support a constant, then [Err(InvalidCodeError)] is 
    /// returned.
    pub fn decode_constant(encoded: u8, constant: dynamic_number::Unsigned) -> Result<Self, InvalidCodeError> {
        Ok(Self::Address(match encoded {
            Self::CONSTANT => return Ok(Self::Constant(constant)),
            Self::CONSTANT_BYTE_ADDRESS => Address::Constant(constant.resize(dynamic_number::Size::Byte)),
            Self::CONSTANT_WORD_ADDRESS => Address::Constant(constant.resize(dynamic_number::Size::Word)),
            Self::CONSTANT_DOUBLE_WORD_ADDRESS => Address::Constant(constant.resize(dynamic_number::Size::DoubleWord)),
            Self::CONSTANT_QUAD_WORD_ADDRESS => Address::Constant(constant.resize(dynamic_number::Size::QuadWord)),
            _ => return Err(InvalidCodeError)
        }))
    }

    /// Decode dynamic operand mode that exclusively contains a calculated structure. This always returns an instance
    /// containing [Address] because the address mode is the only one which has modes which contain a [Calculated]
    /// structure.
    ///
    /// # Result
    /// If the operand type is invalid or doesn't exclusively support a calculated structure, then
    /// [Err(InvalidCodeError)] is returned.
    pub fn decode_calculated(encoded: u8, calculated: Calculated) -> Result<Self, InvalidCodeError> {
        Ok(Self::Address(match encoded {
            Self::ADD_BYTE_ADDRESS
            | Self::ADD_WORD_ADDRESS
            | Self::ADD_DOUBLE_WORD_ADDRESS
            | Self::ADD_QUAD_WORD_ADDRESS => Address::Add(calculated),
            Self::SUBTRACT_BYTE_ADDRESS
            | Self::SUBTRACT_WORD_ADDRESS
            | Self::SUBTRACT_DOUBLE_WORD_ADDRESS
            | Self::SUBTRACT_QUAD_WORD_ADDRESS => Address::Subtract(calculated),
            _ => return Err(InvalidCodeError)
        }))
    }

    /// Get the requirements of a specific dynamic operand.
    ///
    /// # Result
    /// If the dynamic operand is invalid, then [Err(InvalidCodeError)] is returned.
    ///
    /// # Example
    /// ```
    /// use arrseq_instruction::operand::dynamic::{Address, Calculated, Dynamic, Requirement};
    /// use arrseq_instruction::operand::register::{Register, SideInput};
    /// use crate::dynamic_number;
    ///
    /// assert_eq!(Dynamic::requirement(Dynamic::Constant(dynamic_number::Unsigned::Word(u16::MAX)).encode()).unwrap(), Requirement::Constant);
    /// assert_eq!(Dynamic::requirement(Dynamic::Address(Address::Add(Calculated {
    ///     base: Register::SideInput(SideInput::First),
    ///     offset: dynamic_number::Unsigned::Word(u16::MAX)
    /// })).encode()).unwrap(), Requirement::RegisterAndConstant);
    /// ```
    pub fn requirement(encoded: u8) -> Result<Requirement, InvalidCodeError> {
        Ok(match encoded {
            Self::REGISTER
            | Self::REGISTER_ADDRESS => Requirement::Register,
            Self::CONSTANT
            | Self::CONSTANT_BYTE_ADDRESS
            | Self::CONSTANT_WORD_ADDRESS
            | Self::CONSTANT_DOUBLE_WORD_ADDRESS
            | Self::CONSTANT_QUAD_WORD_ADDRESS => Requirement::Constant(None),
            Self::ADD_BYTE_ADDRESS
            | Self::SUBTRACT_BYTE_ADDRESS => Requirement::RegisterAndConstant(Some(dynamic_number::Size::Byte)),
            Self::ADD_WORD_ADDRESS
            | Self::SUBTRACT_WORD_ADDRESS => Requirement::RegisterAndConstant(Some(dynamic_number::Size::Word)),
            Self::ADD_DOUBLE_WORD_ADDRESS
            | Self::SUBTRACT_DOUBLE_WORD_ADDRESS => Requirement::RegisterAndConstant(Some(dynamic_number::Size::DoubleWord)),
            Self::ADD_QUAD_WORD_ADDRESS
            | Self::SUBTRACT_QUAD_WORD_ADDRESS => Requirement::RegisterAndConstant(Some(dynamic_number::Size::QuadWord)),
            _ => return Err(InvalidCodeError)
        })
    }

    pub fn is_valid(encoded: u8) -> bool {
        encoded <= 14
    }
}