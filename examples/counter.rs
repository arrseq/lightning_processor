extern crate arrseq_lightning;

use arrseq_lightning::instruction::Instruction;

fn main() {
    let ins = Instruction::decode(0b00000000_00000000_00000000_00000100);
    dbg!(ins);
}