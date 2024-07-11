use crate::operand::dynamic::Dynamic;
use crate::operand::Name;
use crate::operand::register::Register;

pub mod operand;

/// The register and dynamic operand in one structure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegisterAndDynamic {
    /// The operand in which the result should be copied to.
    result: Name,
    register: Register,
    dynamic: Dynamic,
}

/// Error caused a result is set to point to a dynamic operand which is set to [Dynamic::Constant(_)].
#[derive(Debug)]
pub struct ConstantResultError;

impl RegisterAndDynamic {
    pub fn new(result: Name, register: Register, dynamic: Dynamic) -> Result<Self, ConstantResultError> {
        if matches!(result, Name::Dynamic) && matches!(dynamic, Dynamic::Constant(_)) { return Err(ConstantResultError) }
        Ok(Self { result, register, dynamic })
    }

    pub fn result(self) -> Name {
        self.result
    }

    pub fn register(self) -> Register {
        self.register
    }

    pub fn dynamic(self) -> Dynamic {
        self.dynamic
    }
}

/// Enum containing the valid combinations of the operand.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Combination {
    RegisterAndDynamic(RegisterAndDynamic),
    /// Exclusively the register operand.
    Register(Register),
    /// Exclusively the dynamic  operand.
    Dynamic(Dynamic)
}