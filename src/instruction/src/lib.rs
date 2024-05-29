#![allow(clippy::unusual_byte_groupings)]

pub mod absolute;
pub mod operand;
pub mod operation;

use std::io::Read;
use crate::operand::{Operand, Operands};
use crate::operation::{Extension, ExtensionFromCodeInvalid};

// Bit masks for decoding instructions.

pub const DRIVER0_EXTENSION_MASK  : u8 = 0b111111_0_0;
pub const DRIVER0_LOCK_MASK       : u8 = 0b000000_1_0;
pub const DRIVER0_DESTINATION_MASK: u8 = 0b000000_0_1;

pub const DRIVER1_OPERATION_MASK  : u8 = 0b1111_00_00;
pub const DRIVER1_ADDRESSING_MASK : u8 = 0b0000_11_00;
pub const DRIVER1_ADDRESSING_PARAM: u8 = 0b0000_00_11;

// Instruction implementation, decoder and utilities

/// The operand to dereference store the operation result in.
#[derive(Debug, Clone, PartialEq, Eq)]
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
///         x_static:  0, // r0 target
///         x_dynamic: Dynamic::Constant(absolute::Data::Byte(5))
///     })
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub operation: Extension,
    /// Width of operands when dereferenced and for storing result.
    pub width: absolute::Type,
    pub destination: Destination,
    pub operands: Operands
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Caused by the stream.
    Stream,
    /// The extension of operation code was invalid.
    InvalidCode(ExtensionFromCodeInvalid)
}

/// Caused by using a destination which corresponds to an operand that is not provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DestinationError {
    /// The static operand wasn't present.
    Static,
    /// The dynamic operand wasn't present.
    Dynamic
}

impl Instruction {
    // Decode a binary stream into an instruction then store it in the target.
    pub fn decode(stream: &mut impl Read, target: &mut Instruction) -> Result<(), DecodeError> {
        Ok(())
    }
    
    /// Get the operand that the destination property corresponds to.
    /// ```
    /// use em_instruction::{Destination, Instruction};
    /// use em_instruction::absolute::{Data, Type};
    /// use em_instruction::operand::{AllPresent, Dynamic, Operand, Operands};
    /// use em_instruction::operation::arithmetic::Arithmetic;
    /// use em_instruction::operation::Extension;
    ///
    /// let mut instruction = Instruction {
    ///     operation: Extension::Arithmetic(Arithmetic::Add),
    ///     width: Type::Byte,
    ///     destination: Destination::Static,
    ///     operands: Operands::AllPresent(AllPresent {
    ///         x_static: 10,
    ///         x_dynamic: Dynamic::Constant(Data::Byte(5))
    ///     })
    /// };
    ///
    /// assert!(match instruction.destination().unwrap() {
    ///     Operand::Static(_) => true,
    ///     _                  => false
    /// });
    /// 
    /// instruction.destination = Destination::Dynamic;
    /// 
    /// assert!(match instruction.destination().unwrap() {
    ///     Operand::Static(_) => false,
    ///     _                  => true
    /// });
    /// ```
    pub fn destination(&self) -> Result<Operand, DestinationError> { // "I Am NOT AGAINST YOU SIR" lmao
        Ok(match self.destination {
            Destination::Static => match self.operands.x_static() {
                Some(x_static) => Operand::Static(x_static),
                None => return Err(DestinationError::Static)
            },
            Destination::Dynamic => match self.operands.x_dynamic() {
                Some(x_dynamic) => Operand::Dynamic(x_dynamic.clone()),
                None => return Err(DestinationError::Dynamic)
            }
        })
    }
}
