#![feature(let_chains)]

use std::io::Read;
use crate::instruction::{InstructionConstructError, Instruction, operation::Operation};
use crate::instruction::operation::{Coded, OperationExecuteError};
use crate::memory::{Frame, Memory};
use crate::number::Size;

pub mod number;
pub mod instruction;
pub mod memory;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExecutionContext {
    pub program_counter: u64,
    pub stack_pointer: u64,
    pub base_pointer: u64,
    pub accumulator: u64,
    pub general_purpose: [u64; 4]
}

#[derive(Debug, Default)]
pub struct Core {
    execution_context: ExecutionContext,
    /// The current decoded instruction.
    instruction: Instruction
}

impl Core {
    /// Decode an instruction from memory based on the program counter.
    pub fn decode(&mut self, memory: &mut Memory) -> Result<(), InstructionConstructError> {
        todo!()
        // self.instruction = Instruction::new(memory);
    }
}