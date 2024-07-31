use std::io;
use std::io::Read;
use thiserror::Error;
use crate::math::dynamic_number::{Size, Unsigned};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Overflow occurred when adding to summation buffer")]
    Overflow,
    #[error("Failed to read next byte")]
    Io { #[source] source: io::Error }
}

impl Unsigned {
    const fn interpret(byte: u8, is_last: bool) -> (u8, bool) {
        if is_last { (byte, false) }
        else if byte == u8::MAX { (u8::MAX - 1, true) }
        else { (byte, false) }
    }
    
    pub fn decode_chain(input: &mut impl Read, max: Option<u64>) -> Result<Self, Error> {
        let mut result = 0u64;
        let mut bytes_read = 0u64;
        let mut buffer = [0u8; 1];
        
        loop {
            let is_last = if let Some(max) = max {
                if bytes_read >= max { break };
                max.saturating_sub(1) == bytes_read 
            } else { false };
            input.read_exact(&mut buffer).map_err(|source| Error::Io { source })?;
            let (byte, read_next) = Self::interpret(buffer[0], is_last);
            
            result += byte as u64;
            bytes_read += 1;
            if !read_next { break }
        }
        
        Ok(Unsigned {
            value: result,
            size: Size::get_minimum(result)
        })
    }
}