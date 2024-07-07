use utility::{EncodeDynamic, MaxCode, ToCode, TryCoded, TryFromCode};
use strum::{EnumCount};
use instruction::operation::{Operation, Sized};
use instruction::prefix::{Prefixes};

pub mod operand;
pub mod operation;
pub mod prefix;

pub struct Instruction {
    pub prefixes: Prefixes,
    pub operation: Operation
}

impl EncodeDynamic for Instruction {
    fn encode_dyn(&self, output: &mut Vec<u8>) {
        self.prefixes.encode_dyn(output);
        match self.operation.to_smallest_code() {
            Sized::Byte(x) => output.push(x),
            Sized::Word(x) => output.extend(x.to_le_bytes())
        }
    }
}