use utility::{Encode, MaxCode, ToCode, TryCoded, TryFromCode};
use strum_macros::{EnumCount, FromRepr};
use strum::{EnumCount};
use instruction::operation::Operation;
use instruction::prefix::Prefix;

pub mod operand;
pub mod operation;
pub mod prefix;

pub struct Instruction {
    prefixes: Vec<Prefix>,
    operation: Operation
}