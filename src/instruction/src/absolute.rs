//! Unsized absolute number. 
//! While Rust has u8, u16... for absolute values, it does not have a simple enum for variable length
//! absolute integers. 

// TODO: Use TryFrom<u8..u16..u32..u64> on Data

// Constants

pub const BYTE_SIZE: u8 = 1;
pub const WORD_SIZE: u8 = 2;
pub const DUAL_SIZE: u8 = 4;
pub const QUAD_SIZE: u8 = 8;

// Implementations

/// Absolute modes.
/// Base type variants for representing an absolute value.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Type {
	#[default]
	Byte,
	Word,
	Dual,
	Quad
}

impl From<Data> for Type {
	fn from(value: Data) -> Self {
		match value {
			Data::Byte(_) => Self::Byte,
			Data::Word(_) => Self::Word,
			Data::Dual(_) => Self::Dual,
			Data::Quad(_) => Self::Quad
		}
	}
}

impl Type {
	pub fn from_bytes(value: u8) -> Option<Self> {
		Some(match value {
			BYTE_SIZE => Type::Byte,
			WORD_SIZE => Type::Word,
			DUAL_SIZE => Type::Dual,
			QUAD_SIZE => Type::Quad,
			_ => return None
		})
	}
}

/// Variable absolute data type.
/// Complete variants that annotate numbers with their type in the same enum allowing for the data type to be changed
/// during runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
	Byte(u8),
	Word(u16),
	Dual(u32),
	Quad(u64)
}

impl From<Type> for Data {
	fn from(value: Type) -> Self {
		match value {
			Type::Byte => Self::Byte(0),
			Type::Word => Self::Word(0),
			Type::Dual => Self::Dual(0),
			Type::Quad => Self::Quad(0)
		}
	}
}