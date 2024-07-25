#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Copy,
    Add,
    CarryingAdd,
    Subtract,
    BorrowingSubtract,
    Multiply,
    Divide
}
