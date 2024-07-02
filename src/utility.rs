use std::borrow::Cow;

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

/// Write buffer.len() amount of bytes into the vector starting from the start index. This will return
/// the number of bytes written.
/// ```
/// assert!(false); // TODO: Test
/// ```
pub fn write_buffer_into_vec(vec: &mut Vec<u8>, start: usize, buffer: &[u8]) -> usize {
    let mut bytes_written = 0;
    // Ensure the vector has enough space to accommodate the new data
    if vec.len() < start + buffer.len() {
        vec.resize(start + buffer.len(), 0);
    }
    for index in 0..buffer.len() {
        vec[start + index] = buffer[index];
        bytes_written += 1;
    }
    bytes_written
}

/// Get an identifier code for an item.
pub trait Coded<Type> {
    fn code(&self) -> Type;
}

pub trait Encodable<Type> {
    fn encode(&self) -> Type;
}

/// Allows an object to be represented as some text.
pub trait Representable<'a> {
    fn representation(&self) -> Cow<'a, str>;
}

pub trait FromRepresentation<'a>: Sized {
    fn from_representation(string: Cow<'a, str>) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub enum Bracket {
    Opening,
    Closing,
}

impl<'a> Representable<'a> for Bracket {
    fn representation(&self) -> Cow<'a, str> {
        match self {
            Self::Opening => "[",
            Self::Closing => "]"
        }.into()
    }
}

impl<'a> FromRepresentation<'a> for Bracket {
    fn from_representation(string: Cow<'a, str>) -> Option<Self> {
        Some(match &*string {
            "[" => Self::Opening,
            "]" => Self::Closing,
            _ => return None
        })
    }
}

/// Read all of a structure into another buffer of some sort. This is similar to [Read] with the difference being that
/// all data is read into the buffer and any that don't fit are simply truncated.
///
/// Use this on things such as enums or things without structures. This is impropper and not good, this trait is a retro 
/// fit due to poor early planing, things like [Data] are too deeply nested and implemented to be refactored into a
/// structure to then be later used with Read.
pub trait ReadAll<T>
where
    T: ?Sized {
    /// Read some container and store the result inside a target somehow. This returns the number of bytes stored.
    fn read_all(&self, target: &mut T) -> usize;
}

pub trait LastError<E> {
    /// Get the last emitted error from a member of the parent object.
    fn last_error(&self) -> &Option<E>;
}