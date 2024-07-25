use crate::math::vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Remap {
    pub component_a_source: vector::Component,
    pub component_b_source: vector::Component,
    pub component_c_source: vector::Component,
    pub component_d_source: vector::Component
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vector {
    Remap(Remap),
    Normalize,
    Round,
    Floor,
    Ceil
}

// TODO