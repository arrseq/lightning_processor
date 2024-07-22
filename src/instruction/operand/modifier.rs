pub mod vector;

use vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modifier {
    Vector(Vector)
}

impl Modifier {
    pub const REMAP_VECTOR: u8 = 0;
    pub const NORMALIZE_VECTOR: u8 = 1;
    pub const ROUND_VECTOR: u8 = 2;
    pub const FLOOR_VECTOR: u8 = 3;
    pub const CEIL_VECTOR: u8 = 4;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Modifiers {
    
}