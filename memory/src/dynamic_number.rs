/// Refers to a specific size that a dynamic number could have. These are based on the 4 data sizes a processor can 
/// address.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Byte,
    Word,
    DoubleWord,
    QuadWord
}

/// An invalid exponent representation for a numeric size was used.
#[derive(Debug)]
pub struct InvalidExponentRepresentationError;

#[derive(Debug)]
pub struct InvalidByteCountError;

impl Size {
    pub const WORD_BYTES: u8 = 2;
    pub const DOUBLE_WORD_BYTES: u8 = 4;
    pub const QUAD_WORD_BYTES: u8 = 8;

    /// Get the exponent that needs to be put to the power of 2 to represent the number of bytes used to store
    /// this number.
    ///
    /// # Result
    /// The exponent representation is at most 2 bits.
    pub fn exponent_representation(self) -> u8 {
        match self {
            Self::Byte => 0,
            Self::Word => 1,
            Self::DoubleWord => 2,
            Self::QuadWord => 3
        }
    }

    /// # Result
    /// The exponent representation is at most 2 bits.
    pub fn from_exponent_representation(exponent: u8) -> Result<Self, InvalidExponentRepresentationError> {
        Ok(match exponent {
            0 => Self::Byte,
            1 => Self::Word,
            2 => Self::DoubleWord,
            3 => Self::QuadWord,
            _ => return Err(InvalidExponentRepresentationError)
        })
    }

    pub fn byte_count(self) -> u8 {
        match self {
            Self::Byte => 1,
            Self::Word => Self::WORD_BYTES,
            Self::DoubleWord => Self::DOUBLE_WORD_BYTES,
            Self::QuadWord => Self::QUAD_WORD_BYTES
        }
    }

    pub fn from_byte_count(byte_count: u8) -> Result<Self, InvalidByteCountError> {
        Ok(match byte_count {
            1 => Self::Byte,
            Self::WORD_BYTES => Self::Word,
            Self::DOUBLE_WORD_BYTES => Self::DoubleWord,
            Self::QUAD_WORD_BYTES => Self::QuadWord,
            _ => return Err(InvalidByteCountError)
        })
    }
}

impl From<Unsigned> for Size {
    fn from(value: Unsigned) -> Self {
        match value {
            Unsigned::Byte(_) => Self::Byte,
            Unsigned::Word(_) => Self::Word,
            Unsigned::DoubleWord(_) => Self::DoubleWord,
            Unsigned::QuadWord(_) => Self::QuadWord
        }
    }
}
impl From<Signed> for Size {
    fn from(value: Signed) -> Self {
        match value {
            Signed::Byte(_) => Self::Byte,
            Signed::Word(_) => Self::Word,
            Signed::DoubleWord(_) => Self::DoubleWord,
            Signed::QuadWord(_) => Self::QuadWord
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unsigned {
    Byte(u8),
    Word(u16),
    DoubleWord(u32),
    QuadWord(u64)
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signed {
    Byte(u8),
    Word(u16),
    DoubleWord(u32),
    QuadWord(u64)
}

macro_rules! implement_conversion {
    ($target: ident, $primitive_number: ty) => {
        impl From<$target> for $primitive_number {
            fn from(value: $target) -> Self {
                match value {
                    $target::Byte(byte) => byte as $primitive_number,
                    $target::Word(word) => word as $primitive_number,
                    $target::DoubleWord(double_word) => double_word as $primitive_number,
                    $target::QuadWord(quad_word) => quad_word as $primitive_number
                }
            }
        }
    };
}
macro_rules! implement_primitive_conversion {
    ($target: ident) => {
        implement_conversion!($target, u8);
        implement_conversion!($target, u16);
        implement_conversion!($target, u32);
        implement_conversion!($target, u64);
        implement_conversion!($target, i8);
        implement_conversion!($target, i16);
        implement_conversion!($target, i32);
        implement_conversion!($target, i64);
    };
}
implement_primitive_conversion!(Unsigned);
implement_primitive_conversion!(Signed);

macro_rules! implement_resize {
    ($target: ident) => {
        impl $target {
            pub fn resize(self, new_size: Size) -> Self {
                if Size::from(self) == new_size { return self }
                match new_size {
                    Size::Byte => Self::Byte(u8::from(self)),
                    Size::Word => Self::Word(u16::from(self)),
                    Size::DoubleWord => Self::DoubleWord(u32::from(self)),
                    Size::QuadWord => Self::QuadWord(u64::from(self))
                }
            }
        }
    };
}
implement_resize!(Unsigned);
implement_resize!(Signed);