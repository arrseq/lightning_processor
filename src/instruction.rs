use utility::{Encode, MaxCode, ToCode, TryCoded, TryFromCode};
use strum_macros::{EnumCount, FromRepr};
use strum::{EnumCount};
use instruction::operation::Operation;
use instruction::prefix::Prefix;

pub mod operand;
pub mod operation;
pub mod prefix;

pub struct Instruction {
    pub prefixes: Vec<Prefix>,
    pub operation: Operation
}

impl Encode for Instruction {
    type Output = Vec<u8>;

    fn encode(&self) -> Self::Output {
        for prefix in &self.prefixes {
            let direct = prefix::Direct::from(prefix);
            dbg!(direct);
        }
        
        Vec::new()
    }
}