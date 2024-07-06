use strum_macros::{EnumCount, FromRepr};
use strum::{EnumCount};
use instruction;
use instruction::OpCodeSize;
use utility::{MaxCode, ToCode, TryCoded, TryFromCode};


// region: Prefix implementation.
#[derive(Debug, Clone, Copy, FromRepr, EnumCount)]
#[repr(u8)]
pub enum Prefix {
    Synchronize,
    /// Repeat the current instruction a number of times which is determined by a register provided.
    Repeat
}

impl TryCoded for Prefix {
    type Code = u8;
}

impl TryFromCode for Prefix {
    type Code = u8;

    fn try_from_code(code: Self::Code) -> Option<Self> {
        Prefix::from_repr(code)
    }
}

impl ToCode for Prefix {
    type Code = u8;

    fn to_code(&self) -> Self::Code {
        *self as Self::Code
    }
}

impl MaxCode for Prefix {
    type Code = u8;

    fn max_code() -> Self::Code {
        Prefix::COUNT as Self::Code - 1
    }
    
    fn codes() -> Self::Code {
        Prefix::COUNT as Self::Code
    }
}
// endregion

// region: Operation code.
#[derive(Debug, Clone, Copy, FromRepr, EnumCount)]
#[repr(u16)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl TryFromCode for Operation {
    type Code = u16;

    fn try_from_code(code: Self::Code) -> Option<Self> {
        Operation::from_repr(code)
    }
}

impl ToCode for Operation {
    type Code = u16;

    fn to_code(&self) -> Self::Code {
        *self as Self::Code
    }
}

impl MaxCode for Operation {
    type Code = u16;

    fn max_code() -> Self::Code {
        Self::COUNT as Self::Code - 1
    }

    fn codes() -> Self::Code {
        Self::COUNT  as Self::Code
    }
}

impl TryCoded for Operation {
    type Code = u16;
}
// region

pub type Instruction = instruction::Instruction<Operation, Prefix>;