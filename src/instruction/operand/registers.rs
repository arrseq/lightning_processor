use utility::{FromCode};
use crate::utility::{Encode, ToCode};

use super::register::{self, Register};

/// Package for adjacent static and dynamic registers.
pub struct Registers {
    pub r#static: Register,
    pub dynamic: Register
}

impl Encode for Registers {
    type Output = u8;

    /// Encode both register codes adjacent to each other in a byte.
    /// ```
    /// use atln_processor::instruction::registers::Registers;
    /// use atln_processor::instruction::register::Register;
    /// use atln_processor::utility::TryFromCode;
    ///
    /// let registers = Registers {
    ///     r#static: Register::try_from_code(10).unwrap(),
    ///     dynamic: Register::try_from_code(0).unwrap()
    /// };
    /// ```
    fn encode(&self) -> Self::Output {
        let mut output = 0;
        output |= self.dynamic.to_code();
        output |= self.r#static.to_code() << register::INDEX_BITS;
        output
    }
}

impl Registers {
    fn decode(input: u8) -> Self {
        let r#static = Register::from_code(input >> register::INDEX_BITS);
        let dynamic = Register::from_code(input);
        
        Self { r#static, dynamic }
    }
}