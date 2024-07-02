use emulator::memory::Memory;
use super::processor::instruction::Instruction;

pub mod array;
pub mod instruction;

/// Ports list for input and output.
pub type Ports = [u8; 8];

pub struct Core {

}

/// The execution context.
pub struct Context {

}

impl Core {
    /// Execute an instruction and see if the processor must halt. Doing this could modify the execution context.
    pub fn execute(_instruction: &Instruction, _memory: &mut Memory, ports: &mut Ports) -> bool {
        todo!();
    }
}
