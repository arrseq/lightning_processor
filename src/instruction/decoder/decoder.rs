//! Decode instructions into their respective parameters from byte streams. Instructions are encoded in chunks of 
//! bytes as followed.
//! The goal was to allow for addressing the following types of instructions with the ability to flip the direction
//! of the result path.
//! - Register <-> Register
//! - Register <-> Register + Offset
//! - Register <-> Data at memory address
//! - Register <-> Constant
//!
//! Implementing requires the following to be encoded.
//! - The direction of the result path.
//! - The size of the offset.
//! - Whether it is an immediate constant or immediate address.

use std::io;
use std::io::{Read};
use rhdl_bits::Bits;

pub enum Direction {
    First,
    Second
}


// pub struct Instruction {
//     pub classification: Bits<7>,
//     pub direction: Direction,
//     
//     pub operation: Bits<4>,
//     pub exponent: Bits<2>,
//     pub addressing: Bits<2>,
//     
//     pub method: Bits<2>,
//     pub register: Bits<3>,
//     pub dynamic: Bits<3>,
//     
//     pub immediate: Size
// }

pub struct Instruction {
    // pub classification: 
}

pub type Operand = Bits<4>;
pub type Exponent = Bits<2>;
pub type Addressing = Bits<2>;

pub type TargetExponent = Bits<2>;
pub type Register = Bits<3>;
pub type Dynamic = Bits<3>;

pub type Immediate = u64;

/// Decode a stream of bytes into an instruction. Err() variant will return any error produced from accessing the 
/// read stream. If the instruction cannot be parsed due to insufficient supply of bytes or other errors, then 
/// [`Err`] will be returned.
/// 
/// # Example
/// ```
/// use std::io::Cursor;
/// 
/// let instruction_stream = Cursor::new([
///     10,
///     
/// ]);
/// 
/// 
/// ```
pub fn decode(stream: &mut impl Read) -> Result<(), io::Error> {
    let _ = [
        // Collection:
        // This contains the identifier of the instruction collection. A collection is a group of executable 
        // operations and this exists to allow for grouping instructions based on what their general purpose is.
        //
        // collection
        0b00000000,

        // Control:
        // Specifies how to control the overall functionality of the instruction and sets the instruction up for 
        // addressing setup.
        // - Operand is the instruction out of the collection to issue.
        // - Exponent refers to the exponent on 2 that determines the size of the data being operated on calculated 
        // by performing 2^exponent
        //
        // operand, exponent, direction, scaling
        0b0000_00_0_0
    ];
    todo!()
}