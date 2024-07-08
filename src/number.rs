//! Unsized absolute number. 
//! While Rust has u8, u16... for absolute values, it does not have a simple enum for variable length
//! absolute integers. 

// Constants

use utility::ReadAll;

pub mod high;
pub mod low;

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
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<&Number> for Size {
    fn from(value: &Number) -> Self {
        match value {
            Number::Byte(_) => Self::Byte,
            Number::Word(_) => Self::Word,
            Number::Dual(_) => Self::Dual,
            Number::Quad(_) => Self::Quad
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
            Self::Byte => 0,
            Self::Word => 1,
            Self::Dual => 2,
            Self::Quad => 3
        }
    }

    /// Generate an slice buffer with the correct number of bytes for this instance.
    pub fn buffer<'a>(&self, source_buffer: &'a [u8; 8]) -> &'a [u8] {
        match self {
            Self::Byte => &source_buffer[0..1],
            Self::Word => &source_buffer[0..2],
            Self::Dual => &source_buffer[0..4],
            Self::Quad => &source_buffer[0..8]
        }
    }
}

/// Variable absolute data type.
/// Complete variants that annotate numbers with their type in the same enum allowing for the data type to be changed
/// during runtime.
#[derive(Debug, Clone, Eq, Copy)]
pub enum Number {
    Byte(u8),
    Word(u16),
    Dual(u32),
    Quad(u64)
}

impl Number {
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
    
