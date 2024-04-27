use super::{interrupt::Interrupt, ClassARegisterOperand, ClassBRegisterOperand, ClassCRegisterOperand, ClassDRegisterOperand, ClassERegisterOperand, MacroOperation};

pub fn encode_class_a_register_operand(operands: ClassARegisterOperand) -> [u8; 3] {
    [operands.destination, operands.first, operands.second]
}

pub fn encode_class_b_register_operand(operands: ClassBRegisterOperand) -> [u8; 2] {
    [operands.first, operands.second]
}

pub fn encode_class_c_register_operand(operands: ClassCRegisterOperand) -> [u8; 1] {
    [operands.first]
}

pub fn encode_class_d_register_operand(operands: ClassDRegisterOperand) -> [u8; 1] {
    [operands.destination]
}

pub fn encode_class_e_register_operand(operands: ClassERegisterOperand) -> [u8; 2] {
    [operands.destination, operands.first]
}

/// A block is a memory region that can be assessed as a routine in the
/// program.
pub struct Block {
    pub instructions: Vec<MacroOperation>,
    pub interrupt: Option<Interrupt>
}