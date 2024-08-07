#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flag {
    Negative,
    Zero,
    Overflow,
    Regrouping,
    Parity,
    Auxiliary
}