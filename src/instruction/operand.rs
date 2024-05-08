//! Operands for instructions.
//! Contains the tools for operands in instructions as well as a structure containing both of the 2 operands 
//! supported by an instruction.

use rhdl_bits::Bits;
use crate::instruction::dynamic::Dynamic;

/// First operand.
/// This always takes the register and reads the value from it to do processing. Offsets and other settings cannot be
/// applied to this specific operand.
pub type FirstOperand = Bits<3>;

/// Dual operands.
pub struct Full {
	pub first: FirstOperand,
	pub second: Dynamic
}

/// Only first operand.
pub struct First {
	pub first: FirstOperand
}

/// Only second operand.
pub struct Second {
	pub second: Dynamic
}

/// Operand presence modes.
/// Package containing configurations of how the operands accepted.
pub enum Storage {
	Full(Full),
	Second(Second),
	First(First),
	None
}

/// Destination operand.
/// The operand that should be read to determine the location in which the successful result of the computation will
/// be stored. 
#[derive(Debug)]
pub enum Destination {
	First,
	Second
}

/// Operands and data flow. 
pub struct Operands {
	pub storage: Storage,
	pub destination: Destination
}