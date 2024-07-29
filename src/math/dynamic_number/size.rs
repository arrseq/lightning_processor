use crate::math::dynamic_number::{Unsigned, Size};

impl Size {
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

impl Unsigned {
    pub fn with_size_u64(size: Size, value: u64) -> Self {
        match size {
            Size::U8 => Self::U8(value as u8),
            Size::U16 => Self::U16(value as u16),
            Size::U32 => Self::U32(value as u32),
            Size::U64 => Self::U64(value)
        }
    }
    
    pub fn with_size(size: Size, value: Self) -> Self {
        match size {
            Size::U8 => Self::U8(u8::from(value)),
            Size::U16 => Self::U16(u16::from(value)),
            Size::U32 => Self::U32(u32::from(value)),
            Size::U64 => Self::U64(u64::from(value))
        }
    }

    /// Increase the size of the current number quantization until not possible.
    pub fn upsize(self) -> Self {
        let new_size = Size::from(self)
            .upsize();
        Self::with_size(new_size, self)
    }

    /// Decrease the size of the current number quantization until not possible.
    ///
    /// # Warning
    /// This can result in some information being lost.
    pub fn downsize(self) -> Self {
        let new_size = Size::from(self)
            .downsize();
        Self::with_size(new_size, self)
    }
}

// TODO: Copy methods for signed