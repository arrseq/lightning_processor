extern crate atln_processor;

use atln_processor::instruction::{Instruction, operand, operation, prefix::{Prefixes}, prefix};
use atln_processor::instruction::operand::register::Register;
use atln_processor::instruction::operation::Operation;
use atln_processor::utility::{EncodeDynamic, FromCode};

fn main() {
    let instruction = Instruction {
        prefixes: Prefixes {
            escape: Some(operation::Size::Word),
            extension: None,
            branch_likely_taken: None,
            execution_mode: Some(prefix::ExecutionMode::Synchronize)
        },
        operation: Operation::Basic(operation::basic::Basic::Add(operand::Dual {
            r#static: Register::from_code(0),
            dynamic: operand::dynamic::Dynamic::Register(Register::from_code(1)),
            destination: operand::Type::Static
        }))
    };
    
    let mut encoded = vec![0u8; 0];
    instruction.encode_dyn(&mut encoded);
    
    dbg!(encoded);
}