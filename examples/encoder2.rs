extern crate atln_processor;

use atln_processor::{instruction::register::Register, processor::instruction::{Instruction, Operation}, utility::TryFromCode};
use atln_processor::utility::{Encode, MaxCode, Partitioned};

fn main() {
    let i = Instruction {
        prefixes: vec![],
        operation: Operation::Divide,
        static_operand: Register::try_from_code(10),
        dynamic_operand: None,
    };
    
    dbg!(i.encode());
}