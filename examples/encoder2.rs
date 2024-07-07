extern crate atln_processor;

use atln_processor::{instruction::register::Register, processor::instruction::{Instruction, Operation}, utility::TryFromCode};
use atln_processor::processor::instruction::Prefix;
use atln_processor::utility::{Encode, MaxCode};

fn main() {
    let i = Instruction {
        prefixes: vec![Prefix::Synchronize, Prefix::Repeat],
        operation: Operation::Divide,
        static_operand: Register::try_from_code(10),
        dynamic_operand: None,
    };
    
    i.encode();
}