//! Unsized absolute number. 
//! While Rust has u8, u16... for absolute values, it does not have a simple enum for variable length
//! absolute integers. 

// Constants

use crate::processor::processor::instruction::operand::{IMMEDIATE_EXPONENT_BYTE, IMMEDIATE_EXPONENT_DUAL, IMMEDIATE_EXPONENT_QUAD, IMMEDIATE_EXPONENT_WORD};
use crate::memory::ReadAll;

pub const BYTE_SIZE: usize = 1;
pub const WORD_SIZE: usize = 2;
pub const DUAL_SIZE: usize = 4;
pub const QUAD_SIZE: usize = 8;

// region: Array utilities.
pub trait ArrayBounds {
    /// Whether an array index is inbounds of the self array or list.
    fn in_bounds(&self, index: usize) -> bool;
    
    /// Whether an index is out of bounds to the self array.
    fn out_of_bounds(&self, index: usize) -> bool {
        !self.in_bounds(index)
    }
}

impl<T> ArrayBounds for [T] {
    fn in_bounds(&self, index: usize) -> bool {
        // Since the length is 0 if there are elements, and if the index is 0, that means every index is out of bounds.
        // An in bounds index is always smaller than the length of the array,
        index < self.len()
    }
}
// endregion

// Implementations

/// Absolute modes.
/// Base type variants for representing an absolute value.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Size {
    #[default]
    Byte,
    Word,
    Dual,
    Quad
}

impl Size {
    /// Number of bytes the current variant holds. The uint size.
    pub fn size(&self) -> u8 {
        match self {
            Self::Byte => BYTE_SIZE as u8,
            Self::Word => WORD_SIZE as u8,
            Self::Dual => DUAL_SIZE as u8,
            Self::Quad => QUAD_SIZE as u8
        }
    }
}

impl From<Data> for Size {
    fn from(value: Data) -> Self {
        match value {
            Data::Byte(_) => Self::Byte,
            Data::Word(_) => Self::Word,
            Data::Dual(_) => Self::Dual,
            Data::Quad(_) => Self::Quad
        }
    }
}

impl Size {
    /// Create from a number of bytes.
    pub fn from_size(bytes: usize) -> Option<Self> {
        Some(match bytes {
            BYTE_SIZE => Size::Byte,
            WORD_SIZE => Size::Word,
            DUAL_SIZE => Size::Dual,
            QUAD_SIZE => Size::Quad,
            _ => return None
        })
    }

    /// Create from an exponent of 2. The maximum supported exponent is 3.
    pub fn from_exponent(exponent: u8) -> Option<Self> {
        Self::from_size(2u8.pow(exponent as u32) as usize)
    }

    pub fn exponent(&self) -> u8 {
        match self {
            Self::Byte => IMMEDIATE_EXPONENT_BYTE,
            Self::Word => IMMEDIATE_EXPONENT_WORD,
            Self::Dual => IMMEDIATE_EXPONENT_DUAL,
            Self::Quad => IMMEDIATE_EXPONENT_QUAD
        }
    }
}

/// Variable absolute data type.
/// Complete variants that annotate numbers with their type in the same enum allowing for the data type to be changed
/// during runtime.
#[derive(Debug, Clone, Eq)]
pub enum Data {
    Byte(u8),
    Word(u16),
    Dual(u32),
    Quad(u64)
}

