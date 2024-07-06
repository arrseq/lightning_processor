extern crate atln_processor;

use atln_processor::{instruction::register::Register, processor::instruction::{Instruction, Operation}, utility::TryFromCode};

fn main() {
    let i = Instruction {
        operation: Operation::Divide,
        static_operand: Register::try_from_code(10).unwrap()
    };
}