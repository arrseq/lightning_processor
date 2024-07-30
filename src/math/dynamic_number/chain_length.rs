#[cfg(test)]
mod test;

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
    /// Decode with the unprefixed variable encoding. The `limit` specifies the number of bytes its limited to reading.
    pub fn decode_chain_length(input: &mut impl Read, limit: Option<usize>) -> Result<Self, DecodeError> {
        let mut result = Unsigned::U8(0);
        let mut buffer = [0u8; 1];
        let mut bytes_read = 0usize;
        
        loop {
            let is_last = if let Some(limit) = limit { 
                if bytes_read == limit { break };
                limit.saturating_sub(1) == bytes_read
            } 
            else { false };
            
            input.read_exact(&mut buffer).map_err(DecodeError::Io)?;

            // If `is_last` is true, then the first field of the result will be false, and the second field can be 255.
            // Result is a tuple containing whether a next byte should be read and the value this byte evaluates to.
            let (read_next, offset) = if is_last { (false, buffer[0]) } 
            else if buffer[0] == 255 { (true, 254) } 
            else { (false, buffer[0]) };
            
            if !result.upsizing_add(Unsigned::U8(offset)) { return Err(DecodeError::Overflow); }
            
            bytes_read += 1;
            if !read_next { break }
        }

        Ok(result)
    }
}