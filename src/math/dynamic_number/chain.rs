use std::io;
use std::io::{Read, Write};
use thiserror::Error;
use crate::math::dynamic_number::{Size, Unsigned};

impl Unsigned {
    const fn decoder_interpret(byte: u8, is_last: bool) -> (u8, bool) {
        if is_last { (byte, false) }
        else if byte == u8::MAX { (u8::MAX - 1, true) }
        else { (byte, false) }
    }
    
    pub fn decode_chain(input: &mut impl Read, max: Option<u64>) -> io::Result<Self> {
        let mut result = 0u64;
        let mut bytes_read = 0u64;
        let mut buffer = [0u8; 1];
        
        loop {
            let is_last = if let Some(max) = max {
                if bytes_read >= max { break };
                max.saturating_sub(1) == bytes_read 
            } else { false };
            
            input.read_exact(&mut buffer)?;
            let (byte, read_next) = Self::decoder_interpret(buffer[0], is_last);
            
            result += byte as u64;
            bytes_read += 1;
            if !read_next { break }
        }
        
        Ok(Unsigned {
            value: result,
            size: Size::get_minimum(result)
        })
    }
    
    pub const fn encoder_interpret(remaining: u64, cap_end: bool) -> Option<(u8, u8)> {
        let subtracted: u64 = if cap_end { 0 } else { 1 }; 
        if remaining > u8::MAX as u64 - subtracted { Some((u8::MAX, u8::MAX - 1)) }
            else if remaining == 0 { None }
            else { Some((remaining as u8, remaining as u8)) }
    }
    
    pub fn encode_chain(self, output: &mut impl Write, cap_end: bool) -> io::Result<()> {
        let mut remaining = self.value;
        let mut bytes_written = 0u64;
        
        loop {
            let need_to_write = Self::encoder_interpret(remaining, cap_end);
            if let Some((output_byte, value_byte)) = need_to_write { 
                remaining -= value_byte as u64;
                output.write_all(&[output_byte])?;
            }
        
            if remaining == 0 { break; }
        }
        
        Ok(())
    }
}