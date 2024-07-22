#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Remap {
    
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vector {
    Remap,
    Normalize,
    Round,
    Floor,
    Ceil
}