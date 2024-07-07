use instruction::operand::dynamic::Dynamic;
use instruction::operand::register::Register;

pub mod dynamic;
pub mod register;
pub mod registers;

#[derive(Debug)]
pub struct Dual {
    pub r#static: Register,
    pub dynamic: Dynamic,
    /// The operand in which the primary result of the computation must be stored.
    pub destination: Type
}

/// Different configurations for the operands regarding the presence of individual operands.
#[derive(Debug)]
pub enum Configuration {
    Dual(Dual),
    Static(Register),
    Dynamic(Dynamic)
}

#[derive(Debug)]
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