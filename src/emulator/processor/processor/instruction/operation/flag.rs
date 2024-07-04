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
pub const JUMP_IF_OVERFLOW_CODE      : u8 = 0;
pub const JUMP_IF_NOT_OVERFLOW_CODE  : u8 = 1;
pub const JUMP_IF_REGROUPING_CODE    : u8 = 2;
pub const JUMP_IF_NOT_REGROUPING_CODE: u8 = 3;
pub const JUMP_IF_ZERO_CODE          : u8 = 4;
pub const JUMP_IF_NOT_ZERO           : u8 = 5;
pub const JUMP_IF_SIGN               : u8 = 6;
pub const JUMP_IF_NOT_SIGN           : u8 = 7;
// endregion

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Flag {
    JumpIfOverflow,
    JumpIfNotOverflow,
    JumpIfRegrouping,
    JumpIfNotRegrouping,
    JumpIfZero,
    JumpIfNotZero,
    JumpIfSign,
    JumpIfNotSign
}

impl<'a> Operation<'a> for Flag {
    fn execute<X: AsRef<[u8]> + AsMut<[u8]>>(&self, data: Option<&Data>, context: &mut Context, external_context: &mut ExternalContext<X>) -> Result<(), OperationExecuteError> {
        let data = data.ok_or(OperationExecuteError::Data(true))?;
        let all_operands = data.operands.all().ok_or(OperationExecuteError::Operand(OperandsPresence::AllPresent))?;
        let r#static = number::Data::from_size_selecting(&data.width, *context.registers.get(all_operands.x_static as usize).ok_or(OperationExecuteError::InvalidStaticRegister)?)
            .resize(&data.width);
        let dynamic = &(*all_operands.x_dynamic
            .read(&data.width, &external_context.memory, context.virtual_mode, &context.registers).map_err(OperationExecuteError::DynamicRead)?)
            .resize(&data.width);
        
       
        
        Ok(())
    }

    fn presence(&self) -> Option<OperandsPresence> {
        Some(OperandsPresence::AllPresent)
    }
}

impl Coded<u8> for Flag {
    fn code(&self) -> u8 {
        match self {
            Self::JumpIfOverflow      => JUMP_IF_OVERFLOW_CODE,
            Self::JumpIfNotOverflow   => JUMP_IF_NOT_OVERFLOW_CODE,   
            Self::JumpIfRegrouping    => JUMP_IF_REGROUPING_CODE,     
            Self::JumpIfNotRegrouping => JUMP_IF_NOT_REGROUPING_CODE, 
            Self::JumpIfZero          => JUMP_IF_ZERO_CODE,            
            Self::JumpIfNotZero       => JUMP_IF_NOT_ZERO,             
            Self::JumpIfSign          => JUMP_IF_SIGN,                 
            Self::JumpIfNotSign       => JUMP_IF_NOT_SIGN             
        }
    }
}

impl Flag {
    pub fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            JUMP_IF_OVERFLOW_CODE       => Self::JumpIfOverflow,
            JUMP_IF_NOT_OVERFLOW_CODE   => Self::JumpIfNotOverflow,
            JUMP_IF_REGROUPING_CODE     => Self::JumpIfRegrouping,
            JUMP_IF_NOT_REGROUPING_CODE => Self::JumpIfNotRegrouping,
            JUMP_IF_ZERO_CODE           => Self::JumpIfZero,
            JUMP_IF_NOT_ZERO            => Self::JumpIfNotZero,
            JUMP_IF_SIGN                => Self::JumpIfSign,
            JUMP_IF_NOT_SIGN            => Self::JumpIfNotSign,
            _ => return None
        })
    }
}