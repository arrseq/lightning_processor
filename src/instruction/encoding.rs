use crate::instruction::Instruction;

pub const OPERATION_MASK: u32 = 0x0000007F;

impl Instruction {
    pub fn decode(encoded: u32) {
        let operation = encoded & OPERATION_MASK;
        dbg!(operation);
    }
}