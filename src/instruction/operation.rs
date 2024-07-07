use instruction::operation::basic::Basic;
use instruction::operation::floating::Floating;

pub mod basic;
pub mod floating;

#[derive(Debug)]
pub enum Operation {
    Basic(Basic),
    Floating(Floating)
}

#[derive(Debug)]
pub enum Size {
    Byte,
    Word
}