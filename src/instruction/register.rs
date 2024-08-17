use crate::instruction::vector;
use crate::num::MaskedU8;

pub type Code = MaskedU8<0x1f>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileName {
    General,
    Vector,
    System,
    Interrupt
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct File {
    pub general: [u64; Code::MASK as usize],
    pub vector: [[u64; vector::SIZE]; Code::MASK as usize],
    pub system: [u64; Code::MASK as usize],
    pub interrupt: [u64; Code::MASK as usize],
}