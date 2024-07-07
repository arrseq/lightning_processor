extern crate atln_processor;

use atln_processor::instruction::{Instruction, operand, operation, prefix::{Prefix}, prefix};
use atln_processor::instruction::operand::register::Register;
use atln_processor::instruction::operation::Operation;
use atln_processor::utility::{Encode, FromCode};

fn main() {
    let instruction = Instruction {
        prefixes: Vec::from([ Prefix::Escape(operation::Size::Word), Prefix::Synchronize, Prefix::Repeat(prefix::Repeat::UntilEqual) ]),
        operation: Operation::Basic(operation::basic::Basic::Add(operand::Dual {
            r#static: Register::from_code(0),
            dynamic: operand::dynamic::Dynamic::Register(Register::from_code(1)),
            destination: operand::Type::Static
        }))
    };
    
    instruction.encode();
}