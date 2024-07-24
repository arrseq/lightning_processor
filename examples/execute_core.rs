extern crate arrseq_lightning;

use std::io::Cursor;
use arrseq_lightning::core::Core;
use arrseq_lightning::math::dynamic_number::Size;
use arrseq_lightning::instruction::Instruction;
use arrseq_lightning::instruction::operand::{Destination, Operands};
use arrseq_lightning::instruction::operand::dynamic::Operand;
use arrseq_lightning::instruction::operand::register::{GeneralPurpose, Register};
use arrseq_lightning::instruction::operation::{Arithmetic, Operation};

fn main() {
	let mut core = Core::default();
	let mut memory: Cursor<Vec<u8>> = Cursor::new(Vec::new());

	let add = Instruction {
		branch_likely_taken: None,
		execution: None,
		operation: Operation::Arithmetic(Arithmetic::Add),
		operands: Operands {
			destination: Destination::Register,
			register: Register::Accumulator,
			dynamic: Operand::Register(Register::GeneralPurpose(GeneralPurpose::G0)),
			size: Size::QuadWord,
			external_destination: false
		}
	};

	add.encode(&mut memory).unwrap();
	
	let instruction = core.decode(&mut memory).expect("[core] failed to decode.");
	assert_eq!(add, instruction);
}