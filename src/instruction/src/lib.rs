#![allow(internal_features)]
#![allow(clippy::unusual_byte_groupings)]
#![feature(core_intrinsics)]

pub mod absolute;
pub mod operand;
pub mod operation;

use std::io;
use std::io::Read;
use crate::operation::{Extension, ExtensionFromCodeInvalid};

/// The operand to dereference store the operation result in.
pub enum Destination {
    Static,
    Dynamic
}

pub struct Instruction {
    pub operation: Extension,
    pub destination: Destination,
    pub x_static: operand::Static,
    pub x_dynamic: operand::Dynamic
}

pub enum DecodeError {
    /// Caused by the stream.
    Stream(io::Error),
    /// The extension of operation code was invalid.
    InvalidCode(ExtensionFromCodeInvalid)
}

impl Instruction {
    // Decode a binary stream into an instruction then store it in the target.
    pub fn decode(stream: &mut impl Read, target: &mut Instruction) -> Result<(), DecodeError> {
        Ok(())
    }
}