#[derive(Debug, Clone, Copy)]
pub enum HighSize {
    Dual,
    Quad
}

#[derive(Debug, Clone, Copy)]
pub enum HighNumber {
    Dual(u32),
    Quad(u64)
}

impl From<&HighNumber> for HighSize {
    fn from(value: &HighNumber) -> Self {
        match value {
            HighNumber::Dual(_) => Self::Dual,
            HighNumber::Quad(_) => Self::Quad
        }
    }
}