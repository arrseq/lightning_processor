use instruction::operand::SizedOperand;
use number::high::HighNumber;
use crate::number::Number;

use super::register::Register;

impl From<HighNumber> for u64 {
    fn from(value: HighNumber) -> Self {
        match value { 
            HighNumber::Dual(v) => v as u64, 
            HighNumber::Quad(v) => v 
        }
    } 
}

#[derive(Debug, Clone, Copy)]
pub struct Added {
    pub constant: HighNumber,
    pub offset: Register
}

#[derive(Debug, Clone, Copy)]
pub enum Address {
    Constant(HighNumber),
    Register(Register),
    Added(Added)
}

#[derive(Debug, Clone, Copy)]
pub enum Dynamic {
    Register(Register),
    Constant(Number),
    Address(Address)
}

pub type SizedDynamic = SizedOperand<Dynamic>;