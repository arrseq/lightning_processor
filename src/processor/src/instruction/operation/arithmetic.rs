use crate::ExecutionContext;
use crate::instruction::{Data, Instruction};
use crate::instruction::operand::{Destination, Dynamic, Operands, Static};
use crate::instruction::operand::AllPresent;
use crate::instruction::operation::{Coded, Extension, Operation, OperationExecuteError};
use crate::number::Size;

// region: Constants
pub const ADD_CODE     : u8 = 0;
pub const SUBTRACT_CODE: u8 = 1;
// endregion

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Arithmetic {
    #[default]
    Add,
    Subtract
}

impl<'a> Operation<'a> for Arithmetic {
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