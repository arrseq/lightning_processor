use crate::instruction::absolute;

pub enum Register {
	Direct,
	Dereference
}

pub enum Dynamic {
	Register(Register),
	RegisterOffset(absolute::Type),
	Address(absolute::Type),
	Constant(absolute::Type)
}