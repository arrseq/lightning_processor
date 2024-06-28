use crate::{instruction::Instruction, memory::Memory};

/// Used when something prevented the execution of something.
pub enum Signal {
    /// The instruction must be syncronized.
    Synchronise
}

pub struct Core {

}

impl Core {
    /// Execute an instruction.
    pub fn execute(instruction: &Instruction, memory: &mut Memory) -> Result<> {
        todo!();
    }
}