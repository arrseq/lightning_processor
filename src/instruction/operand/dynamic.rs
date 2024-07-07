use crate::number::Number;

use super::register::Register;

pub enum Dynamic {
    Register(Register),
    Constant(Number),
    Indexing,
    Address
}