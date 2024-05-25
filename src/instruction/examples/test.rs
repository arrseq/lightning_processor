use em_instruction::{Destination, Instruction};
use em_instruction::absolute::Data;
use em_instruction::operand::{Dynamic, Static};
use em_instruction::operation::arithmetic::Arithmetic;
use em_instruction::operation::Extension;

fn main() {
	// add br0, 5 ; Store 5 in byte register 0
	let operation = Instruction {
		operation: Extension::Arithmetic(Arithmetic::Add),
		destination: Destination::Static, // Store value in r0
		x_static: Static(Some(0)), // r0 target
		x_dynamic: Dynamic::Constant(Data::Byte(5))
	};
}