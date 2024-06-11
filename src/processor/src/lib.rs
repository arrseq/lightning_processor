#![feature(let_chains)]

use std::io::Read;
use crate::instruction::{DecodeError, Instruction, operation::Operation};
use crate::instruction::operation::{Coded, OperationExecuteError};
use crate::memory::Memory;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Core {
    execution_context: ExecutionContext
}

#[derive(Debug)]
pub enum ExecuteError {
    /// The instruction could not be decoded from the stream.
    Decode(DecodeError),
    Execute(OperationExecuteError)
}

impl Core {
    pub fn new() -> Self {
        Self {
            execution_context: ExecutionContext::default()
        }
    }

    /// Execute instructions from a stream
    pub fn execute(&mut self, stream: &mut impl Read, memory: &mut Memory) -> Result<(), ExecuteError> {
        let mut instruction = match Instruction::from_encoded(stream) {
            Ok(value) => value,
            Err(error) => return Err(ExecuteError::Decode(error))
        };

        let code = instruction.extension.code();
        if let Err(error) = instruction.extension.operation().execute(code, instruction.data.as_ref(), &mut
            self.execution_context) { return Err(ExecuteError::Execute(error)) }

        // memory.at(1, number::Type::Byte).unwrap();

        Ok(())
    }
}                                                               