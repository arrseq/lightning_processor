use std::io::Write;

use super::ClassARegisterOperand;

pub fn encode_class_a_register_operand<Stream: Write>(stream: &mut Stream, operands: ClassARegisterOperand) -> Result<(), ()> {
    
}