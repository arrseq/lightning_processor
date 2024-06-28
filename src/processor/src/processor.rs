use signal::Signal;

use crate::{instruction::Instruction, memory::Memory};

pub mod array;
pub mod signal;

pub struct Core {

}

/// The execution context.
pub struct Context {

}

impl Core {
    /// Execute an instruction and get any resource signals. Doing this could modify the 
    /// execution context.
    pub fn execute(instruction: &Instruction, memory: &mut Memory) -> Vec<Signal> {
        todo!();
    }
}