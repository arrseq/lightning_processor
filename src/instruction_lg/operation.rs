use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arithmetic {
    Add,
    Subtract,
    CarryingAdd,
    BorrowingSubtract,
    Multiply,
    Divide
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Arithmetic(Arithmetic)
}

#[derive(Debug, Error)]
#[error("Invalid operation code")]
pub struct InvalidCodeError;

impl Operation {
    pub const ARITHMETIC_ADD: u16 = 0;
    pub const ARITHMETIC_CARRYING_ADD: u16 = 1;
    pub const ARITHMETIC_SUBTRACT: u16 = 2;
    pub const ARITHMETIC_BORROWING_SUBTRACT: u16 = 3;
    pub const ARITHMETIC_MULTIPLY: u16 = 4;
    pub const ARITHMETIC_DIVIDE: u16 = 5;

    pub fn encode(self) -> u16 {
        match self {
            Self::Arithmetic(arithmetic) => match arithmetic {
                Arithmetic::Add => Self::ARITHMETIC_ADD,
                Arithmetic::CarryingAdd => Self::ARITHMETIC_CARRYING_ADD,
                Arithmetic::Subtract => Self::ARITHMETIC_SUBTRACT,
                Arithmetic::BorrowingSubtract => Self::ARITHMETIC_BORROWING_SUBTRACT,
                Arithmetic::Multiply => Self::ARITHMETIC_MULTIPLY,
                Arithmetic::Divide => Self::ARITHMETIC_DIVIDE
            }
        }
    }
    
    pub fn decode(encoded: u16) -> Result<Self, InvalidCodeError> {
        Ok(match encoded {
            Self::ARITHMETIC_ADD => Self::Arithmetic(Arithmetic::Add),
            Self::ARITHMETIC_CARRYING_ADD => Self::Arithmetic(Arithmetic::CarryingAdd),
            Self::ARITHMETIC_SUBTRACT => Self::Arithmetic(Arithmetic::Subtract),
            Self::ARITHMETIC_BORROWING_SUBTRACT => Self::Arithmetic(Arithmetic::BorrowingSubtract),
            Self::ARITHMETIC_MULTIPLY => Self::Arithmetic(Arithmetic::Multiply),
            Self::ARITHMETIC_DIVIDE => Self::Arithmetic(Arithmetic::Divide),
            _ => return Err(InvalidCodeError)
        })
    }
    
    pub fn requires_operand(self) -> bool {
        match self {
            Self::Arithmetic(_) => true
        }
    }
}