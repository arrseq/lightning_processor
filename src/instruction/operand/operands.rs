#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operands {
    pub destination: Option<Operand>,
    pub operands: [Option<Operand>; 3]
}