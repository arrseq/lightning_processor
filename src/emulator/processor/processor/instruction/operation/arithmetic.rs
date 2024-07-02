use emulator::memory::Memory;
use emulator::processor::processor::{Context, Ports, Registers};
use emulator::processor::processor::instruction::operand::Dynamic;
use crate::emulator::processor;
use crate::emulator::processor::processor::instruction::Data;
use crate::emulator::processor::processor::instruction::operand::OperandsPresence;
use crate::emulator::processor::processor::instruction::operation::{Coded, Operation, OperationExecuteError};

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
    type CustomError = ();

    fn execute(&self, data: Option<&Data>, memory: &mut Memory, context: &mut Context, ports: &mut Ports) -> Result<(), OperationExecuteError<Self::CustomError>> {
        let data = data.ok_or(OperationExecuteError::Data(true))?;
        let all_operands = data.operands.all().ok_or(OperationExecuteError::Operand(OperandsPresence::AllPresent))?;
        let dynamic = all_operands.x_dynamic.read(&data.width, memory, context.virtual_mode, &context.registers).map_err(OperationExecuteError::DynamicRead)?;
        
        dbg!(dynamic);

        todo!();
    }

    fn presence(&self) -> Option<OperandsPresence> {
        Some(OperandsPresence::AllPresent)
    }
}

impl Coded<u8> for Arithmetic {
    fn code(&self) -> u8 {
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