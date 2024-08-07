extern crate arrseq_lightning;

use arrseq_lightning::instruction::{Instruction, memory};
use arrseq_lightning::instruction::register::Register;
use arrseq_lightning::num::Size;

fn main() {
    let program = [
        Instruction::Memory {
            operation: memory::Operation::AddressedCopy,
            size: Size::X8,
            register: Register::new(0),
            address: memory::Address::Immediate { 
                mode: memory::Mode::Absolute,
                immediate: 0
            }
        }
    ];
}