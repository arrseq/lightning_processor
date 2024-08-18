use crate::instruction::register;

pub mod arithmetic;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State {
    pub instruction_pointer: u64,
    // TODO: Operand modifiers state
    pub general: [u64; register::Code::MASK as usize],
    // pub general_vectors: []
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Core {
    /// The emulation context
    pub state: State
}