use std::io::Cursor;
use crate::instruction::operand::Operand;

#[test]
fn decode() {
    let mut cursor = Cursor::new([ 0b00_000000 ]);
    dbg!(Operand::decode(&mut cursor));
}