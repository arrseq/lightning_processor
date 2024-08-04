use crate::instruction::Instruction;
use crate::read_cursor;

#[test]
fn decode_branch_override() {
    dbg!(read_cursor([ 0x04, 0x03, 0b11_101_111, 0b111_111_00, 0x00 ], Instruction::decode));
}