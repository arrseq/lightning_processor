use crate::operation::{Operation, RawOperationCode};

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
	fn get_code(&mut self) -> u8 {
		match self {
			Self::Add      => ADD_CODE,
			Self::Subtract => SUBTRACT_CODE
		}
	}

	fn accepts_static(&mut self) -> bool {
		todo!()
	}

	fn accepts_dynamic(&mut self) -> bool {
		todo!()
	}
}

impl TryFrom<RawOperationCode> for Arithmetic {
	type Error = ();

	fn try_from(value: RawOperationCode) -> Result<Self, Self::Error> {
		Ok(match value.0 {
			ADD_CODE      => Self::Add,
			SUBTRACT_CODE => Self::Subtract,
			_ => return Err(())
		})
	}
}