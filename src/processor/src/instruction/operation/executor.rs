use crate::ExecutionContext;
use crate::instruction::Data;
use crate::instruction::operation::{Coded, Operation, OperationExecuteError};

// region: Constants
pub const HALT_CODE: u8 = 0;
pub const DIVERT_CODE: u8 = 1;
// endregion

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Executor {
    Halt,
    Divert
}

impl<'a> Operation<'a> for Executor {
    fn expects_static(&mut self) -> bool {
        false
    }

    fn expects_dynamic(&mut self) -> bool { 
        match self {
            Self::Divert => true,
            _ => false
        }
    }

    fn execute(&mut self, _code: u8, _data: Option<&Data>, context: &mut ExecutionContext) -> Result<(),
        OperationExecuteError> {
        context.accumulator = 100;
        Ok(())
    }
}

impl Coded<u8> for Executor {
    fn code(&mut self) -> u8 {
        match self {
            Self::Halt => HALT_CODE,
            Self::Divert => DIVERT_CODE
        }
    }
}

impl Executor {
    pub fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            HALT_CODE => Self::Halt,
            DIVERT_CODE => Self::Divert,
            _ => return None
        })
    }
}