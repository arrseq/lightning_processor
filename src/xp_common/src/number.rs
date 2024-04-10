pub trait SignedNumber {}
impl SignedNumber for i8 {}
impl SignedNumber for i16 {}
impl SignedNumber for i32 {}
impl SignedNumber for i64 {}

pub trait UnsignedNumber {}
impl UnsignedNumber for u8 {}
impl UnsignedNumber for u16 {}
impl UnsignedNumber for u32 {}
impl UnsignedNumber for u64 {}