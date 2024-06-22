#![feature(let_chains)]
#![forbid(clippy::result_unit_err)]
#![forbid(clippy::question_mark)]

use crate::instruction::{InstructionConstructError, Instruction};
use crate::memory::{Memory};

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
    pub execution_context: ExecutionContext,
    /// The current decoded instruction.
    pub instruction: Instruction
}

impl Core {
    /// Decode an instruction from memory based on the program counter.
    pub fn decode(&mut self, _memory: &mut Memory) -> Result<(), InstructionConstructError> {
        todo!()
        // self.instruction = Instruction::new(memory);
    }
}