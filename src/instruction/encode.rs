use super::{interrupt::Interrupt, ClassARegisterOperand, ClassBRegisterOperand, MacroOperation};

pub fn encode_class_a_register_operand(operands: ClassARegisterOperand) -> [u8; 2] {
    [operands.first, operands.second]
}

pub fn encode_class_b_register_operand(operands: ClassBRegisterOperand) -> [u8; 1] {
    [operands.first]
}

/// A block is a memory region that can be assessed as a routine in the
/// program.
pub struct Block {
    pub instructions: Vec<MacroOperation>,
    pub interrupt: Option<Interrupt>
}