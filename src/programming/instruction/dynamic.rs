//! Dynamic parameter.
//! This is used to allow accepting of differently addressed data.
//! - Used for custom addressing.
//! - Supports architecture sized data through an immediate.

use std::intrinsics::variant_count;
use crate::programming::instruction::absolute;

pub const MAX_MODE_VARIANTS: u8 = 2^2;
pub const MAX_METHOD_VARIANTS: u8 = 2^2;

/// If the addressing mode is invalid or does not exist.
pub struct ModeError {}

/// Register addressing mode.
/// Mode in which a register's value is pre-processed.
#[derive(Debug, Default)]
pub enum Register {
	/// Direct read.
	/// Find the register value and use it directly.
	#[default]
	Direct,
	/// Dereference register value as memory address.
	/// Treat the value in the register as a memory address and dereference it by fetching the data at the address in
	/// the register to compute.
	Dereference,
	/// Identifier literal.
	/// Do not read the register being referred to by the operand and take the operand value literally.
	Literal
}

impl TryFrom<u8> for Register {
	type Error = ModeError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		if value as usize > variant_count::<Register>() {
			return Err(ModeError {});
		}
		
		Ok(match value {
			0 => Self::Direct,
			1 => Self::Dereference,
			2 => Self::Literal,
			_ => return Err(ModeError {})
		})
	}
}

/// Addressing method.
/// Determines the behavior of the parameters and how it is addressed and pre-processed. When doing reads and writes,
/// the addressing is taken into account for determining where to store or read the underlying value.
#[derive(Debug)]
pub enum Addressing {
	/// Register addressing.
	/// Deal with the register with minimal or processing.
	Register(Register),
	/// Dereference with constant offset.
	/// Add a positive offset to the dereferenced register which is pointing to an address in memory. 
	DereferenceOffset(absolute::Data),
	/// Dereference address from immediate.
	/// Dereference the address in the immediate.
	Address(absolute::Data),
	/// Immediate constant.
	/// Use the immediate as a constant value for this operand.
	Constant(absolute::Data)
}

impl Default for Addressing {
	fn default() -> Self {
		Self::Register(Register::default())
	}
}

/// Dynamic operand.
/// Allows for the value to be addressed and marked differently.
#[derive(Debug, Default)]
pub struct Dynamic {
	pub addressing: Addressing,
	pub value: u8
}