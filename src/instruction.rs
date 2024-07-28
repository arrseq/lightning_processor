use crate::instruction::operation::Operation;

pub mod operation;
pub mod operand;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub operation: Operation
}