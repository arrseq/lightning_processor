//! # Declarations
//! - References to parts of a binary number will be in big endian. Saying left most bits would refer to the most 
//!   significant bits.
//! - The architecture this library implements is for the xT3 processor. xT3 also reefers to the architecture of the
//!   processor.
//! 
//! # Terms
//! - Real mode is a state of the processor that allows addressing to be as is rather than being translated. An 
//!   operating system kernel would run in real mode.

#![forbid(clippy::result_unit_err)]
#![forbid(clippy::question_mark)]
#![allow(clippy::module_inception)]

use crate::processor::processor::instruction::{InstructionConstructError, Instruction};
use crate::memory::{Memory};

pub mod graphics;
pub mod memory;
pub mod number;
pub mod processor;
pub mod utility;

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