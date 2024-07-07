use instruction::operation::basic::Basic;
use instruction::operation::floating::Floating;

pub mod basic;
pub mod floating;

pub enum Operation {
    Basic(Basic),
    Floating(Floating)
}

pub enum FlatOperation {
    BasicAdd,
    BasicCarryingAdd,
    BasicSubtract,
    BasicBorrowingSubtract,
    
    FloatingAdd,
    FloatingSubtract
}