//! Dynamic parameter.
//! This is used to allow accepting of differently addressed data.
//! - Used for custom addressing.
//! - Supports architecture sized data through an immediate.

use rhdl_bits::Bits;
use crate::instruction::absolute;

/// Register addressing mode.
/// Mode in which a register's value is pre-processed.
pub enum Register {
	/// Direct read.
	/// Find the register value and use it directly.
	Direct,
	/// Dereference register value as memory address.
	/// Treat the value in the register as a memory address and dereference it by fetching the data at the address in
	/// the register to compute.
	Dereference,
	/// Identifier literal.
	/// Do not read the register being referred to by the operand and take the operand value literally.
	Literal
}

/// Addressing mode.
/// Determines the behavior of the parameters and how it is addressed and pre-processed. When doing reads and writes,
/// the addressing is taken into account for determining where to store or read the underlying value.
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

/// Dynamic operand.
/// Allows for the value to be addressed and marked differently.
pub struct Dynamic {
	pub addressing: Addressing,
	pub value: Bits<3>
}