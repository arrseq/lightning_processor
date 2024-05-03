pub mod instruction;
pub mod firmware;

pub struct MacroInstruction {
    pub operation: u8,
    pub register_a: u8,
    pub register_b: u8,
    pub immediate: u64
}

impl MacroInstruction {
    pub fn from() {
        // TODO:
        todo!();
    }
}