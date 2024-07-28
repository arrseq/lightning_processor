use std::io;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DynamicNumber {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64)
}

impl DynamicNumber {
    pub fn with_size(size: Size, value: u64) -> Self {
        match size {
            Size::U8 => Self::U8(value as u8),
            Size::U16 => Self::U16(value as u16),
            Size::U32 => Self::U32(value as u32),
            Size::U64 => Self::U64(value)
        }
    }
    
    pub fn decode_unprefixed(input: &mut impl Read) -> io::Result<Self> {
        let mut buffer = 0u64;
    }
}

impl From<DynamicNumber> for u64 {
    fn from(value: DynamicNumber) -> Self {
        match value {
            DynamicNumber::U8(value) => value as u64,
            DynamicNumber::U16(value) => value as u64,
            DynamicNumber::U32(value) => value as u64,
            DynamicNumber::U64(value) => value
        }
    }
}

/// # Power
/// The power is a representation of this primitive data type which when set to the power of 2 gives the size in bytes.
/// The power only has its 2 least significant bits used and the rest are discarded.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    U8,
    U16,
    U32,
    U64
}

impl Size {
    /// Construct this enum from an exponent of the power of 2.
    pub fn from_power(size: u8) -> Self {
        match size & 0b000000_11 {
            0 => Self::U8,
            1 => Self::U16,
            2 => Self::U32,
            3 => Self::U64,
            _ => unreachable!()
        }
    }

    /// Convert this enum representation to a power of 2.
    pub fn to_power(self) -> u8 {
        match self {
            Self::U8 => 0,
            Self::U16 => 1,
            Self::U32 => 2,
            Self::U64 => 3
        }
    }
    
    /// Get the number of bytes it would take to represent a value of this size.
    pub fn size(self) -> u8 {
        let size = match self {
            Self::U8 => u8::BITS / 8,
            Self::U16 => u16::BITS / 8,
            Self::U32 => u32::BITS / 8,
            Self::U64 => u64::BITS / 8
        };
        
        size as u8
    }
    
    /// Attempt to increase the size to the next quantization. 
    /// 
    /// # Result
    /// If this cannot be upsized anymore, then self is returned.
    pub fn upsize(self) -> Self {
        match self {
            Self::U8 => Self::U16,
            Self::U16 => Self::U32,
            Self::U32 => Self::U64,
            Self::U64 => Self::U64
        }
    }
    
    /// Whether if upsizing will return a different value.
    pub fn can_upsize(self) -> bool {
        match self {
            Self::U64 => false,
            _ => true
        }
    }

    /// Attempt to decrease the size to the next quantization. 
    ///
    /// # Result
    /// If this cannot be downsized anymore, then self is returned.
    pub fn downsize(self) -> Self {
        match self {
            Self::U64 => Self::U32,
            Self::U32 => Self::U16,
            Self::U16 => Self::U8,
            Self::U8 => Self::U8
        }
    }

    /// Whether if downsizing will return a different value.
    pub fn can_downsize(self) -> bool {
        match self {
            Self::U8 => false,
            _ => true
        }
    }
}

impl From<DynamicNumber> for Size {
    fn from(value: DynamicNumber) -> Self {
        match value {
            DynamicNumber::U8(_) => Self::U8,
            DynamicNumber::U16(_) => Self::U16,
            DynamicNumber::U32(_) => Self::U32,
            DynamicNumber::U64(_) => Self::U64
        }
    }
}