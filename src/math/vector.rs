#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4<T>(pub [T; 4]);

impl<T> Vector4<T> {
    pub const SIZE: usize = 4;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3<T>(pub [T; 3]);

impl<T> Vector3<T> {
    pub const SIZE: usize = 3;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2<T>(pub [T; 2]);

impl<T> Vector2<T> {
    pub const SIZE: usize = 2;
}

macro_rules! implement_for_vector {
    ($ty: ty, $ident4: ident, $ident3: ident, $ident2: ident) => {
        pub type $ident4 = Vector4<$ty>;
        pub type $ident3 = Vector3<$ty>;
        pub type $ident2 = Vector2<$ty>;
    };
}

implement_for_vector!(u8, U8Vector4, U8Vector3, U8Vector2);
implement_for_vector!(u16, U16Vector4, U16Vector3, U16Vector2);
implement_for_vector!(u32, U32Vector4, U32Vector3, U32Vector2);
implement_for_vector!(u64, U64Vector4, U64Vector3, U64Vector2);