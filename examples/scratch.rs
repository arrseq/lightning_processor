use rhdl_bits::Bits;
use exr_p::instruction::dynamic::{Addressing, Dynamic, Register};
use exr_p::instruction::{absolute, Instruction};
use exr_p::instruction::operand::{Destination, Full, Storage, Operands};
use exr_p::instruction::operation::{Classification, Numerical};

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
	
	// add [r0], r1 // Add r1 to the data at the memory address dereferenced by r0.
	let add_r2m = Instruction {
		operation: Classification::Magnitude(Numerical::Add),
		operands: Operands {
			destination: Destination::Second, // While the assembly has it visibly as "First" it's an illusion. Only 
			// the second operand can be dereferenced.
			storage: Storage::Full(Full {
				first: Bits::from(0),
				second: Dynamic {
					value: Bits::from(0),
					addressing: Addressing::RegisterOffset(absolute::Data::Byte(0))
				}
			})
		}
	};
}