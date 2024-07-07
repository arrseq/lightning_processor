use instruction::operand;
use utility::ToCode;

#[repr(u16)]
pub enum Code {
    Add,
    CarryingAdd,
    Subtract,
    BorrowingSubtract,

    Copy,

    AppendStack,
    AppendStackRegisters,
    DetachStack,
    DetachStackRegisters,

    LogicalAnd,
    LogicalOr,
    LogicalNot,
    LogicalXor,

    Increment,
    Decrement,

    JumpIfZero,
    JumpIfOverflow,
    JumpIfRegrouping,
    JumpIfNegative
}

#[derive(Debug)]
pub enum Basic {
    Add(operand::Dual),
    CarryingAdd(operand::Dual),
    Subtract(operand::Dual),
    BorrowingSubtract(operand::Dual),
    
    /// Copy the value between the operands corresponding to the result destination of the data.
    Copy(operand::Dual),
    
    /// Append and item from the operand onto the stack.
    AppendStack(operand::dynamic::Dynamic),
    /// Append all public registers to the stack. The stack pointer register will be the one before calling this 
    /// operation.
    AppendStackRegisters,
    /// Remove an item from the stack and store it in the operand.
    DetachStack(operand::dynamic::Dynamic),
    /// Copy the public registers from the stack and back into the public registers while removing them from the stack. 
    /// The stack pointer is not loaded back into the stack pointer register and only removed from the stack.
    DetachStackRegisters,
    
    LogicalAnd(operand::dynamic::Dynamic),
    LogicalOr(operand::dynamic::Dynamic),
    LogicalNot(operand::dynamic::Dynamic),
    LogicalXor(operand::dynamic::Dynamic),
    
    Increment(operand::dynamic::Dynamic),
    Decrement(operand::dynamic::Dynamic),
    
    JumpIfZero(operand::dynamic::Dynamic),
    JumpIfOverflow(operand::dynamic::Dynamic),
    JumpIfRegrouping(operand::dynamic::Dynamic),
    JumpIfNegative(operand::dynamic::Dynamic)
}

impl ToCode for Basic {
    type Code = u16;

    fn to_code(&self) -> Self::Code {
        (match self {
            Self::Add(_) => Code::Add,
            Basic::CarryingAdd(_) => Code::CarryingAdd,
            Basic::Subtract(_) => Code::Subtract,
            Basic::BorrowingSubtract(_) => Code::BorrowingSubtract,
            Basic::Copy(_) => Code::Copy,
            Basic::AppendStack(_) => Code::AppendStack,
            Basic::AppendStackRegisters => Code::AppendStackRegisters,
            Basic::DetachStack(_) => Code::DetachStack,
            Basic::DetachStackRegisters => Code::DetachStackRegisters,
            Basic::LogicalAnd(_) => Code::LogicalAnd,
            Basic::LogicalOr(_) => Code::LogicalOr,
            Basic::LogicalNot(_) => Code::LogicalNot,
            Basic::LogicalXor(_) => Code::LogicalXor,
            Basic::Increment(_) => Code::Increment,
            Basic::Decrement(_) => Code::Decrement,
            Basic::JumpIfZero(_) => Code::JumpIfZero,
            Basic::JumpIfOverflow(_) => Code::JumpIfOverflow,
            Basic::JumpIfRegrouping(_) => Code::JumpIfRegrouping,
            Basic::JumpIfNegative(_) => Code::JumpIfNegative
        }) as Self::Code
    }
}