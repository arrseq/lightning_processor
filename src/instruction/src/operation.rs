mod arithmetic;

use crate::fault::Fault;

pub trait Operation {
	fn get_code(&mut self) -> u8;
	fn from_code(code: u8) -> Option<Self>;
	
	fn accepts_static(&mut self) -> bool;
	fn accepts_dynamic(&mut self) -> bool;
}

pub enum Table {
	Add(arithmetic::Adder),
	AddInteger(arithmetic::IntegerAdder)
}