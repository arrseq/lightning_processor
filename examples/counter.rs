extern crate arrseq_lightning;

use arrseq_lightning::instruction::Instruction;
use arrseq_lightning::instruction::memory::LockOperation;

fn main() {
    let ins = Instruction::decode(0b00000000_00000000_00000000_00000100);
    dbg!(ins);
    
    let i = Instruction::Lock(LockOperation::from(0)
        .with_relative(false)
        .with_base(10));
    
    dbg!(i.encode());
}