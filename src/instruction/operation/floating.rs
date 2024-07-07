use instruction::operand;

pub enum Floating {
    Add(operand::Dual),
    Subtract(operand::Dual)
}