use instruction::operand::dynamic::{Dynamic, SizedDynamic};
use instruction::operand::register::Register;
use number;
use utility::{Encode, ToCode};

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
}

pub type SizedDual = SizedOperand<Dual>;

impl Encode for SizedDual {
    type Output = u8;

    fn encode(&self) -> Self::Output {
        self.encode_operand_properties(Some(self.operand.destination), Some(self.operand.dynamic))
    }
}

pub type SizedStatic = SizedOperand<Register>;

impl Encode for SizedStatic {
    type Output = u8;

    fn encode(&self) -> Self::Output {
        self.encode_operand_properties(None, None)
    }
}

/// Different configurations for the operands regarding the presence of individual operands.
#[derive(Debug, Clone, Copy)]
pub enum Configuration {
    Dual(SizedDual),
    Static(SizedStatic),
    Dynamic(SizedDynamic)
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