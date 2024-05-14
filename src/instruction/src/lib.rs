#![allow(internal_features)]
#![allow(clippy::unusual_byte_groupings)]
#![feature(core_intrinsics)]

use crate::operand::Operands;
use crate::operation::Classification;

pub mod absolute;
pub mod coder;
pub mod dynamic;
pub mod operand;
pub mod operation;

#[derive(Debug)]
pub struct Instruction {
    pub operation: Classification,
    pub operands: Operands
}