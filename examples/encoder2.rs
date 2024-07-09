extern crate atln_processor;

use atln_processor::instruction::{Instruction, operand, operation, prefix::{Prefixes}, prefix};
use atln_processor::instruction::operand::register::Register;
use atln_processor::instruction::operand::{dynamic, register, SizedDual};
use atln_processor::instruction::operation::Operation;
use atln_processor::number;
use atln_processor::utility::{EncodeDynamic, FromCode};

fn main() {
    let instruction = Instruction {
        branch_likely_taken: None,
        execution_mode: None,
        operation: Operation::Floating(operation::floating::Floating::Subtract(SizedDual {
            data_size: number::Size::Quad,
            operand: operand::Dual {
                r#static: Register::General(register::General::A0),
                dynamic: operand::dynamic::Dynamic::Address(dynamic::Address::Constant(number::Number::Quad(10000000))),
                destination: operand::Type::Dynamic 
            }
        }))
    };

    let mut encoded = vec![0u8; 0];
    instruction.encode_dyn(&mut encoded);

    dbg!(encoded);
}