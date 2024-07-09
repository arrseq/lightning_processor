use std::io;
use std::io::Read;
use instruction::operand::dynamic::{Dynamic, SizedDynamic};
use instruction::operand::register::Register;
use instruction::operand::registers::Registers;
use number;
use number::Number;
use utility::{ToCode};

pub mod dynamic;
pub mod register;
pub mod registers;

#[derive(Debug, Clone, Copy)]
pub struct Dual {
    pub r#static: Register,
    pub dynamic: Dynamic,
    /// The operand in which the primary result of the computation must be stored.
    pub destination: Type
}

#[derive(Debug, Clone, Copy)]
pub struct SizedOperand<Operand> {
    pub operand: Operand,
    pub data_size: number::Size
}

#[derive(Debug)]
pub enum DecodeError {
    Read(io::Error)
}

impl<Operand> SizedOperand<Operand> {
    pub fn encode_operand_properties(self, destination: Option<Type>, dynamic_operand: Option<Dynamic>) -> u8 {
        let data_size = self.data_size.exponent();

        // Default to static operand destination. In these cases, the destination is irrelevant.
        let destination = bool::from(destination.unwrap_or(Type::Static)) as u8;

        let mut byte = 0u8;
        byte |= data_size << 6;
        byte |= destination << 5;

        if let Some(dynamic_operand) = dynamic_operand {
            let addressing = dynamic_operand.to_code();
            byte |= addressing << 1;
        }

        byte
    }

    pub fn decode_operand_properties<Input: Read>(input: &mut Input) -> Result<(Option<Type>, Option<Dynamic>, Register, number::Size), DecodeError> {
        let mut buffer = [0u8; 2];
        input.read_exact(&mut buffer).map_err(DecodeError::Read)?;

        let info = buffer[0];
        let registers = Registers::decode(buffer[1]);

        // Due to bitwise this is always valid and can be unwrapped.
        let data_size = number::Size::from_exponent((info & 0b11_0_0000_0) >> 6).unwrap();
        let destination = Type::from(((info & 0b00_1_0000_0) >> 5) == 1);
        // There can never be an invalid dynamic code due to the bitwise operations.
        let dynamic_code = dynamic::Code::from_repr((info & 0b00_0_1111_0) >> 1).unwrap();
        
        if dynamic_code.requires_generic_constant() {
            // Set up a buffer to read the constant at the correct size.
            let mut max_constant_buffer = [0u8; 8];
            let buffer = if dynamic_code.requires_constant() {
                data_size.buffer(&mut max_constant_buffer)
            } else {
                // The dynamic operand is valid and confirmed to use a constant. This is safe to unwrap.
                let operand_size = dynamic_code.address_constant_size().unwrap();
                operand_size.buffer(&mut max_constant_buffer)
            };
            
            // Read the constant and make a number.
            input.read_exact(buffer).map_err(DecodeError::Read)?;
            let constant = Number::from_buffer(buffer).unwrap();
            
            // Generate a dynamic operand.
            let dynamic = Dynamic::decode(dynamic_code, constant, registers.dynamic);
            return Ok((Some(destination), Some(dynamic), registers.r#static, data_size));
        }

        Ok((Some(destination), None, registers.r#static, data_size))
    }
}

pub type SizedDual = SizedOperand<Dual>;

#[derive(Debug)]
pub enum DualDecodeError {
    SizedOperand(DecodeError),
    MissingDynamic,
    MissingDestination
}

impl SizedDual {
    pub fn encode(&self) -> u8 {
        self.encode_operand_properties(Some(self.operand.destination), Some(self.operand.dynamic))
    }
    
    pub fn decode<Input: Read>(input: &mut Input) -> Result<Self, DualDecodeError> {
        let meta = Self::decode_operand_properties(input).map_err(DualDecodeError::SizedOperand)?;
        
        Ok(Self {
            data_size: meta.3,
            operand: Dual {
                destination: meta.0.ok_or(DualDecodeError::MissingDestination)?,
                dynamic: meta.1.ok_or(DualDecodeError::MissingDynamic)?,
                r#static: meta.2
            }
        })
    }
}

pub type SizedStatic = SizedOperand<Register>;

impl SizedStatic {
    pub fn encode(&self) -> u8 {
        self.encode_operand_properties(None, None)
    }

    pub fn decode<Input: Read>(input: &mut Input) -> Result<Self, DualDecodeError> {
        let meta = Self::decode_operand_properties(input).map_err(DualDecodeError::SizedOperand)?;

        Ok(Self {
            data_size: meta.3,
            operand: meta.2
        })
    }
}

/// Different configurations for the operands regarding the presence of individual operands.
#[derive(Debug, Clone, Copy)]
pub enum Configuration {
    Dual(SizedDual),
    Static(SizedStatic),
    Dynamic(SizedDynamic)
}

#[derive(Debug, Clone, Copy)]
pub enum ConfigurationCode {
    Dual,
    Static,
    Dynamic
}

impl Configuration {
    pub fn get_static_register(self) -> Option<Register> {
        Some(match self {
            Self::Dual(x) => x.operand.r#static,
            Self::Static(x) => x.operand,
            Self::Dynamic(_) => return None
        })
    }
    
    pub fn get_dynamic(self) -> Option<Dynamic> {
        Some(match self {
            Self::Dual(x) => x.operand.dynamic,
            Self::Dynamic(x) => x.operand,
            Self::Static(_) => return None
        })
    }
    
    pub fn get_size(self) -> number::Size {
        match self {
            Self::Dual(x) => x.data_size,
            Self::Static(x) => x.data_size,
            Self::Dynamic(x) => x.data_size
        }
    }
}

pub trait GetConfiguration {
    /// Get the configuration of the current operation being references.
    fn get_configuration(&self) -> Option<Configuration>;
}

pub trait GetCodeConfiguration {
    fn get_code_configuration(&self) -> Option<ConfigurationCode>;
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Static,
    Dynamic
}

impl From<bool> for Type {
    /// Convert a boolean to an operand type. If the value is [true] then [Type::Dynamic] is returned.
    fn from(value: bool) -> Self { if value { Self::Dynamic } else { Self::Static } }
}

impl From<Type> for bool {
    /// Convert an operand type to a boolean. If the operand is [Type::Dynamic] then [true] is returned.
    fn from(value: Type) -> Self { matches!(value, Type::Dynamic) }
}