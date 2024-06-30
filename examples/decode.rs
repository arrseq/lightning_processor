extern crate atln_processor;

use std::io::Cursor;
use atln_processor::emulator::processor::processor::instruction::Instruction;

fn main() {
    let mut cursor = Cursor::new([ 0b000000_0_0, 0b0000_10_01, 0b00_000_000, 0b11111111, 0b00000000 ]);
    let instruction = Instruction::new(&mut cursor).unwrap();
    
    dbg!(instruction);
}