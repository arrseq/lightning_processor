use std::io;
use std::io::Read;
use crate::{absolute};
use crate::absolute::{BYTE_SIZE, DUAL_SIZE, QUAD_SIZE, WORD_SIZE};

// region: Constants
pub const REGISTER_ADDRESSING    : u8 = 0;
pub const OFFSET_ADDRESSING      : u8 = 1;
pub const CONSTANT_ADDRESSING    : u8 = 2;
pub const MEMORY_ADDRESSING      : u8 = 3;
pub const IMMEDIATE_EXPONENT_BYTE: u8 = 0;
pub const IMMEDIATE_EXPONENT_WORD: u8 = 1;
pub const IMMEDIATE_EXPONENT_DUAL: u8 = 2;
pub const IMMEDIATE_EXPONENT_QUAD: u8 = 3;
// endregion

// region: Single
/// A register code. This is static because this only serves as a register code operand and can only be used to 
/// dereference a register. Instruction executors never get access to this value directly, instead they get a 
/// register target.
pub type Static = u8;

/// Allows dereferencing a memory address by reading the value from a register then adding an offset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset {
	register: u8,
	offset: absolute::Data
}

/// Either a register code or immediate value addressing mode. Being dynamic means this gives the programmer freedom to 
/// pick either of the addressing modes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dynamic {
	/// Read value from register.
	Register(u8),
	/// Read value from register, add an offset to it, then use the sum to dereference memory.
	Offset(Offset),
	/// Read value from immediate as data.
	Constant(absolute::Data),
	/// Read value from memory address by addressing it with the immediate.
	Memory(absolute::Data)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadImmediateError {
	/// Caused by reading from the stream.
	Read,
	/// The stream does not contain enough bytes.
	Length,
	/// The exponent is larger than 3.
	Exponent
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromCodesError {
	/// The immediate exponent is out of bounds. 3 is the largest exponent for immediate.
	Immediate(ReadImmediateError),
	/// The addressing mode does not exist.
	Addressing
}

impl Dynamic {
	// fn read_stream<const buffer>()

	/// Read the immediate based on the exponent. The number of bytes read from the stream is based on using the
	/// immediate exponent as a power of 2.
	pub fn read_immediate(exponent: u8, stream: &mut impl Read) -> Result<absolute::Data, ReadImmediateError> {
		Ok(match exponent {
			IMMEDIATE_EXPONENT_BYTE => {
				let mut buffer = [0u8; BYTE_SIZE as usize];
				match stream.read(&mut buffer) {
					Ok(length) => if length != buffer.len() { return Err(ReadImmediateError::Length) },
					Err(_) => return Err(ReadImmediateError::Read)
				};

				absolute::Data::Byte(buffer[0])
			},
			IMMEDIATE_EXPONENT_WORD => {
				let mut buffer = [0u8; WORD_SIZE as usize];
				match stream.read(&mut buffer) {
					Ok(length) => if length != buffer.len() { return Err(ReadImmediateError::Length) },
					Err(_) => return Err(ReadImmediateError::Read)
				};

				absolute::Data::Word(u16::from_le_bytes(buffer))
			},
			IMMEDIATE_EXPONENT_DUAL => {
				let mut buffer = [0u8; DUAL_SIZE as usize];
				match stream.read(&mut buffer) {
					Ok(length) => if length != buffer.len() { return Err(ReadImmediateError::Length) },
					Err(_) => return Err(ReadImmediateError::Read)
				};

				absolute::Data::Dual(u32::from_le_bytes(buffer))
			},
			IMMEDIATE_EXPONENT_QUAD => {
				let mut buffer = [0u8; QUAD_SIZE as usize];
				match stream.read(&mut buffer) {
					Ok(length) => if length != buffer.len() { return Err(ReadImmediateError::Length) },
					Err(_) => return Err(ReadImmediateError::Read)
				};

				absolute::Data::Quad(u64::from_le_bytes(buffer))
			},
			_ => return Err(ReadImmediateError::Exponent)
		})
	}

	/// Create a new dynamic operand from codes. Not all the codes may be used. Returns [None] if the addressing code
	/// is invalid.
	///
	/// The immediate is expected to start where the immediate bytes would be. The immediate exponent is
	/// used to calculate how many immediate bytes should be read. These bytes will only be read if not in Register
	/// addressing mode.
	/// - The register is only used by the Register and Offset addressing modes.
	pub fn from_codes(register: u8, addressing: u8, immediate_exponent: u8, immediate_stream: &mut impl Read) -> 
																											  Result<Self, FromCodesError> {
		if addressing == REGISTER_ADDRESSING { return Ok(Self::Register(register)) }

		let immediate = match Self::read_immediate(immediate_exponent, immediate_stream) {
			Ok(immediate) => immediate,
			Err(error) => return Err(FromCodesError::Immediate(error))
		};

		Ok(match addressing {
			OFFSET_ADDRESSING => Self::Offset(Offset {
				register,
				offset: immediate,
			}),
			CONSTANT_ADDRESSING => Self::Constant(immediate),
			MEMORY_ADDRESSING => Self::Memory(immediate),
			_ => return Err(FromCodesError::Addressing)
		})
	}
	
	pub fn addressing(&self) -> u8 {
		match self {
			Self::Register(_) => REGISTER_ADDRESSING,
			Self::Offset(_) => OFFSET_ADDRESSING,
			Self::Constant(_) => CONSTANT_ADDRESSING,
			Self::Memory(_) => MEMORY_ADDRESSING
		}
	}
	
	pub fn immediate(&self) -> Option<&absolute::Data> {
		Some(match self {
			Self::Register(_) => return None,
			Self::Offset(offset) => &offset.offset,
			Self::Constant(constant) => constant,
			Self::Memory(memory) => memory
		})
	}
	
	/// Get the register code if the addressing includes one. Addressing modes [Register] and [Offset] support this 
	/// function and will return an instance of [Some] otherwise [None] will be returned.
	pub fn register(&self) -> Option<u8> {
		Some(match self {
			Self::Register(register) => *register,
			Self::Offset(offset) => offset.register,
			_ => return None
		})
	}
}

/// Operands provide the operation the arguments necessary for computing, There are 2 types of operands, static and 
/// dynamic operands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
	Static(Static),
	Dynamic(Dynamic)
}
// endregion

