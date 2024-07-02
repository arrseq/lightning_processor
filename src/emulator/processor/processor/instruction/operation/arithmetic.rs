use emulator::memory::Memory;
use emulator::processor::processor::{Context, ExternalContext, Ports, Registers};
use emulator::processor::processor::instruction::operand::{Destination, Dynamic};
use number;
use number::{CheckedAdd, CheckedMul, CheckedSub};
use crate::emulator::processor;
use crate::emulator::processor::processor::instruction::Data;
use crate::emulator::processor::processor::instruction::operand::OperandsPresence;
use crate::emulator::processor::processor::instruction::operation::{Coded, Operation, OperationExecuteError};

// region: Constants
pub const ADD_CODE     : u8 = 0;
pub const SUBTRACT_CODE: u8 = 1;
pub const MULTIPLY_CODE: u8 = 2;
pub const DIVIDE_CODE  : u8 = 3;
// endregion

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Arithmetic {
    #[default]
    Add,
    Subtract,
    Multiply, 
    Divide
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecuteError {
    /// A calculation resulted in an overflow.
    Overflow
}

impl<'a> Operation<'a> for Arithmetic {
    type CustomError = ExecuteError;

    fn execute(&self, data: Option<&Data>, context: &mut Context, external_context: &mut ExternalContext) -> Result<(), OperationExecuteError<Self::CustomError>> {
        let data = data.ok_or(OperationExecuteError::Data(true))?;
        let all_operands = data.operands.all().ok_or(OperationExecuteError::Operand(OperandsPresence::AllPresent))?;
        let r#static = number::Data::from_size_selecting(&data.width, *context.registers.get(all_operands.x_static as usize).ok_or(OperationExecuteError::InvalidStaticRegister)?);
        let dynamic = all_operands.x_dynamic.read(&data.width, &mut external_context.memory, context.virtual_mode, &context.registers).map_err(OperationExecuteError::DynamicRead)?;

        let result = match self {
            Self::Add => r#static.checked_add(dynamic.into_owned()),
            Self::Subtract => r#static.checked_sub(dynamic.into_owned()),
            Self::Multiply => r#static.checked_mul(dynamic.into_owned()),
            Self::Divide => r#static.checked_mul(dynamic.into_owned()),
            _ => todo!()
        }
            .ok_or(OperationExecuteError::Custom(ExecuteError::Overflow))?;
        
        match data.destination {
            Destination::Static => *context.registers.get_mut(all_operands.x_static as usize).unwrap() = result.quad(),
            Destination::Dynamic => all_operands.x_dynamic
                .write(&data.width, &mut external_context.memory, context.virtual_mode, &mut context.registers, result)
                .map_err(OperationExecuteError::DynamicRead)?
        };
        
        Ok(())
    }

    fn presence(&self) -> Option<OperandsPresence> {
        Some(OperandsPresence::AllPresent)
    }
}

impl Coded<u8> for Arithmetic {
    fn code(&self) -> u8 {
        match self {
            Self::Add      => ADD_CODE,
            Self::Subtract => SUBTRACT_CODE,
            Self::Multiply => MULTIPLY_CODE,
            Self::Divide   => DIVIDE_CODE
        }
    }
}

impl Arithmetic {
    pub fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            ADD_CODE      => Self::Add,
            SUBTRACT_CODE => Self::Subtract,
            MULTIPLY_CODE => Self::Multiply,
            DIVIDE_CODE   => Self::Divide,
            _ => return None
        })
    }
}