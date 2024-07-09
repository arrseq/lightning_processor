extern crate atln_processor;

use std::io::Cursor;
use atln_processor::instruction::{Instruction, operand, operation};
use atln_processor::instruction::operand::{register, SizedDual};
use atln_processor::instruction::operand::register::Register;
use atln_processor::instruction::operation::Operation;
use atln_processor::number;
use atln_processor::number::Number;

fn main() {
    let instruction = Instruction {
        branch_likely_taken: Some(true),
        execution_mode: None,
        operation: Operation::Basic(operation::basic::Basic::Copy(SizedDual {
            data_size: number::Size::Quad,
            operand: operand::Dual {
                r#static: Register::General(register::General::F1),
                dynamic: operand::dynamic::Dynamic::Constant(Number::Word(u16::MAX)),
                destination: operand::Type::Dynamic 
            }
        }))
    };

    let mut encoded = vec![0u8; 0];
    instruction.encode(&mut encoded);

    let decoded = Instruction::decode(&mut Cursor::new(encoded));
}