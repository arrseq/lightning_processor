use rhdl_bits::Bits;
use crate::instruction::dynamic::Dynamic;

pub struct Full {
	pub first: Bits<3>,
	pub second: Dynamic
}

pub struct Second {
	pub first: Bits<3>
}

pub struct First {
	pub second: Dynamic
}

pub enum Modes {
	Full(Full),
	Second(Second),
	First(First),
	None
}

pub enum Destination {
	First,
	Second
}

pub struct Operands {
	pub storage: Modes,
	pub direction: Destination
}