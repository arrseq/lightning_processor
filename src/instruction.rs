//! Instruction.
//! Tools and structures used for storing, decoding and encoding instructions.

use crate::instruction::operand::Operands;
use crate::instruction::operation::Classification;

pub mod absolute;
pub mod dynamic;
pub mod operand;
pub mod operation;
pub mod coder;

pub struct Instruction {
	pub operation: Classification,
	pub operands: Operands
}