extern crate arrseq_lightning;

use std::io::Cursor;
use arrseq_lightning::core::Core;
use arrseq_lightning::dynamic_number::Size;
use arrseq_lightning::instruction::Instruction;
use arrseq_lightning::instruction::operand::{Name, Operands};
use arrseq_lightning::instruction::operand::dynamic::Dynamic;
use arrseq_lightning::instruction::operand::register::{GeneralPurpose, Register};
use arrseq_lightning::instruction::operation::{Arithmetic, Operation};

fn main() {
	let core = Core::default();
	let mut memory: Cursor<Vec<u8>> = Cursor::new(Vec::new());
	
	let add = Instruction {
		branch_likely_taken: None,
		execution: None,
		operation: Operation::Arithmetic(Arithmetic::Add),
		operands: Operands {
			result: Name::Register,
			register: Register::Accumulator,
			dynamic: Dynamic::Register(Register::GeneralPurpose(GeneralPurpose::G0)),
			size: Size::QuadWord,
			custom_data: false
		}
	};
	
	add.
}