    pub fn exponent(&self) -> u8 {
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

    pub fn quad_buffer(&self) -> [u8; 8] {
        self.quad().to_le_bytes()
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
    /// Only exponents 0, 1, 2, and 3 are supported.
    /// 
    /// TODO: Test
    pub fn from_exponent_selecting(exponent: u8, number: u64) -> Option<Self> {
        Some(match exponent {
            0 => Self::Byte(number as u8),
            1 => Self::Word(number as u16),
            2 => Self::Dual(number as u32),
            3 => Self::Quad(number),
            _ => return None
        })
    }
    
    /// Turn u64 with the size into an instance of data.
    pub fn from_size_selecting(size: &Size, number: u64) -> Self {
        match size {
            Size::Byte => Self::Byte(number as u8),
            Size::Word => Self::Word(number as u16),
            Size::Dual => Self::Dual(number as u32),
            Size::Quad => Self::Quad(number),
        }
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

    pub fn is_zero(&self) -> bool {
        match self {
            Self::Byte(v) => *v == 0,
            Self::Word(v) => *v == 0,
            Self::Dual(v) => *v == 0,
            Self::Quad(v) => *v == 0
        }
    }

    pub fn not_zero(&self) -> bool {
        !self.is_zero()
    }
    
    pub fn resize(&self, new_size: &Size) -> Self {
        match new_size {
            Size::Byte => Self::Byte(u8::from(self)),
            Size::Word => Self::Word(u16::from(self)),
            Size::Dual => Self::Dual(u32::from(self)),
            Size::Quad => Self::Quad(u64::from(self))
        }
    }
}

// region: Converting numbers to data instances
impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Self::Word(value)
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Self::Dual(value)
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Self::Quad(value)
    }
}
// endregion

impl From<&Size> for Number {
    fn from(value: &Size) -> Self {
        match value {
            Size::Byte => Self::Byte(0),
            Size::Word => Self::Word(0),
            Size::Dual => Self::Dual(0),
            Size::Quad => Self::Quad(0)
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.quad() == other.quad()
    }
}

impl ReadAll<[u8]> for Number {
    /// Read bytes of the stored number type into a slice reference.
    /// ```
    /// use atln_processor::number::Data;
    /// use atln_processor::utility::ReadAll;
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
    fn read_all(&self, target: &mut [u8]) -> usize {
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
impl From<&Number> for u8 {
    fn from(value: &Number) -> Self {
        value.quad() as u8
    }
}

impl From<&Number> for u16 {
    fn from(value: &Number) -> Self {
        value.quad() as u16
    }
}

impl From<&Number> for u32 {
    fn from(value: &Number) -> Self {
        value.quad() as u32
    }
}

impl From<&Number> for u64 {
    fn from(value: &Number) -> Self {
        value.quad()
    }
}
// endregion

// region: Checked functions.
pub trait CheckedAdd: Sized {
    fn checked_add(self, factor: &Self) -> Option<Self>;
}

pub trait CheckedSub: Sized {
    fn checked_sub(self, factor: &Self) -> Option<Self>;
}

pub trait CheckedMul: Sized {
    fn checked_mul(self, factor: &Self) -> Option<Self>;
}

pub trait CheckedDiv: Sized {
    fn checked_div(self, factor: &Self) -> Option<Self>;
}

impl CheckedAdd for Number {
    fn checked_add(self, factor: &Self) -> Option<Number> {
        Some(match self {
            Self::Byte(v) => Number::Byte(v.checked_add(u8::from(factor))?),
            Self::Word(v) => Number::Word(v.checked_add(u16::from(factor))?),
            Self::Dual(v) => Number::Dual(v.checked_add(u32::from(factor))?),
            Self::Quad(v) => Number::Quad(v.checked_add(u64::from(factor))?)
        })
    }
}

impl CheckedSub for Number {
    fn checked_sub(self, factor: &Self) -> Option<Number> {
        Some(match self {
            Self::Byte(v) => Number::Byte(v.checked_sub(u8::from(factor))?),
            Self::Word(v) => Number::Word(v.checked_sub(u16::from(factor))?),
            Self::Dual(v) => Number::Dual(v.checked_sub(u32::from(factor))?),
            Self::Quad(v) => Number::Quad(v.checked_sub(u64::from(factor))?)
        })
    }
}

impl CheckedMul for Number {
    fn checked_mul(self, factor: &Self) -> Option<Number> {
        Some(match self {
            Self::Byte(v) => Number::Byte(v.checked_mul(u8::from(factor))?),
            Self::Word(v) => Number::Word(v.checked_mul(u16::from(factor))?),
            Self::Dual(v) => Number::Dual(v.checked_mul(u32::from(factor))?),
            Self::Quad(v) => Number::Quad(v.checked_mul(u64::from(factor))?)
        })
    }
}

impl CheckedDiv for Number {
    fn checked_div(self, factor: &Self) -> Option<Number> {
        Some(match self {
            Self::Byte(v) => Number::Byte(v.checked_div(u8::from(factor))?),
            Self::Word(v) => Number::Word(v.checked_div(u16::from(factor))?),
            Self::Dual(v) => Number::Dual(v.checked_div(u32::from(factor))?),
            Self::Quad(v) => Number::Quad(v.checked_div(u64::from(factor))?)
        })
    }
}
// endregion

// region: Carrying
pub trait CarryingAdd {
    fn carrying_add(&self, factor: &Number, carry: bool) -> Option<(Number, bool)>;
}

pub trait CarryingSub {
    fn carrying_sub(&self, factor: &Number, carry: bool) -> Option<(Number, bool)>;
}

pub trait CarryingMul {
    fn carrying_mul(&self, factor: &Number, carry: bool) -> Option<(Number, bool)>;
}

pub trait CarryingDiv {
    fn carrying_div(&self, factor: &Number, carry: bool) -> Option<(Number, bool)>;
}

impl CarryingAdd for Number {
    fn carrying_add(&self, factor: &Number, carry: bool) -> Option<(Number, bool)> {
        Some(match self {
            Self::Byte(v) => {
                let binding = v.carrying_add(u8::from(factor), carry);
                (Number::Byte(binding.0), binding.1)
            },
            Self::Word(v) => {
                let binding = v.carrying_add(u16::from(factor), carry);
                (Number::Word(binding.0), binding.1)
            },
            Self::Dual(v) => {
                let binding = v.carrying_add(u32::from(factor), carry);
                (Number::Dual(binding.0), binding.1)
            },
            Self::Quad(v) => {
                let binding = v.carrying_add(u64::from(factor), carry);
                (Number::Quad(binding.0), binding.1)
            }
        })
    }
}

impl CarryingSub for Number {
    fn carrying_sub(&self, factor: &Number, carry: bool) -> Option<(Number, bool)> {
        Some(match self {
            Self::Byte(v) => {
                let binding = v.borrowing_sub(u8::from(factor), carry);
                (Number::Byte(binding.0), binding.1)
            },
            Self::Word(v) => {
                let binding = v.borrowing_sub(u16::from(factor), carry);
                (Number::Word(binding.0), binding.1)
            },
            Self::Dual(v) => {
                let binding = v.borrowing_sub(u32::from(factor), carry);
                (Number::Dual(binding.0), binding.1)
            },
            Self::Quad(v) => {
                let binding = v.borrowing_sub(u64::from(factor), carry);
                (Number::Quad(binding.0), binding.1)
            }
        })
    }
}
// endregion

// region: Wrapping calculations.
pub trait WrappingAdd {
    fn wrapping_add(&self, factor: &Self) -> Self;
}

pub trait WrappingSub {
    fn wrapping_sub(&self, factor: &Self) -> Self;
}

pub trait WrappingMul {
    fn wrapping_mul(&self, factor: &Self) -> Self;
}

pub trait WrappingDiv {
    fn wrapping_div(&self, factor: &Self) -> Self;
}

impl WrappingAdd for Number {
    fn wrapping_add(&self, factor: &Self) -> Self {
        match self {
            Self::Byte(v) => Number::Byte(v.wrapping_add(u8::from(factor))),
            Self::Word(v) => Number::Word(v.wrapping_add(u16::from(factor))),
            Self::Dual(v) => Number::Dual(v.wrapping_add(u32::from(factor))),
            Self::Quad(v) => Number::Quad(v.wrapping_add(u64::from(factor)))
        }
    }
}

impl WrappingSub for Number {
    fn wrapping_sub(&self, factor: &Self) -> Self {
        match self {
            Self::Byte(v) => Number::Byte(v.wrapping_sub(u8::from(factor))),
            Self::Word(v) => Number::Word(v.wrapping_sub(u16::from(factor))),
            Self::Dual(v) => Number::Dual(v.wrapping_sub(u32::from(factor))),
            Self::Quad(v) => Number::Quad(v.wrapping_sub(u64::from(factor)))
        }
    }
}

impl WrappingMul for Number {
    fn wrapping_mul(&self, factor: &Self) -> Self {
        match self {
            Self::Byte(v) => Number::Byte(v.wrapping_mul(u8::from(factor))),
            Self::Word(v) => Number::Word(v.wrapping_mul(u16::from(factor))),
            Self::Dual(v) => Number::Dual(v.wrapping_mul(u32::from(factor))),
            Self::Quad(v) => Number::Quad(v.wrapping_mul(u64::from(factor)))
        }
    }
}

impl WrappingDiv for Number {
    fn wrapping_div(&self, factor: &Self) -> Self {
        match self {
            Self::Byte(v) => Number::Byte(v.wrapping_div(u8::from(factor))),
            Self::Word(v) => Number::Word(v.wrapping_div(u16::from(factor))),
            Self::Dual(v) => Number::Dual(v.wrapping_div(u32::from(factor))),
            Self::Quad(v) => Number::Quad(v.wrapping_div(u64::from(factor)))
        }
    }
}
// endregion

// region: Number max const.
pub trait Max {
    const MAX: Self;
}

impl Max for u8 {
    const MAX: Self = Self::MAX as u8;
}

impl Max for u16 {
    const MAX: Self = Self::MAX as u16;
}

impl Max for u32 {
    const MAX: Self = Self::MAX as u32;
}

impl Max for u64 {
    const MAX: Self = Self::MAX as u64;
}
// endregion