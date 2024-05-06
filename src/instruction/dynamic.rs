use rhdl_bits::Bits;
use crate::instruction::absolute;

pub enum Register {
	Direct,
	Dereference
}

pub enum Addressing {
	Register(Register),
	RegisterOffset(absolute::Data),
	Address(absolute::Data),
	Constant(absolute::Data)
}

pub struct Dynamic {
	pub addressing: Addressing,
	pub value: Bits<3>
}