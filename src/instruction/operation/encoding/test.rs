use crate::cursor_test;
use crate::instruction::operation::Operation;

#[test]
fn decode() {
    cursor_test([255, 255], Operation::decode).expect("Failed to decode operation");
}