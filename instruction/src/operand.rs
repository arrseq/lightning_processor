use std::io::Read;
use arrseq_memory::dynamic_number;
use crate::operand::dynamic::Dynamic;
use crate::operand::register::Register;

pub mod dynamic;
pub mod register;

/// Named of the 2 supported operands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Name {
    /// Register only operands.
    Register,
    
    /// Dynamically addressed operand. This operand could potentially refer to one of many things.
    Dynamic
}

/// The static and dynamic operand in one structure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StaticAndDynamic {
    /// The operand in which the result should be copied to.
    pub result: Name,
    pub r#static: Register,
    pub dynamic: Dynamic,
}

/// Enum containing the valid combinations of the operand.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Combination {
    StaticAndDynamic(StaticAndDynamic),
    /// Exclusively the static register.
    Static(Register),
    /// Exclusively the dynamic  operand.
    Dynamic(Dynamic)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operands {
    /// The size of the data that the operands refer to.
    pub size: dynamic_number::Size,
    
    /// The operands in their valid combination.
    pub combination: Combination
}

impl Operands {
    /// ```
    /// use arrseq_instruction::operand;
    /// use arrseq_instruction::operand::{Combination, Operands, StaticAndDynamic};
    /// use arrseq_instruction::operand::dynamic::Dynamic;
    /// use arrseq_instruction::operand::register::Register;
    /// use arrseq_memory::dynamic_number;
    ///
    /// let operands = Operands {
    ///     size: dynamic_number::Size::Word,
    ///     combination: Combination::StaticAndDynamic(StaticAndDynamic {
    ///         result: operand::Name::Register,
    ///         r#static: Register::Accumulator,
    ///         dynamic: Dynamic::Constant(dynamic_number::Unsigned::Word(10))
    ///     })
    /// };
    /// ```
    pub fn decode(input: &mut impl Read) -> Self {
        todo!()
    }
}