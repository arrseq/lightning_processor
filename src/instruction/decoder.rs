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
//!
//! ```
//! /// The sections of each byte is listed at the bottom of each of the following comments.
//! let _ = [
//!     // Collection:
//!     // This contains the identifier of the instruction collection. A collection is a group of executable 
//!     // operations and this exists to allow for grouping instructions based on what their general purpose is.
//!     //
//!     // Collection
//!     0b00000000,
//! 
//!     // Control:
//!     // Specifies how to control the overall functionality of the instruction and sets the instruction up for 
//!     // addressing setup.
//!     // - Operand is the instruction out of the collection to issue.
//!     // - Exponent refers to the exponent on 2 that determines the size of the data being operated on calculated 
//!     // by performing 2^exponent.
//!     // - Direction refers to which operand the result of the computation should be stored in.
//!     // - Offset Most refers to the most significant bit in the offset exponent. The scaling exponent is 
//!     // then used as 2^scaling exponent to determine the immediate bytes count.
//!     //
//!     // Operand, Exponent, Direction, Offset Most
//!     0b0000_00_0_0,
//! 
//!     // Addressing:
//!     // This byte is used to specify addressing mode and register information for the instruction.
//!     //
//!     // Source Target, Control, Register, 
//!     0b0_0_000_000
//! ];
//! ```

use std::io;
use std::io::{Read};

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