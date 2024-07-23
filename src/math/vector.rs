use std::ops::Add;

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