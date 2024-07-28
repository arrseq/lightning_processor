use crate::math::dynamic_number::DynamicNumber;

impl DynamicNumber {
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
}