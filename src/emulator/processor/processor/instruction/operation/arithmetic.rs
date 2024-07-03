use std::borrow::Cow;
use emulator::memory::Memory;
use emulator::processor::processor::{Context, ExternalContext, Ports, Registers};
use emulator::processor::processor::instruction::operand::{Destination, Dynamic};
use number;
use number::{CarryingAdd, CarryingSub, CheckedAdd, CheckedMul, CheckedSub, WrappingAdd, WrappingDiv, WrappingMul, WrappingSub};
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
    Divide,
    CarryingAdd,
    BorrowingSub,
}

/// Flags that can be set or will be set by this unit. Flags could be used by instructions that read the flags. Specific 
/// instructions can be used to read the flags. 
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Flags {
    pub overflow: bool,
    pub cary: bool,
    pub zero: bool,
    pub sign: bool
}

impl<'a> Operation<'a> for Arithmetic {
    type CustomError = ();

    fn execute<X: AsRef<[u8]> + AsMut<[u8]>>(&self, data: Option<&Data>, context: &mut Context, external_context: &mut ExternalContext<X>) -> Result<(), OperationExecuteError<Self::CustomError>> {
        let data = data.ok_or(OperationExecuteError::Data(true))?;
        let all_operands = data.operands.all().ok_or(OperationExecuteError::Operand(OperandsPresence::AllPresent))?;
        let r#static = number::Data::from_size_selecting(&data.width, *context.registers.get(all_operands.x_static as usize).ok_or(OperationExecuteError::InvalidStaticRegister)?);
        let dynamic = &*all_operands.x_dynamic
            .read(&data.width, &external_context.memory, context.virtual_mode, &context.registers).map_err(OperationExecuteError::DynamicRead)?;
        
        context.arithmetic_flags.overflow = match self {
            Self::Add | Arithmetic::CarryingAdd => r#static.checked_add(dynamic),
            Self::Subtract | Arithmetic::BorrowingSub => r#static.checked_sub(dynamic),
            Self::Multiply => r#static.checked_mul(dynamic),
            Self::Divide => r#static.checked_mul(dynamic)
        }.is_none();

        context.arithmetic_flags.cary = match self {
            Self::Add => r#static.carrying_add(dynamic, false).unwrap().1,
            Self::Subtract => r#static.carrying_sub(dynamic, false).unwrap().1,
            Self::CarryingAdd => r#static.carrying_add(dynamic, context.arithmetic_flags.cary).unwrap().1,
            Self::BorrowingSub => r#static.carrying_add(dynamic, context.arithmetic_flags.cary).unwrap().1,
            _ => false
        };
        
        let result = match self {
            Self::Add | Self::CarryingAdd => r#static.wrapping_add(dynamic),
            Self::Subtract | Self::BorrowingSub => r#static.wrapping_sub(dynamic),
            Self::Multiply => r#static.wrapping_mul(dynamic),
            Self::Divide => r#static.wrapping_div(dynamic)
        };
        
        context.arithmetic_flags.zero = result.is_zero();
        
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
            Self::Divide   => DIVIDE_CODE,
            _ => todo!() // TODO
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
            // TODO
            _ => return None
        })
    }
}