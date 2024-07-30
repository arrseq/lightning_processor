use std::io;
use std::io::Read;
use thiserror::Error;
use crate::math::dynamic_number::{Unsigned};

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Overflow occurred when adding to summation buffer")]
    Overflow,
    #[error("Failed to read next byte")]
    Io(#[source] io::Error)
}

impl Unsigned {
    pub fn decode_unprefixed(input: &mut impl Read) -> Result<Self, DecodeError> {
        /// # Result
        /// Tuple containing whether a next byte should be read and the value this byte evaluates to.
        const fn evaluate(byte: u8) -> (bool, u8) {
            if byte == 255 { (true, 254) } else { (false, byte) }
        }

        let mut result = Unsigned::U8(0);
        let mut buffer = [0u8; 1];

        loop {
            input.read_exact(&mut buffer).map_err(DecodeError::Io)?;
            let (read_next, offset) = evaluate(buffer[0]);
            if !result.upsizing_add(Unsigned::U8(offset)) { return Err(DecodeError::Overflow); }

            if !read_next { break }
        }

        Ok(result)
    }
}