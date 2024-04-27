use std::io::{Error, Read, Write};

pub fn read_byte<Stream: Read>(stream: &mut Stream) -> Option<u8> {
    let mut buffer = [0 as u8; 1];
    match stream.read(&mut buffer) {
        Err(_) => return None,
        Ok(filled_length) => {
            if filled_length != buffer.len() {
                return None;
            }
        }
    };

    return Some(buffer[0]);
}

pub fn read_word<Stream: Read>(stream: &mut Stream) -> Option<u16> {
    let mut buffer = [0 as u8; 2];
    match stream.read(&mut buffer) {
        Err(_) => return None,
        Ok(filled_length) => {
            if filled_length != buffer.len() {
                return None;
            }
        }
    };

    return Some(u16::from_be_bytes(buffer));
}

pub fn read_double_word<Stream: Read>(stream: &mut Stream) -> Option<u32> {
    let mut buffer = [0 as u8; 4];
    match stream.read(&mut buffer) {
        Err(_) => return None,
        Ok(filled_length) => {
            if filled_length != buffer.len() {
                return None;
            }
        }
    };

    return Some(u32::from_be_bytes(buffer));
}

pub fn read_quad_word<Stream: Read>(stream: &mut Stream) -> Option<u64> {
    let mut buffer = [0 as u8; 8];
    match stream.read(&mut buffer) {
        Err(_) => return None,
        Ok(filled_length) => {
            if filled_length != buffer.len() {
                return None;
            }
        }
    };

    return Some(u64::from_be_bytes(buffer));
}

pub fn write_byte<Stream: Write>(stream: &mut Stream, byte: u8) -> Result<(), Error> {
    let buffer = [byte];
    match stream.write(&buffer) {
        Err(e) => return Err(e),
        Ok(_) => return Ok(())
    }
}

pub fn write_word<Stream: Write>(stream: &mut Stream, word: u16) -> Result<(), Error> {
    let buffer = word.to_le_bytes();
    match stream.write(&buffer) {
        Err(e) => return Err(e),
        Ok(_) => return Ok(())
    }
}

pub fn write_double_word<Stream: Write>(stream: &mut Stream, double_word: u32) -> Result<(), Error> {
    let buffer = double_word.to_le_bytes();
    match stream.write(&buffer) {
        Err(e) => return Err(e),
        Ok(_) => return Ok(())
    }
}

pub fn write_quad_word<Stream: Write>(stream: &mut Stream, quad_word: u64) -> Result<(), Error> {
    let buffer = quad_word.to_le_bytes();
    match stream.write(&buffer) {
        Err(e) => return Err(e),
        Ok(_) => return Ok(())
    }
}