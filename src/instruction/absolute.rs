//! Unsized absolute number. 
//! While Rust has u8, u16... for absolute values, it does not have a simple enum for variable length
//! absolute integers.

/// Absolute modes.
/// Base type variants for representing an absolute value.
#[derive(Debug)]
pub enum Type {
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

/// Variable absolute data type.
/// Complete variants that annotate numbers with their type in the same enum allowing for the data type to be changed
/// during runtime.
#[derive(Debug)]
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