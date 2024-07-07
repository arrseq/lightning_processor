use instruction::operand;

pub enum Basic {
    Add(operand::Dual),
    CarryingAdd(operand::Dual),
    Subtract(operand::Dual),
    BorrowingSubtract(operand::Dual),
    
    /// Copy the value between the operands corresponding to the result destination of the data.
    Copy(operand::Dual),
    /// Append and item from the operand onto the stack.
    AppendStack(operand::dynamic::Dynamic),
    /// Remove an item from the stack and store it in the operand.
    DetachStack(operand::dynamic::Dynamic),
    
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
