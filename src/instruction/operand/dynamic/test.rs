use crate::math::dynamic_number;
use crate::instruction::operand::dynamic::{Address, Calculated, Operand, Requirement};
use crate::instruction::operand::register::{Register, SideInput};

#[test]
fn requirement() {
    assert_eq!(Operand::requirement(Operand::Constant(dynamic_number::Unsigned::Word(u16::MAX)).encode()).unwrap(), Requirement::Constant(None));
    assert_eq!(Operand::requirement(Operand::Address(Address::Array(Calculated {
        base: Register::SideInput(SideInput::First),
        offset: dynamic_number::Unsigned::Word(u16::MAX)
    })).encode()).unwrap(), Requirement::RegisterAndConstant(Some(dynamic_number::Size::Word)));
}