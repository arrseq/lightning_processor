use std::io::Read;

use crate::binary;

use super::MacroOperation;

pub struct ClassARegisterOperand {
    destination: u8,
    first: u8,
    second: u8
}

pub struct ClassBRegisterOperand {
    first: u8,
    second: u8
}

pub struct ClassCRegisterOperand {
    first: u8
}

pub struct ClassDRegisterOperand {
    destination: u8
}

pub struct ClassERegisterOperand {
    destination: u8,
    first: u8
}

pub fn decode_class_a_register_operand<Stream: Read>(stream: &mut Stream) -> Option<ClassARegisterOperand> {
    let destination = match binary::read_byte(stream) {
        None => return None,
        Some(destination) => destination
    };

    let first = match binary::read_byte(stream) {
        None => return None,
        Some(first) => first
    };

    let second = match binary::read_byte(stream) {
        None => return None,
        Some(second) => second
    };

    Some(ClassARegisterOperand { destination, first, second })
}

pub fn decode_class_b_register_operand<Stream: Read>(stream: &mut Stream) -> Option<ClassBRegisterOperand> {
    let first = match binary::read_byte(stream) {
        None => return None,
        Some(first) => first
    };

    let second = match binary::read_byte(stream) {
        None => return None,
        Some(second) => second
    };

    Some(ClassBRegisterOperand { first, second })
}

pub fn decode_class_c_register_operand<Stream: Read>(stream: &mut Stream) -> Option<ClassCRegisterOperand> {
    let first = match binary::read_byte(stream) {
        None => return None,
        Some(first) => first
    };

    Some(ClassCRegisterOperand { first })
}

pub fn decode_class_d_register_operand<Stream: Read>(stream: &mut Stream) -> Option<ClassDRegisterOperand> {
    let destination = match binary::read_byte(stream) {
        None => return None,
        Some(destination) => destination
    };

    Some(ClassDRegisterOperand { destination })
}

pub fn decode_class_e_register_operand<Stream: Read>(stream: &mut Stream) -> Option<ClassERegisterOperand> {
    let destination = match binary::read_byte(stream) {
        None => return None,
        Some(destination) => destination
    };

    let first = match binary::read_byte(stream) {
        None => return None,
        Some(first) => first
    };

    Some(ClassERegisterOperand { destination, first })
}

/// Decode the macro instruction.
pub fn decode_macro<Stream: Read>(stream: &mut Stream) -> Option<MacroOperation> {
    let operation_byte = match binary::read_byte(stream) {
        None => return None,
        Some(byte) => byte
    };

    match operation_byte {
        0 => return Some(MacroOperation::Nothing),
        1 => return Some(MacroOperation::Terminate),
        2 => {
            let decoded = match decode_class_c_register_operand(stream) {
                None => return None,
                Some(some) => some 
            };

            return Some(MacroOperation::Interrupt { code: decoded.first })
        },
        3 => {
            let decoded = match decode_class_c_register_operand(stream) {
                None => return None,
                Some(some) => some 
            };

            return Some(MacroOperation::Safe { divert_location: decoded.first })
        }
        _ => return None
    }
} 

/// Decode macro instructions into their micro instructions.
pub fn decode_micro() {
    todo!();
}