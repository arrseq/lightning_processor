use std::io::Cursor;
use crate::instruction::operand::Operand;

#[test]
fn decode() {
    let mut cursor = Cursor::new([ 0b10_110000, 0b00000001 ]);
    dbg!(Operand::decode(&mut cursor));
}