extern crate arrseq_lightning;

use arrseq_lightning::instruction::{address, Instruction, operation, RegisterCode};
use arrseq_lightning::instruction::address::{Address};
use arrseq_lightning::instruction::flag::Flag;
use arrseq_lightning::num::MaskedU8;

fn main() {
    let c_max = 10u16;

    let program = [
        Instruction::LoadImmediate { destination: RegisterCode::new(0), segment: MaskedU8::new(0), immediate: c_max },                                              // li r0, c_max
        Instruction::DualSource { operation: operation::DualSource::Compare, sources: [RegisterCode::new(0), RegisterCode::new(1)] },                               // cmp r0, r1
        Instruction::Branch { hint: None, condition: Flag::Zero, address: Address::Immediate { immediate: address::Immediate::new(8), mode: address::Mode::Relative }},    // jz pc+8
        Instruction::Memory { operation: operation::Memory::Branch, address: Address::Immediate { immediate: address::Immediate::new(4), mode: address::Mode::Absolute }}, // jmp 0
        Instruction::WaitForInterrupt                                                                                                                                             // hlt

        // pseudo code:
        //
        // ```
        // let count = c_max;
        // loop {
        //     count -= 1;
        //     if count == 0 { return 0 }
        // }
        // ```
    ];

    dbg!(Instruction::decode(0b1000_1000_1000_1000_0001_1000_0000010));
}