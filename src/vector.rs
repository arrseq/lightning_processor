#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Component {
    A,
    B,
    C,
    D
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct N2ComponentVector<Component> {
    pub a: Component,
    pub b: Component
}

pub type N2U8ComponentVector = N2ComponentVector<u8>;
pub type N2U16ComponentVector = N2ComponentVector<u16>;
pub type N2U32ComponentVector = N2ComponentVector<u32>;
pub type N2U64ComponentVector = N2ComponentVector<u64>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct N3ComponentVector<Component> {
    pub a: Component,
    pub b: Component,
    pub c: Component
}

pub type N3U8ComponentVector = N3ComponentVector<u8>;
pub type N3U16ComponentVector = N3ComponentVector<u16>;
pub type N3U32ComponentVector = N3ComponentVector<u32>;
pub type N3U64ComponentVector = N3ComponentVector<u64>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct N4ComponentVector<Component> {
    pub a: Component,
    pub b: Component,
    pub c: Component,
    pub d: Component
}

pub type N4U8ComponentVector = N4ComponentVector<u8>;
pub type N4U16ComponentVector = N4ComponentVector<u16>;
pub type N4U32ComponentVector = N4ComponentVector<u32>;
pub type N4U64ComponentVector = N4ComponentVector<u64>;