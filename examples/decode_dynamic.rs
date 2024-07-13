extern crate arrseq_instruction;

use std::io::Cursor;
use arrseq_instruction::instruction::Instruction;

fn main() {
    let mut cursor = Cursor::new([
        0x00,
        0x00,
        0b00_1_1110_1,
        0b1110_0110,
        100,
        100,
        100,
        100,
        100,
        100,
        100,
        100
    ]);

    let instruction = Instruction::decode(&mut cursor)
        .unwrap();
    dbg!(instruction);
}