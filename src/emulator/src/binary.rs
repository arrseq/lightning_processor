use std::io::Read;

/// An iterator that allows you to read streams and get a dynamic
/// number of bytes.
pub struct Traverser<Stream: Read> {
    stream: Stream
}

impl<Stream: Read> Traverser<Stream> {
    pub fn new(stream: Stream) -> Self {
        Self { stream }
    }

    /// Read a single byte from the byte stream.
    pub fn read_byte(&mut self) -> Option<u8> {
        let mut buffer = [0 as u8; 1];
        match self.stream.read(&mut buffer) {
            Err(_) => return None,
            Ok(filled_length) => {
                if filled_length != buffer.len() {
                    return None;
                }
            }
        };

        return Some(buffer[0]);
    }

    /// Read a single byte from the byte stream.
    pub fn read_word(&mut self) -> Option<u16> {
        let mut buffer = [0 as u8; 2];
        match self.stream.read(&mut buffer) {
            Err(_) => return None,
            Ok(filled_length) => {
                if filled_length != buffer.len() {
                    return None;
                }
            }
        };

        return Some(u16::from_be_bytes(buffer));
    }

    /// Read a single byte from the byte stream.
    pub fn read_dword(&mut self) -> Option<u32> {
        let mut buffer = [0 as u8; 4];
        match self.stream.read(&mut buffer) {
            Err(_) => return None,
            Ok(filled_length) => {
                if filled_length != buffer.len() {
                    return None;
                }
            }
        };

        return Some(u32::from_be_bytes(buffer));
    }

    /// Read a single byte from the byte stream.
    pub fn read_qword(&mut self) -> Option<u64> {
        let mut buffer = [0 as u8; 8];
        match self.stream.read(&mut buffer) {
            Err(_) => return None,
            Ok(filled_length) => {
                if filled_length != buffer.len() {
                    return None;
                }
            }
        };

        return Some(u64::from_be_bytes(buffer));
    }
}