// region: Instruction ready operand parameter that contains addressing for a different modes of having operands.
/// All operands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllPresent {
	pub x_static: Static,
	pub x_dynamic: Dynamic
}

/// Multi configuration of operands for an instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operands {
	AllPresent(AllPresent),
	Static(Static),
	Dynamic(Dynamic)
}

impl Operands {
	/// Try to get the static operand.
	pub fn x_static(&self) -> Option<Static> {
		Some(match self {
			Self::Static(x_static) => *x_static,
			Self::AllPresent(x_all) => x_all.x_static,
			_ => return None
		})
	}

	/// Try to get the dynamic operand.
	pub fn x_dynamic(&self) -> Option<&Dynamic> {
		Some(match self {
			Self::Dynamic(x_dynamic) => x_dynamic,
			Self::AllPresent(x_all) => &x_all.x_dynamic,
			_ => return None
		})
	}
}
// endregion

#[cfg(test)]
mod dynamic_test {
	use std::io::Cursor;
	use crate::absolute;
	use crate::operand::{CONSTANT_ADDRESSING, Dynamic, IMMEDIATE_EXPONENT_BYTE, IMMEDIATE_EXPONENT_DUAL, 
						 IMMEDIATE_EXPONENT_QUAD, IMMEDIATE_EXPONENT_WORD, MEMORY_ADDRESSING, Offset, 
						 OFFSET_ADDRESSING, REGISTER_ADDRESSING};
	
	#[test]
	fn read_immediate() {
		let word = 0b11110000_11111111u16;
		let dual = 0b00001111_11111111_11110000_11001100u32;
		let quad = 0b00001111_11111111_11110000_11001100_00001111_11111111_11110000_11001100u64;
		
		assert!(matches!(Dynamic::read_immediate(IMMEDIATE_EXPONENT_BYTE, &mut Cursor::new([10])).unwrap(), 
				absolute::Data::Byte(10)));
		assert!(matches!(Dynamic::read_immediate(IMMEDIATE_EXPONENT_WORD, &mut Cursor::new(word.to_le_bytes()))
			.unwrap(), absolute::Data::Word(word)));
		assert!(matches!(Dynamic::read_immediate(IMMEDIATE_EXPONENT_DUAL, &mut Cursor::new(dual.to_le_bytes())).unwrap(), 
			absolute::Data::Dual(dual)));
		assert!(matches!(Dynamic::read_immediate(IMMEDIATE_EXPONENT_QUAD, &mut Cursor::new(quad.to_le_bytes())).unwrap(), 
			absolute::Data::Quad(quad)));
	}

	#[test]
	fn from_codes() {
		// Immediate is not used here.
		let register = Dynamic::from_codes(5, REGISTER_ADDRESSING, 0, &mut Cursor::new([])).unwrap();
		// Word sized immediate.
		let offset = Dynamic::from_codes(7, OFFSET_ADDRESSING, 1, &mut Cursor::new([0b00001111, 0b00111111])).unwrap();
		// Byte sized immediate.
		let constant = Dynamic::from_codes(0, CONSTANT_ADDRESSING, 0, &mut Cursor::new([0])).unwrap();
		// Quad sized immediate.
		let memory = Dynamic::from_codes(0, MEMORY_ADDRESSING, 2, &mut Cursor::new([0b00001111, 0b00111111,
			0b00001111, 0b00111111]))
			.unwrap();

		dbg!(memory.clone());

		assert!(matches!(register, Dynamic::Register(5)));
		assert!(matches!(offset, Dynamic::Offset(Offset {
			offset: absolute::Data::Word(0b00111111_00001111),
			register: 7
		})));
		assert!(matches!(constant, Dynamic::Constant(absolute::Data::Byte(0))));
		assert!(matches!(memory, Dynamic::Memory(absolute::Data::Dual(0b00111111_00001111_00111111_00001111))));
	}
}

#[cfg(test)]
mod operands_test {
	use crate::absolute;
	use crate::operand::{AllPresent, Dynamic, Operands};

	#[test]
	fn x_static() {
		let x_static = 5;

		let all = Operands::AllPresent(AllPresent {
		    x_static,
		    x_dynamic: Dynamic::Constant(absolute::Data::Byte(5))
		});

		let static_only = Operands::Static(x_static);
		let dynamic_only = Operands::Dynamic(Dynamic::Constant(absolute::Data::Byte(5)));

		assert_eq!(all.x_static().unwrap(), x_static);
		assert_eq!(static_only.x_static().unwrap(), x_static);
		assert!(dynamic_only.x_static().is_none());
	}

	#[test]
	fn x_dynamic() {
		let x_dynamic = Dynamic::Constant(absolute::Data::Byte(5));

		let all = Operands::AllPresent(AllPresent {
		    x_static: 10,
		    x_dynamic: x_dynamic.clone()
		});

		let static_only = Operands::Static(10);
		let dynamic_only = Operands::Dynamic(x_dynamic.clone());

		assert_eq!(*all.x_dynamic().unwrap(), x_dynamic);
		assert_eq!(*dynamic_only.x_dynamic().unwrap(), x_dynamic);
		assert!(static_only.x_dynamic().is_none());
	}
}