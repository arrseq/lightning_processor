use utility::{EncodeDynamic, MaxCode, ToCode, TryCoded, TryFromCode};
use strum::{EnumCount};
use instruction::operation::{Extension, Operation, Sized};
use instruction::prefix::{Prefixes};

pub mod operand;
pub mod operation;
pub mod prefix;

pub struct Instruction {
    pub synchronize: bool,
    pub operation: Operation
}

impl EncodeDynamic for Instruction {
    fn encode_dyn(&self, output: &mut Vec<u8>) {
        let op_code =  self.operation.to_smallest_code();
        let extension = Extension::from(&self.operation);
        
        let prefixes = Prefixes {
            escape: operation::Size::from(&op_code),
            // The default extension is always the basic extension, so to avoid adding unnecessary prefixes, add the
            // extension prefix indicator only if It's something other than the default.
            extension: if !matches!(extension, Extension::Basic) { Some(extension) } else { None },
            execution_mode: None,
            branch_likely_taken: None
        };
        
        prefixes.encode_dyn(output);
        match op_code {
            Sized::Byte(x) => output.push(x),
            Sized::Word(x) => output.extend(x.to_le_bytes())
        }
    }
}