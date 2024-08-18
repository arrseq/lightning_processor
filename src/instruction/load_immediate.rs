use crate::num::MaskedU32;

pub type Immediate = MaskedU32<0x1FFFFF>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Segment {
    Segment0(Immediate),
    Segment1(Immediate),
    Segment2(Immediate),
    Segment3(bool)
}