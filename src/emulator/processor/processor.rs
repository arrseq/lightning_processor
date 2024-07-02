use emulator::memory::Memory;
use super::processor::instruction::{Instruction, operation::Operation};

pub mod array;
pub mod instruction;

/// Ports list for input and output.
pub type Ports = [u8; 8];

/// Registers array.
pub type Registers = [u64; 8];

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Core {
    pub context: Context,
}

/// The execution context of an individual core.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Context {
    pub registers: Registers,
    /// Whether virtual memory address translation is enabled.
    pub virtual_mode: bool
}

impl Core {
    /// Execute an instruction and see if the processor must halt. Doing this could modify the execution context.
    pub fn execute(&mut self, instruction: &Instruction, memory: &mut Memory, ports: &mut Ports) -> bool {
        instruction.extension().operation().execute(instruction.data().as_ref(), memory, &mut self.context, ports).expect("TODO: panic message");
        false
    }
}
