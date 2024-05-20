use crate::fault::Fault;
use crate::operation::Operation;

pub struct Adder {
	
}

impl Operation for Adder {
	fn get_code(&mut self) -> u8 {
		todo!()
	}

	fn from_code(code: u8) -> Result<(), impl Self> {
		todo!()
	}

	fn accepts_primary(&mut self) -> bool {
		todo!()
	}

	fn accepts_dynamic(&mut self) -> bool {
		todo!()
	}

	fn accepts_immediate(&mut self) -> bool {
		todo!()
	}

	fn execute(&mut self) -> Result<(), Fault> {
		todo!()
	}
}

pub struct IntegerAdder {
	
}

impl Operation for IntegerAdder {
	fn get_code(&mut self) -> u8 {
		todo!()
	}

	fn from_code(code: u8) -> Result<(), impl Self> {
		todo!()
	}

	fn accepts_primary(&mut self) -> bool {
		todo!()
	}

	fn accepts_dynamic(&mut self) -> bool {
		todo!()
	}

	fn accepts_immediate(&mut self) -> bool {
		todo!()
	}

	fn execute(&mut self) -> Result<(), Fault> {
		todo!()
	}
}