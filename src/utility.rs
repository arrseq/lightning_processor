use std::borrow::Cow;
use std::ops::Deref;

/// Read a vector like a stream. Read buffer.len() amount of bytes from the vector and into the buffer. This will return
/// the number of bytes read.
/// ```
/// assert!(false); // TODO: Test
/// ```
pub fn read_bytes_into_buffer<X: AsRef<[u8]>>(vec: &X, start: usize, buffer: &mut [u8]) -> usize {
    let mut bytes_read = 0;
    let vec_slice = vec.as_ref();

    for index in 0..buffer.len() {
        match vec_slice.get(start + index) {
            Some(&value) => buffer[index] = value,
            None => return bytes_read,
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
pub fn write_buffer_into_bytes<X: AsRef<[u8]> + AsMut<[u8]>>(vec: &mut X, start: usize, buffer: &[u8]) -> usize {
    let mut bytes_written = 0;
    let vec_len = vec.as_ref().len();
    let buffer_len = buffer.len();

    // Get a mutable reference to the underlying slice
    let vec_mut = vec.as_mut();

    // Write to the buffer, stopping if we run out of space
    for index in 0..buffer_len {
        if start + index >= vec_len {
            break;
        }
        vec_mut[start + index] = buffer[index];
        bytes_written += 1;
    }

    bytes_written
}

/// Get an identifier code for an item.
pub trait CodedLegacy<Type> {
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













// region: Coded items that use identifiers from some documentation.
pub trait ToCode {
    type Code;
    fn to_code(&self) -> Self::Code;
}

pub trait TryFromCode: Sized {
    type Code;
    
    /// Try to get an instance of the item from a code. [None] returned for an invalid code.
    fn try_from_code(code: Self::Code) -> Option<Self>;
}

pub trait FromCode: Sized {
    type Code;

    /// Get an instance of the item from a code.
    fn from_code(code: Self::Code) -> Self;
}

pub trait TryCoded: TryFromCode + ToCode {}
pub trait Coded: FromCode + ToCode {}
// endregion

// region: Traits for encoded items.
/// It is assumed that whatever is being encoded is also valid.
pub trait Encode {
    type Output;

    /// Encode this item into some encoded output.
    fn encode(&self) -> Self::Output;
}

/// Decide a valid structure into a valid encoded form.
pub trait Decode {
    type Input;

    /// Decode an encoded input into self.
    fn decode(input: Self::Input) -> Self;
}

/// Try to decode a potentially invalid encoded form of a structure.
pub trait TryDecode: Sized {
    type Input;
    type Error;
    
    /// Decide a potentially invalid input into result.
    fn try_decode(input: Self::Input) -> Result<Self::Error, Self>;
}
// endregion

// region: Get the binary max value from a number of bits.
#[const_trait]
pub trait MaxWithBits: Sized {
    /// As a number of bits, get the largest number that could represent. Returns [None] if the number of bits is 0.
    fn max_with_bits(&self) -> Option<Self>;
}

impl const MaxWithBits for usize {
    /// ```
    /// use atln_processor::utility::MaxWithBits;
    /// 
    /// assert_eq!(4usize.max_with_bits().unwrap(), 15);
    /// assert_eq!(1usize.max_with_bits().unwrap(), 1);
    /// assert_eq!(0usize.max_with_bits(), None);
    /// ```
    fn max_with_bits(&self) -> Option<Self> {
        if *self == 0 { return None; }
        Some(2usize.pow(*self as u32) - 1)
    }
}
// endregion