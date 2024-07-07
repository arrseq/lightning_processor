use crate::number::Number;

use super::register::Register;

#[derive(Debug)]
pub enum Constant {
    Dual(u32),
    Quad(u64)
}

impl From<Constant> for u64 {
    fn from(value: Constant) -> Self {
        match value { 
            Constant::Dual(v) => v as u64, 
            Constant::Quad(v) => v 
        }
    } 
}

#[derive(Debug)]
pub struct Added {
    pub constant: Constant,
    pub offset: Register
}

#[derive(Debug)]
pub enum Address {
    Constant(Constant),
    Register(Register),
    Added(Added)
}

#[derive(Debug)]
pub enum Dynamic {
    Register(Register),
    Constant(Number),
    Address(Address)
}
