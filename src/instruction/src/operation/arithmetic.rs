use crate::operation::{Operation};

// Operation codes

pub const ADD_CODE     : u8 = 0;
pub const SUBTRACT_CODE: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arithmetic {
	Add,
	Subtract
}

// Implementation

impl Operation for Arithmetic {
	fn code(&mut self) -> u8 {
		match self {
			Self::Add      => ADD_CODE,
			Self::Subtract => SUBTRACT_CODE
		}
	}

	fn expects_static(&mut self) -> bool { true }
	fn expects_dynamic(&mut self) -> bool { true }
}

impl Arithmetic {
	pub fn from_code(code: u8) -> Option<Self> {
		Some(match code {
			ADD_CODE      => Self::Add,
			SUBTRACT_CODE => Self::Subtract,
			_ => return None
		})
	}
}