impl Data {
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            Self::Byte(number) => bytes.extend(number.to_le_bytes()),
            Self::Word(number) => bytes.extend(number.to_le_bytes()),
            Self::Dual(number) => bytes.extend(number.to_le_bytes()),
            Self::Quad(number) => bytes.extend(number.to_le_bytes()),
        }

        bytes
    }
    
    pub fn exponent(self) -> u8 {
        Size::from(self).exponent()
    }

    /// Get the data as a quad sized uint.
    pub fn quad(&self) -> u64 {
        match *self {
            Self::Byte(value) => value as u64,
            Self::Word(value) => value as u64,
            Self::Dual(value) => value as u64,
            Self::Quad(value) => value
        }
    }

    /// Fit a 64-bit number into the smallest division variant of this type.
    pub fn from_quad_selecting(quad: u64) -> Self {
        if quad <= u8::MAX as u64 { return Self::Byte(quad as u8) }
        if quad <= u16::MAX as u64 { return Self::Word(quad as u16) }
        if quad <= u32::MAX as u64 { return Self::Dual(quad as u32) }
        Self::Quad(quad)
    }
    
    /// Store a u64 into the correct type with an exponent hint. If the exponent is for a smaller number, then
    /// some information may be lost due to type conversion. If the exponent is not supported, then [None] is returned. 
    /// Only exponents 1, 2, 3 and 4 are supported.
    /// 
    /// TODO: Test
    pub fn from_exponent_selecting(exponent: u8, number: u64) -> Option<Self> {
        Some(match exponent {
            IMMEDIATE_EXPONENT_BYTE => Self::Byte(number as u8),
            IMMEDIATE_EXPONENT_WORD => Self::Word(number as u16),
            IMMEDIATE_EXPONENT_DUAL => Self::Dual(number as u32),
            IMMEDIATE_EXPONENT_QUAD => Self::Quad(number),
            _ => return None
        })
    }
    
    /// Get the number of bytes that is stored in the variant associative data of the enum.
    /// 
    /// TODO: Test
    pub fn size(&self) -> u8 {
        match self {
            Self::Byte(_) => BYTE_SIZE as u8,
            Self::Word(_) => WORD_SIZE as u8,
            Self::Dual(_) => DUAL_SIZE as u8,
            Self::Quad(_) => QUAD_SIZE as u8
        }
    }
}

// region: Converting numbers to data instances
impl From<u8> for Data {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

impl From<u16> for Data {
    fn from(value: u16) -> Self {
        Self::Word(value)
    }
}

impl From<u32> for Data {
    fn from(value: u32) -> Self {
        Self::Dual(value)
    }
}

impl From<u64> for Data {
    fn from(value: u64) -> Self {
        Self::Quad(value)
    }
}
// endregion

impl From<Size> for Data {
    fn from(value: Size) -> Self {
        match value {
            Size::Byte => Self::Byte(0),
            Size::Word => Self::Word(0),
            Size::Dual => Self::Dual(0),
            Size::Quad => Self::Quad(0)
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.quad() == other.quad()
    }
}

impl ReadAll<[u8]> for Data {
    /// Read bytes of the stored number type into a slice reference.
    /// ```
    /// use atln_processor::memory::ReadAll;
    /// use atln_processor::number::Data;
    ///
    /// let mut byte_buffer = [0u8; 1];
    ///
    /// Data::Byte(255).read_all(&mut byte_buffer); 
    /// assert_eq!(byte_buffer, [255; 1]);
    /// Data::Byte(65).read_all(&mut byte_buffer);
    /// assert_eq!(byte_buffer, [65; 1]);
    ///
    /// let mut dual_buffer = [0u8; 4];
    ///
    /// assert_eq!(Data::Dual(u16::MAX as u32).read_all(&mut dual_buffer), 4);
    /// assert_eq!(dual_buffer, [255, 255, 0, 0]);
    ///
    /// // Clean up buffer for next test.
    /// dual_buffer = [0u8; 4];
    ///
    /// // Test to ensure larger numbers still store but are chopped off.
    /// Data::Quad(u64::MAX).read_all(&mut dual_buffer);
    /// assert_eq!(dual_buffer, [255u8; 4]);
    /// ```
    fn read_all(&mut self, target: &mut [u8]) -> usize {
        let mut bytes_written = 0;
        let bytes = self.to_le_bytes();
        
        for index in 0..self.size() {
            if target.in_bounds(index as usize) {
                target[index as usize] = bytes[index as usize];
                bytes_written += 1;
                continue;
            }
            
            break;
        }

        bytes_written
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractError {
    /// The start byte index is out of bounds.
    StartOutOfBounds,
    /// The selection size is too large for the start.
    EndOutOfBounds
}

// region: Data to number conversion
impl From<Data> for u8 {
    fn from(value: Data) -> Self {
        value.quad() as u8
    }
}

impl From<Data> for u16 {
    fn from(value: Data) -> Self {
        value.quad() as u16
    }
}

impl From<Data> for u32 {
    fn from(value: Data) -> Self {
        value.quad() as u32
    }
}

impl From<Data> for u64 {
    fn from(value: Data) -> Self {
        value.quad() as u64
    }
}
// endregion