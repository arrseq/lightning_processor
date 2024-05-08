extern crate core;

use std::io::Cursor;
use rhdl_bits::Bits;
use exr_p::instruction::dynamic::{Addressing, Dynamic, Register};
use exr_p::instruction::{absolute, coder, Instruction};
use exr_p::instruction::dynamic::Addressing::Address;
use exr_p::instruction::operand::{Destination, Full, Storage, Operands};
use exr_p::instruction::operation::{Classification, Memory, Numerical};

fn main() {
	// add r0, r1
	let add_r2r  = Instruction {
		operation: Classification::Magnitude(Numerical::Add),
		operands: Operands {
			destination: Destination::First,
			storage: Storage::Full(Full {
				first: Bits::from(0),
				second: Dynamic {
					value: Bits::from(0),
					addressing: Addressing::Register(Register::Direct)
				}
			})
		}
	};

	// add [r0+10], r1 // Add r1 to the data at the memory address dereferenced by r0.
	let add_r2m = Instruction {
		operation: Classification::Magnitude(Numerical::Add),
		operands: Operands {
			destination: Destination::Second, // While the assembly has it visibly as "First" it's an illusion. Only
			// the second operand can be dereferenced.
			storage: Storage::Full(Full {
				first: Bits::from(0),
				second: Dynamic {
					value: Bits::from(0),
					addressing: Addressing::DereferenceOffset(absolute::Data::Byte(10))
				}
			})
		}
	};

	// clo r0, #FF
	let mut clo_c2r = Instruction {
		operation: Classification::Memory(Memory::Clone),
		operands: Operands {
			destination: Destination::First,
			storage: Storage::Full(Full {
				first: Bits::from(0),
				second: Dynamic {
					value: Bits::from(0),
					addressing: Addressing::DereferenceOffset(absolute::Data::Byte(10))
				}
			})
		}
	};
	
	let mut stream = Cursor::new([
		// Collection, destination
		0b0000001_1,

		// Operation, mode, addressing
		0b0001_00_00,

		// Exponent, register A, register b
		0b000_00_000,

		// Immediate
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
	]);

	coder::decoder::decode(&mut stream, &mut clo_c2r).expect("Failed");
}