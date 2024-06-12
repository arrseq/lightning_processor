use crate::ExecutionContext;
use crate::instruction::Data;
use crate::instruction::operation::{Coded, Operation, OperationExecuteError};

// Operation codes

pub const ADD_CODE     : u8 = 0;
pub const SUBTRACT_CODE: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arithmetic {
    Add,
    Subtract
}

// Implementation

impl<'a> Operation<'a> for Arithmetic {
    fn name(&mut self) -> &'a str { "add" }

    fn expects_static(&mut self) -> bool { true }
    fn expects_dynamic(&mut self) -> bool { true }

    fn execute(&mut self, _code: u8, _data: Option<&Data>, context: &mut ExecutionContext) -> Result<(),
        OperationExecuteError> {
        context.accumulator = 100;
        Ok(())
    }
}

impl Coded<u8> for Arithmetic {
    fn code(&mut self) -> u8 {
        match self {
            Self::Add      => ADD_CODE,
            Self::Subtract => SUBTRACT_CODE
        }
    }
}

impl Arithmetic {
    pub fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            ADD_CODE      => Self::Add,
            SUBTRACT_CODE => Self::Subtract,
            _ => return None
        })
    }
}