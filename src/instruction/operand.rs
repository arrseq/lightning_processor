use instruction::operand::dynamic::{Dynamic, SizedDynamic};
use instruction::operand::register::Register;
use number;
use utility::Encode;

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

pub type SizedDual = SizedOperand<Dual>;

impl Encode for SizedDual {
    type Output = u8;

    fn encode(&self) -> Self::Output {
        // [data size] [destination] [dynamic mode] [address mode] [address constant size]
        let data_size = self.data_size.exponent();
        let destination = bool::from(&self.operand.destination) as u8;
        
        
        let mut byte = 0u8;
        byte |= data_size << 6;
        byte |= destination << 5;
        
        byte
    }
}

pub type SizedStatic = SizedOperand<Register>;

/// Different configurations for the operands regarding the presence of individual operands.
#[derive(Debug, Clone, Copy)]
pub enum Configuration {
    Dual(SizedDual),
    Static(SizedStatic),
    Dynamic(SizedDynamic)
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

impl From<&Type> for bool {
    /// Convert an operand type to a boolean. If the operand is [Type::Dynamic] then [true] is returned.
    fn from(value: &Type) -> Self { matches!(value, Type::Dynamic) }
}