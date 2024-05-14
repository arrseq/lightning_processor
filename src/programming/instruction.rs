//! Instruction.
//! Tools and structures used for storing, decoding and encoding instructions.

use crate::programming::instruction::operand::Operands;
use crate::programming::instruction::operation::Classification;

pub mod absolute;
pub mod dynamic;
pub mod operand;
pub mod operation;
pub mod coder;

#[derive(Debug)]
pub struct Instruction {
	pub operation: Classification,
	pub operands: Operands
}