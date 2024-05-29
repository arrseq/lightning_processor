#![allow(internal_features)]
#![allow(clippy::unusual_byte_groupings)]
#![feature(core_intrinsics)]

pub mod absolute;
pub mod operand;
pub mod operation;

use std::io;
use std::io::Read;
use crate::operand::{Operand, Operands};
use crate::operation::{Extension, ExtensionFromCodeInvalid};

/// The operand to dereference store the operation result in.
pub enum Destination {
    Static,
    Dynamic
}

/// ```
/// use em_instruction::{absolute, Destination, Instruction};
/// use em_instruction::operand::{AllPresent, Dynamic, Operands, Static};
/// use em_instruction::operation::arithmetic::Arithmetic;
/// use em_instruction::operation::Extension;
///
/// let operation = Instruction {
///     operation:     Extension::Arithmetic(Arithmetic::Add),
///     width:         absolute::Type::Byte,
///     destination:   Destination::Static, // Store value in r0
///     operands:      Operands::AllPresent(AllPresent {
///         x_static:  Static(Some(0)), // r0 target
///         x_dynamic: Dynamic::Constant(absolute::Data::Byte(5))
///     })
/// };
/// ```
pub struct Instruction {
    pub operation: Extension,
    /// Width of operands when dereferenced and for storing result.
    pub width: absolute::Type,
    pub destination: Destination,
    pub operands: Operands
}

pub enum DecodeError {
    /// Caused by the stream.
    Stream(io::Error),
    /// The extension of operation code was invalid.
    InvalidCode(ExtensionFromCodeInvalid)
}

/// Caused by using a destination which corresponds to an operand that is not provided.
pub enum DestinationError {
    
}

impl Instruction {
    // Decode a binary stream into an instruction then store it in the target.
    pub fn decode(stream: &mut impl Read, target: &mut Instruction) -> Result<(), DecodeError> {
        Ok(())
    }
    
    /// Get the operand that the destination property corresponds to.
    pub fn destination(&self) -> Result<Operand, DestinationError> {
        Ok(match self.destination {
            Destination::Static => todo!(),
            _ => todo!()
        })
    }
}