use instruction::operand;

#[derive(Debug)]
pub enum Floating {
    Add(operand::Dual),
    Subtract(operand::Dual)
}