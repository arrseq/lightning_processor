use std::io::Cursor;
use em_instruction::Instruction;

fn print_bytes(b: [u8; 2]) {
	for x in b { print!("{:08b}, ", x) }
	println!();
}

fn main() {
	// let mut rom = Cursor::new([0, 0, 0, 0, 0, 0]); // Everything as 0 is a syntactically correct instruction.
	// 
	// let mut instruction;
	// 
	// loop {
	// 	instruction = Instruction::from_encoded(&mut rom)
	// 		.expect("Could not decode");
	// 
	// 	println!("{:?}", instruction);
	// }
	
	let num: u16 = 0b11110000_10101010;
	let nub = u16::from_le_bytes([0b10101010, 0b11110000]);
	
	println!("num: {:016b}", num);
	println!("nub: {:016b}", nub);
	
	let num_bytes = u16::to_be_bytes(num);
	print_bytes(num_bytes);
}