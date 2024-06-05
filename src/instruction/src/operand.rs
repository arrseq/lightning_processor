use crate::{absolute};

// region: Constants
pub const REGISTER_ADDRESSING: u8 = 0;
pub const OFFSET_ADDRESSING  : u8 = 1;
pub const CONSTANT_ADDRESSING: u8 = 2;
pub const MEMORY_ADDRESSING  : u8 = 3;
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
pub enum FromCodesError {
	/// The addressing code is invalid.
	InvalidAddressing
}

impl Dynamic {
	/// Create a new dynamic operand from codes. Not all the codes may be used. 
	/// - The immediate is only used by the Offset, Constant, and Memory addressing modes.
	/// - The register is only used by the Register and Offset addressing modes.
	pub fn from_codes(register: u8, addressing: u8, immediate: absolute::Data) -> Result<Self,
		FromCodesError> {
		Ok(match addressing {
			REGISTER_ADDRESSING => Self::Register(register),
			OFFSET_ADDRESSING => Self::Offset(Offset {
				register,
				offset: immediate,
			}),
			CONSTANT_ADDRESSING => Self::Constant(immediate),
			MEMORY_ADDRESSING => Self::Memory(immediate),
			_ => return Err(FromCodesError::InvalidAddressing)
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
	use crate::absolute;
	use crate::operand::{CONSTANT_ADDRESSING, Dynamic, MEMORY_ADDRESSING, Offset, OFFSET_ADDRESSING, REGISTER_ADDRESSING};

	#[test]
	fn from_codes() {
		// Exhaustive testing.
		let register = Dynamic::from_codes(5, REGISTER_ADDRESSING, absolute::Data::Byte(0)).unwrap();
		let offset = Dynamic::from_codes(7, OFFSET_ADDRESSING, absolute::Data::Byte(10)).unwrap();
		let constant = Dynamic::from_codes(0, CONSTANT_ADDRESSING, absolute::Data::Quad(u64::MAX)).unwrap();
		let memory = Dynamic::from_codes(0, MEMORY_ADDRESSING, absolute::Data::Quad(1)).unwrap();

		assert!(matches!(register, Dynamic::Register(5)));
		assert!(matches!(offset, Dynamic::Offset(Offset {
			offset: absolute::Data::Byte(10),
			register: 7
		})));
		assert!(matches!(constant, Dynamic::Constant(absolute::Data::Quad(u64::MAX))));
		assert!(matches!(memory, Dynamic::Memory(absolute::Data::Quad(1))));
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