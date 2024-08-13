pub mod arithmetic;

use crate::instruction::RegisterCode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State {
    pub instruction_pointer: u64,
    // TODO: Operand modifiers state
    pub general: [u64; RegisterCode::MASK as usize],
    // pub general_vectors: []
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Core {
    /// The emulation context
    pub state: State
}