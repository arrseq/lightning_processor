use crate::instruction::Instruction;
use crate::instruction::operand::Operands;
use crate::instruction::operation::{Arithmetic, Operation};

// TODO: Implement surrounding tests.
#[test]
fn encode() {
    let instruction = Instruction {
        branch_likely_taken: None,
        execution: None,
        operation: Operation::Arithmetic(Arithmetic::Add),
        operands: Operands {
            destination: Nam
        }
    }
}

#[test]
fn decode() {
    
}