use crate::instruction_lg::operand::{Address, Calculated, Operand, Requirement};
use crate::instruction_lg::operand::register::{Register, SideInput};
use crate::math::dynamic_number;

#[test]
fn requirement() {
    assert_eq!(Operand::requirement(Operand::Constant(dynamic_number::Unsigned::Word(u16::MAX)).encode()).unwrap(), Requirement::Constant(None));
    assert_eq!(Operand::requirement(Operand::Address(Address::Array(Calculated {
        base: Register::SideInput(SideInput::First),
        offset: dynamic_number::Unsigned::Word(u16::MAX)
    })).encode()).unwrap(), Requirement::RegisterAndConstant(Some(dynamic_number::Size::Word)));
}