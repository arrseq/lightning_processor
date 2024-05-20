//! [operation, direction, operation, addressing_mode, addressing_parameter, data_size, right_register, 
//! left_register, immediate_bytes...]

#![allow(internal_features)]
#![allow(clippy::unusual_byte_groupings)]
#![feature(core_intrinsics)]

// use crate::operand::Operands;
// use crate::operation::Classification;
// 
// pub mod absolute;
// pub mod coder;
// pub mod dynamic;
// pub mod operand;
// pub mod operation;
// 
// #[derive(Debug)]
// pub struct Instruction {
//     pub operation: Classification,
// }

pub mod absolute;
pub mod operation;
pub mod fault;

use std::io;
use std::io::Read;

pub struct Instruction {
    
}

pub enum DecodeError {
    IoError(io::Error),
}

impl Instruction {
    // Decode a binary stream into an instruction then store it in the target.
    pub fn decode(stream: &mut impl Read, target: &mut Instruction) -> Result<(), ()> {
        Ok(())
    }
}