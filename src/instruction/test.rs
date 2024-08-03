use crate::instruction::Instruction;
use crate::read_cursor;

#[test]
fn decode_branch_override() {
    dbg!(read_cursor([ 0x04, 0x03, 0, 0, 0x00 ], Instruction::decode));
}