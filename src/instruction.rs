use crate::instruction::operand::Operands;
use crate::instruction::operation::Classification;

pub mod absolute;
pub mod dynamic;
pub mod operand;
pub mod operation;

pub struct Instruction {
	pub operation: Classification,
	pub operands: Operands
}