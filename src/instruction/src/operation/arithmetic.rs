use crate::operation::{Operation, OperationCode};

pub const ADD_CODE: u8 = 0;
pub const SUBTRACT_CODE: u8 = 1;

#[derive(Debug)]
pub enum Arithmetic {
	Add,
	Subtract
}

impl Operation for Arithmetic {
	fn get_code(&mut self) -> u8 {
		0
	}

	fn accepts_static(&mut self) -> bool {
		todo!()
	}

	fn accepts_dynamic(&mut self) -> bool {
		todo!()
	}
}

impl TryFrom<OperationCode> for Arithmetic {
	type Error = ();

	fn try_from(value: OperationCode) -> Result<Self, Self::Error> {
		Ok(match value.0 {
			ADD_CODE => Self::Add,
			SUBTRACT_CODE => Self::Subtract,
			_ => return Err(())
		})
	}
}