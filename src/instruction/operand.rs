use instruction::operand::dynamic::Dynamic;
use instruction::operand::register::Register;

pub mod dynamic;
pub mod register;
pub mod registers;

pub struct Dual {
    pub r#static: Register,
    pub dynamic: Dynamic,
    /// The operand in which the primary result of the computation must be stored.
    pub destination: Type
}

/// Different configurations for the operands regarding the presence of individual operands.
pub enum Configuration {
    Dual(Dual),
    Static(Register),
    Dynamic(Dynamic)
}

pub enum Type {
    Static,
    Dynamic
}