use std::io::Cursor;
use em_instruction::{Instruction, operand};
use em_instruction::coder::decoder::decode;
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

	let mut c_stream = Cursor::new([
		0x00
	]);
	
	match decode(&mut c_stream, &mut target_ins) {
		Err(error) => panic!("Failed to decode instruction: {:?}", error),
		Ok(_) => println!("Ok")
	}
}