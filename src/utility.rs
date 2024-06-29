/// Read a vector like a stream. Read buffer.len() amount of bytes from the vector and into the buffer. This will return
/// the number of bytes read.
/// ```
/// assert!(false); // TODO: Test
/// ```
pub fn read_vec_into_buffer(vec: &Vec<u8>, start: usize, buffer: &mut [u8]) -> usize {
    let mut bytes_read = 0;
    for index in 0..buffer.len() {
        match vec.get(start + index) {
            Some(value) => buffer[index] = *value,
            None => return bytes_read
        }
        
        bytes_read += 1;
    }
    
    bytes_read
}

/// Get an identifier code for an item.
pub trait Coded<Type> {
    fn code(&mut self) -> Type;
}

pub trait Encodable<Type> {
    fn encode(&mut self) -> Type;
}