use utility::{FromCode};
use crate::utility::{ToCode};

use super::register::{self, Register};

/// Package for adjacent static and dynamic registers.
#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pub r#static: Register,
    pub dynamic: Register
}

impl Registers {
    /// Encode both register codes adjacent to each other in a byte.
    /// ```
    /// use atln_processor::instruction::operand::register::Register;
    /// use atln_processor::instruction::operand::registers::Registers;
    /// use atln_processor::utility::TryFromCode;
    ///
    /// let registers = Registers {
    ///     r#static: Register::try_from_code(10).unwrap(),
    ///     dynamic: Register::try_from_code(7).unwrap()
    /// };
    /// 
    /// // TODO: Continue
    /// ```
    pub(crate) fn encode(&self) -> u8 {
        let mut output = self.dynamic.to_code();
        output |= self.r#static.to_code() << register::INDEX_BITS;
        output
    }
}

impl Registers {
    pub fn decode(input: u8) -> Self {
        let r#static = Register::from_code(input >> register::INDEX_BITS);
        let dynamic = Register::from_code(input);
        
        Self { r#static, dynamic }
    }
}