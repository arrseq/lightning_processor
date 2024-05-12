//! Operation codes.
//! Module contains all operations for the architecture that are classified into behavioral groups. 
//! When referring to moving data from one operand to another, this always refers to the operand data that has 
//! already been processed and the value has been extracted from where it references. 
//! 
//! There are two types of operands that are based on purpose:
//! - Source: Serves as the not destination target
//! - Target: Serves as the destination and unit to base an operation on.

use std::intrinsics::variant_count;
use crate::instruction::operand;

/// Maximum number of classifications supported by the coding.
pub const MAX_CLASSIFICATIONS: usize = 2^7;
/// Maximum number of operations supported by the coding of a single classification.
pub const MAX_OPERATIONS: usize = 2^4;
/// Operation code type used for referring to operations by their numeric identifier.
pub struct OperationNumeric(pub u8);

pub trait StorageMode {
	/// Get the operand storage mode of the classification.
	fn get_storage() -> operand::StorageMode;
}

/// Thrown when trying to convert numbers into operation variants.
pub struct InvalidOperation {}

/// Memory based operations.
#[derive(Debug)]
pub enum Memory {
	/// Clone source to target.
	Clone
}

impl StorageMode for Memory {
	fn get_storage() -> operand::StorageMode {
		operand::StorageMode::Full
	}
}

type Garbage = u8;
impl TryFrom<Garbage> for Memory {
	type Error = ();

	fn try_from(value: Garbage) -> Result<Self, Self::Error> {
		todo!()
	}
}

impl TryFrom<OperationNumeric> for Memory {
	type Error = InvalidOperation;

	/// Convert a numerical index into a variant.
	/// ```
	/// #![feature(variant_count)]
	///
	/// use std::mem::variant_count;
	/// use exr_p::instruction::operation::Memory;
	///
	/// // Test to see if overflow errors work.
	/// assert_eq!(Memory::try_from(10).is_err(), true);
	/// assert_eq!(Memory::try_from(0).is_ok(), true);
	/// assert_eq!(Memory::try_from(variant_count::<Memory>() as u8).is_err(), true);
	/// ```
	fn try_from(value: OperationNumeric) -> Result<Self, Self::Error> {
		// To ensure the number of supported instructions stays between the bounds of the instruction coding.
		if variant_count::<Memory>() - 1 < value.0 as usize {
			return Err(InvalidOperation {});
		}

		Ok(match value.0 {
			0 => Self::Clone,
			_ => return Err(InvalidOperation {})
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

impl StorageMode for Numerical {
	fn get_storage() -> operand::StorageMode {
		operand::StorageMode::Full
	}
}

impl TryFrom<OperationNumeric> for Numerical {
	type Error = InvalidOperation;

	/// Convert a numerical index into a variant.
	/// ```
	/// #![feature(variant_count)]
	///
	/// use std::mem::variant_count;
	/// use exr_p::instruction::operation::Numerical;
	///
	/// // Test to see if overflow errors work.
	/// assert_eq!(Numerical::try_from(10).is_err(), true);
	/// assert_eq!(Numerical::try_from(0).is_ok(), true);
	/// assert_eq!(Numerical::try_from(variant_count::<Numerical>() as u8).is_err(), true);
	/// ```
	fn try_from(value: OperationNumeric) -> Result<Self, Self::Error> {
		// To ensure the number of supported instructions stays between the bounds of the instruction coding.
		if variant_count::<Numerical>() - 1 < value.0 as usize {
			return Err(InvalidOperation {});
		}

		Ok(match value.0 {
			0 => Self::Add,
			1 => Self::Subtract,
			2 => Self::Multiply,
			3 => Self::Divide,
			_ => return Err(InvalidOperation {})
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

impl StorageMode for IntegerSign {
	fn get_storage() -> operand::StorageMode {
		operand::StorageMode::Second
	}
}

impl TryFrom<OperationNumeric> for IntegerSign {
	type Error = InvalidOperation;

	/// Convert a numerical index into a variant.
	/// ```
	/// #![feature(variant_count)]
	///
	/// use std::mem::variant_count;
	/// use exr_p::instruction::operation::IntegerSign;
	///
	/// // Test to see if overflow errors work.
	/// assert_eq!(IntegerSign::try_from(10).is_err(), true);
	/// assert_eq!(IntegerSign::try_from(0).is_ok(), true);
	/// assert_eq!(IntegerSign::try_from(variant_count::<IntegerSign>() as u8).is_err(), true);
	/// ```
	fn try_from(value: OperationNumeric) -> Result<Self, Self::Error> {
		// To ensure the number of supported instructions stays between the bounds of the instruction coding.
		if variant_count::<IntegerSign>() - 1 < value.0 as usize {
			return Err(InvalidOperation {});
		}

		Ok(match value.0 {
			0 => Self::Negate,
			1 => Self::Invert,
			_ => return Err(InvalidOperation {})
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

impl StorageMode for Logical {
	fn get_storage() -> operand::StorageMode {
		operand::StorageMode::Second
	}
}

impl TryFrom<OperationNumeric> for Logical {
	type Error = InvalidOperation;

	/// Convert a numerical index into a variant.
	/// ```
	/// #![feature(variant_count)]
	///
	/// use std::mem::variant_count;
	/// use exr_p::instruction::operation::Logical;
	///
	/// // Test to see if overflow errors work.
	/// assert_eq!(Logical::try_from(10).is_err(), true);
	/// assert_eq!(Logical::try_from(0).is_ok(), true);
	/// assert_eq!(Logical::try_from(variant_count::<Logical>() as u8).is_err(), true);
	/// ```
	fn try_from(value: OperationNumeric) -> Result<Self, Self::Error> {
		// To ensure the number of supported instructions stays between the bounds of the instruction coding.
		if variant_count::<Logical>() - 1 < value.0 as usize {
			return Err(InvalidOperation {});
		}

		Ok(match value.0 {
			0 => Self::And,
			1 => Self::Or,
			2 => Self::ExclusiveOr,
			_ => return Err(InvalidOperation {})
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
	// Bytewise operations apply to manipulate the entire byte and manipulation the positioning of bits.
	// TODO: Add
}

pub struct RawOperationTarget { pub classification: u8, pub operation: u8 }
pub enum Invalid {
	Classification,
	Operation
}

impl TryFrom<RawOperationTarget> for Classification {
	type Error = Invalid;

	/// Convert a numerical index into a variant.
	///
	fn try_from(value: RawOperationTarget) -> Result<Self, Self::Error> {
		// Number of variants.
		let variants = variant_count::<Classification>();
		if value.classification as usize > variants || variants > MAX_CLASSIFICATIONS {
			return Err(Invalid::Classification);
		}

		Ok(match value.classification {
			0 => match Memory::try_from(OperationNumeric(value.operation)) {
				Err(_) => return Err(Invalid::Operation),
				Ok(operation) => Self::Memory(operation)
			},
			1 => match Numerical::try_from(OperationNumeric(value.operation)) {
				Err(_) => return Err(Invalid::Operation),
				Ok(operation) => Self::Integer(operation)
			},
			3 => match IntegerSign::try_from(OperationNumeric(value.operation)) {
				Err(_) => return Err(Invalid::Operation),
				Ok(operation) => Self::IntegerSign(operation)
			},
			4 => match Logical::try_from(OperationNumeric(value.operation)) {
				Err(_) => return Err(Invalid::Operation),
				Ok(operation) => Self::Logical(operation)
			},
			_ => return Err(Invalid::Classification)
		})
	}
}