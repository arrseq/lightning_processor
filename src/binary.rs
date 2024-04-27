use std::{fmt::Write, io::Read};

/// Read a byte from the byte stream.
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

/// Read a word from the byte stream.
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

/// Read a double word from the byte stream.
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

/// Read a quad word from the byte stream.
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