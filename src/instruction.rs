use utility::{EncodeDynamic, MaxCode, ToCode, TryCoded, TryFromCode};
use strum::{EnumCount};
use instruction::operation::Operation;
use instruction::prefix::{Prefixes};

pub mod operand;
pub mod operation;
pub mod prefix;

pub struct Instruction {
    pub prefixes: Prefixes,
    pub operation: Operation
}

#[derive(Debug)]
pub enum EncodeError {
    
}

impl EncodeDynamic for Instruction {
    fn encode_dyn(&self, output: &mut Vec<u8>) {
        output.clear();
        self.prefixes.encode_dyn(output);
    }
}