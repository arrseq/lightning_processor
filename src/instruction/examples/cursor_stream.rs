use std::io::Cursor;
use em_instruction::{Instruction, operand};
use em_instruction::operand::{Destination, Operands};
use em_instruction::operation::{Classification, Logical};

fn main() {
	let mut target_ins = Instruction {
		operation: Classification::Logical(Logical::And),
		operands: Operands {
			storage: operand::Mode::None.into(),
			destination: Destination::First
		}
	};

	let c_stream = Cursor::new([

	]);
}