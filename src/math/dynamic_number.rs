use std::io;
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DynamicNumber {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64)
}

#[derive(Debug, Error)]
pub enum DecodeUnprefixedError {
    #[error("Overflow occurred when adding to summation buffer")]
    Overflow,
    #[error("Failed to read next byte")]
    Io(#[source] io::Error)
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
    
    pub fn decode_unprefixed(input: &mut impl Read) -> Result<Self, DecodeUnprefixedError> {
        /// # Result
        /// Tuple containing whether a next byte should be read and the value this byte evaluates to.
        fn evaluate(byte: u8) -> (bool, u8) {
            if byte == 255 { (true, 254) } else { (false, byte) }
        }
        
        let mut result = DynamicNumber::U8(0);
        let mut buffer = [0u8; 1];
        
        loop {
            input.read_exact(&mut buffer).map_err(DecodeUnprefixedError::Io)?;
            let (read_next, offset) = evaluate(buffer[0]);
            if !result.upsizing_add(DynamicNumber::U8(offset)) { return Err(DecodeUnprefixedError::Overflow); }
            
            if !read_next { break }
        }
        
        Ok(result)
    }
    
    /// If an overflow will happen then the increment will not happen.
    /// 
    /// # Result
    /// Returns true if an overflow did not happen.
    pub fn checked_increment(&mut self) -> bool {
        match self {
            Self::U8(value) => if *value == u8::MAX { return false; } else { *value += 1 },
            Self::U16(value) => if *value == u16::MAX { return false; } else { *value += 1 },
            Self::U32(value) => if *value == u32::MAX { return false; } else { *value += 1 },
            Self::U64(value) => if *value == u64::MAX { return false; } else { *value += 1 }
        }
        
        true
    }

    /// If an overflow will happen then the addition will not happen. The other value is size cast to this instance of
    /// Self.
    ///
    /// # Result
    /// Returns true if an overflow did not happen.
    pub fn checked_add(&mut self, other: Self) -> bool {
        match self {
            Self::U8(value) => match value.checked_add(u8::from(other)) {
                Some(new_value) => *value = new_value,
                None => return false
            },
            Self::U16(value) => match value.checked_add(u16::from(other)) {
                Some(new_value) => *value = new_value,
                None => return false
            },
            Self::U32(value) => match value.checked_add(u32::from(other)) {
                Some(new_value) => *value = new_value,
                None => return false
            },
            Self::U64(value) => match value.checked_add(u64::from(other)) {
                Some(new_value) => *value = new_value,
                None => return false
            }
        }

        true
    }
    
    pub fn upsize(self) -> Self {
        match self {
            Self::U8(value) => Self::U16(value as u16),
            Self::U16(value) => Self::U32(value as u32),
            Self::U32(value) => Self::U64(value as u64),
            Self::U64(value) => Self::U64(value),
        }
    }
    
    // TODO: Implement downsize()
    
    /// Increment and upsize when necessary. If an overflow will happen then the increment and upsizing will not happen.
    /// 
    /// # Result
    /// Returns true if an overflow did not happen.
    pub fn upsizing_increment(&mut self) -> bool {
        let success = self.checked_increment();
        if success { return true };

        let mut new_self = self.upsize();
        let success = new_self.checked_increment();
        
        if !success { return false };
        *self = new_self;
        
        true
    }

    /// Add and upsize when necessary. If an overflow will happen then the increment and upsizing will not happen.
    ///
    /// # Result
    /// Returns true if an overflow did not happen.
    pub fn upsizing_add(&mut self, other: Self) -> bool {
        let success = self.checked_add(other);
        if success { return true };

        let mut new_self = self.upsize();
        let success = new_self.checked_add(other);

        if !success { return false };
        *self = new_self;

        true
    }
}

impl From<DynamicNumber> for u8 {
    fn from(value: DynamicNumber) -> Self {
        match value {
            DynamicNumber::U8(value) => value,
            DynamicNumber::U16(value) => value as u8,
            DynamicNumber::U32(value) => value as u8,
            DynamicNumber::U64(value) => value as u8,
        }
    }
}

impl From<DynamicNumber> for u16 {
    fn from(value: DynamicNumber) -> Self {
        match value {
            DynamicNumber::U8(value) => value as u16,
            DynamicNumber::U16(value) => value,
            DynamicNumber::U32(value) => value as u16,
            DynamicNumber::U64(value) => value as u16,
        }
    }
}

impl From<DynamicNumber> for u32 {
    fn from(value: DynamicNumber) -> Self {
        match value {
            DynamicNumber::U8(value) => value as u32,
            DynamicNumber::U16(value) => value as u32,
            DynamicNumber::U32(value) => value,
            DynamicNumber::U64(value) => value as u32,
        }
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
        !matches!(self, Self::U64)
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
        !matches!(self, Self::U8)
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