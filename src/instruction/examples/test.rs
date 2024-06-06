use std::io::Cursor;
use em_instruction::{absolute, Data, Instruction};
use em_instruction::operand::{AllPresent, Dynamic, Operands};

fn print_bytes(b: [u8; 2]) {
	for x in b { print!("{:08b}, ", x) }
	println!();
}

fn main() {
	let mut rom = Cursor::new([0, 0, 0, 0, 0, 0]); // Everything as 0 is a syntactically correct instruction.

	let mut instruction;

	instruction = Instruction::from_encoded(&mut rom)
		.expect("Could not decode");

	if let Some(data) = instruction.data {
		instruction.data = Some(Data {
			width: data.width,
			operands: Operands::AllPresent(AllPresent {
				x_static: 3,
				x_dynamic: Dynamic::Constant(absolute::Data::Quad(2))
			}),
			destination: data.destination,
			synchronise: true
		});
	}

	println!("{:?}", instruction);

	let encoded = instruction.encode();
	println!("{:?}", encoded);
}