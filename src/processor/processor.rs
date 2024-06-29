use processor::processor::signal::Signal;
use crate::{memory::Memory};
use crate::processor::processor::instruction::Instruction;

pub mod array;
pub mod signal;
pub mod instruction;

pub struct Core {

}

/// The execution context.
pub struct Context {

}

impl Core {
    /// Execute an instruction and get any resource signals. Doing this could modify the 
    /// execution context.
    pub fn execute(_instruction: &Instruction, _memory: &mut Memory) -> Vec<Signal> {
        todo!();
    }
}
