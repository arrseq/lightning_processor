//! Operation codes.
//! Module contains all operations for the architecture that are classified into behavioral groups. 
//! When referring to moving data from one operand to another, this always refers to the operand data that has 
//! already been processed and the value has been extracted from where it references. 
//! 
//! There are two types of operands that are based on purpose:
//! - Source: Serves as the not destination target
//! - Target: Serves as the destination and unit to base an operation on.

use std::intrinsics::variant_count;

/// Maximum number of classifications supported by the coding.
pub const MAX_CLASSIFICATIONS: usize = 2^7;
/// Maximum number of operations supported by the coding of a single classification.
pub const MAX_OPERATIONS: usize = 2^4;

/// Thrown when trying to convert numbers into operation variants.
pub struct Error {}

/// Memory based operations.
#[derive(Debug)]
pub enum Memory {
	/// Clone source to target.
	Clone
}

impl TryFrom<u8> for Memory {
	type Error = Error;

	/// Convert a numerical index into a variant.
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		if variant_count::<Memory>() > value as usize {
			return Err(Error {});
		}
		
		Ok(match value {
			0 => Self::Clone,
			_ => return Err(Error {})
		})
	}
}

#[derive(Debug)]
pub enum Numerical {
	/// Add source to target.
	Add,
	/// Subtract source from target.
	Subtract,
	/// Multiply target by source.
	Multiply,
	/// Divide target by source.
	Divide
}

impl TryFrom<u8> for Numerical {
	type Error = Error;

	/// Convert a numerical index into a variant.
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		if variant_count::<Numerical>() > value as usize {
			return Err(Error {});
		}

		Ok(match value {
			0 => Self::Add,
			1 => Self::Subtract,
			2 => Self::Multiply,
			3 => Self::Divide,
			_ => return Err(Error {})
		})
	}
}

/// Operations that manipulate an integers sign bit.
#[derive(Debug)]
pub enum IntegerSign {
	/// Set sign to true.
	Negate,
	Invert
}

impl TryFrom<u8> for IntegerSign {
	type Error = Error;

	/// Convert a numerical index into a variant.
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		if variant_count::<IntegerSign>() > value as usize {
			return Err(Error {});
		}

		Ok(match value {
			0 => Self::Negate,
			1 => Self::Invert,
			_ => return Err(Error {})
		})
	}
}

/// Perform a logical operation on each bit of the target.
#[derive(Debug)]
pub enum Logical {
	And,
	Or,
	ExclusiveOr
}

impl TryFrom<u8> for Logical {
	type Error = Error;

	/// Convert a numerical index into a variant.
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		if variant_count::<Logical>() > value as usize {
			return Err(Error {});
		}

		Ok(match value {
			0 => Self::And,
			1 => Self::Or,
			2 => Self::ExclusiveOr,
			_ => return Err(Error {})
		})
	}
}

/// Classified based on similar function and behavior. 
#[derive(Debug)]
#[repr(u8)]
pub enum Classification {
	Memory(Memory),
	Integer(Numerical),
	/// Integer operations without sign.
	Magnitude(Numerical),
	IntegerSign(IntegerSign),
	/// Bitwise operations that apply logical operations to every bit in a byte.
	Logical(Logical),
	/// Bytewise operations apply to manipulate the entire byte and manipulation the positioning of bits.
	/// TODO: Add
}

pub struct RawOperationTarget { classification: u8, operation: u8 }

impl TryFrom<RawOperationTarget> for Classification {
	type Error = Error;

	/// Convert a numerical index into a variant.
	fn try_from(value: RawOperationTarget) -> Result<Self, Self::Error> {
		// Number of variants.
		let variants = variant_count::<Classification>();
		if value.classification as usize > variants || variants > MAX_CLASSIFICATIONS {
			return Err(Error {});
		}
		
		Ok(match value.classification {
			0 => match Memory::try_from(value.operation) {
				Err(error) => return Err(error),
				Ok(operation) => Self::Memory(operation)
			},
			1 => match Numerical::try_from(value.operation) {
				Err(error) => return Err(error),
				Ok(operation) => Self::Integer(operation)
			},
			3 => match IntegerSign::try_from(value.operation) {
				Err(error) => return Err(error),
				Ok(operation) => Self::IntegerSign(operation)
			},
			4 => match Logical::try_from(value.operation) {
				Err(error) => return Err(error),
				Ok(operation) => Self::Logical(operation)
			},
			_ => return Err(Error {})
		})
	}
}