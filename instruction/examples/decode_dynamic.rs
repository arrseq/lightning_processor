extern crate arrseq_instruction;
extern crate arrseq_memory;

use std::io::Cursor;
use arrseq_instruction::operand::Operands;

fn main() {
    let mut cursor = Cursor::new([
        0b00_1_0001_1,
        0b1010_0000,
        100
    ]);

    let decoded = Operands::decode(&mut cursor);
    dbg!(decoded);
}