use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Component {
    A,
    B,
    C,
    D
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DoubleVector<Component> {
    pub component_a: Component,
    pub component_b: Component
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrioVector<Component> {
    pub component_a: Component,
    pub component_b: Component,
    pub component_c: Component
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuartetVector<Component> {
    pub component_a: Component,
    pub component_b: Component,
    pub component_c: Component,
    pub component_d: Component
}

pub type U8DoubleVector = DoubleVector<u8>;
pub type U8TrioVector = TrioVector<u8>;
pub type U8QuartetVector = QuartetVector<u8>;

pub type U16DoubleVector = DoubleVector<u16>;
pub type U16TrioVector = TrioVector<u16>;
pub type U16QuartetVector = QuartetVector<u16>;

pub type U32DoubleVector = DoubleVector<u32>;
pub type U32TrioVector = TrioVector<u32>;
pub type U32QuartetVector = QuartetVector<u32>;

pub type U64DoubleVector = DoubleVector<u64>;
pub type U64TrioVector = TrioVector<u64>;
pub type U64QuartetVector = QuartetVector<u64>;

macro_rules! implement_operation {
    ($operation: path, $function: tt, $operator: tt, $for: ty) => {
        impl $operation for DoubleVector<$for> {
            type Output = Self;

            fn $function(self, other: Self) -> Self::Output {
                Self {
                    component_a: self.component_a $operator other.component_a,
                    component_b: self.component_b $operator other.component_b
                }
            }
        }
    };
}

macro_rules! implement_for_numeric_types {
    ($operation: path, $function: tt, $operator: tt) => {
        implement_operation!($operation, $function, $operator, u8);
        implement_operation!($operation, $function, $operator, u16);
        implement_operation!($operation, $function, $operator, u32);
        implement_operation!($operation, $function, $operator, u64);
        
        implement_operation!($operation, $function, $operator, i8);
        implement_operation!($operation, $function, $operator, i16);
        implement_operation!($operation, $function, $operator, i32);
        implement_operation!($operation, $function, $operator, i64);
        
        implement_operation!($operation, $function, $operator, f32);
        implement_operation!($operation, $function, $operator, f64);
    };
}

implement_for_numeric_types!(Add, add, +);
implement_for_numeric_types!(Sub, sub, -);
implement_for_numeric_types!(Mul, mul, *);
implement_for_numeric_types!(Div, div, /);