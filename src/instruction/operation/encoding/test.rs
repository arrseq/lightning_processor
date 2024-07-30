use crate::cursor_test;
use crate::instruction::operand::{AddressingMode, Operand};
use crate::instruction::operation::{Input, Operation};
use crate::math::dynamic_number::Size;

#[test]
fn decode() {
    // stack register 0 as byte
    assert_eq!(cursor_test([0, 0], Operation::decode).unwrap(), Operation::Input {
        operation: Input::Stack,
        input: Operand {
            mode: AddressingMode::Register { register: 0 },
            size: Size::U8
        }
    });
}