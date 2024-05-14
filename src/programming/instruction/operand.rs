//! Operands for instructions.
//! Contains the tools for operands in instructions as well as a structure containing both of the 2 operands 
//! supported by an instruction.

use crate::programming::instruction::dynamic::Dynamic;

/// First operand.
/// This always takes the register and reads the value from it to do processing. Offsets and other settings cannot be
/// applied to this specific operand.
#[derive(Debug, Default)]
pub struct FirstOperand(pub u8);

/// Dual operands.
#[derive(Debug, Default)]
pub struct Full {
	pub first: FirstOperand,
	pub second: Dynamic
}

/// Only first operand.
#[derive(Debug, Default)]
pub struct First {
	pub first: FirstOperand
}

/// Only second operand.
#[derive(Debug, Default)]
pub struct Second {
	pub second: Dynamic
}

/// Operand presence modes.
/// Package containing configurations of how the operands accepted.
#[derive(Debug, Default)]
pub enum Storage {
	Full(Full),
	Second(Second),
	First(First),
	#[default]
	None
}

impl From<Mode> for Storage {
	fn from(value: Mode) -> Self {
		match value {
			Mode::Full => Self::Full(Full::default()),
			Mode::Second => Self::Second(Second::default()),
			Mode::First => Self::First(First::default()),
			Mode::None => Self::None
		}
	}
}

/// Operand presence storage mode with no storage.
#[derive(Debug, Default)]
pub enum Mode {
	Full,
	Second,
	First,
	#[default]
	None
}

impl From<Storage> for Mode {
	fn from(value: Storage) -> Self {
		match value {
			Storage::Full(_) => Self::Full,
			Storage::Second(_) => Self::Second,
			Storage::First(_) => Self::First,
			Storage::None => Self::None
		}
	}
}

/// Destination operand.
/// The operand that should be read to determine the location in which the successful result of the computation will
/// be stored. 
#[derive(Debug, Default)]
pub enum Destination {
	#[default]
	First,
	Second
}

/// Operands and data flow. 
#[derive(Debug, Default)]
pub struct Operands {
	pub destination: Destination,
	pub storage: Storage
}