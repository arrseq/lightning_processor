macro_rules! impl_mask {
    ($ty: ty, $name: tt) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name<const MASK: $ty>($ty);
        
        impl<const MASK: $ty> $name<MASK> {
            pub const fn new(value: $ty) -> Self { Self(value & MASK) }
            pub const fn get(self) -> $ty { self.0 }
            pub fn set(&mut self, value: $ty) { self.0 = value & MASK }
        }
    };
}

impl_mask!(u8, MaskedU8);
impl_mask!(u16, MaskedU16);
impl_mask!(u32, MaskedU32);
impl_mask!(u64, MaskedU64);
impl_mask!(i8, MaskedI8);
impl_mask!(i16, MaskedI16);
impl_mask!(i32, MaskedI32);
impl_mask!(i64, MaskedI64);