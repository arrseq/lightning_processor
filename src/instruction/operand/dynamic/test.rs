use crate::dynamic_number;
use crate::instruction::operand::dynamic::{Address, Calculated, Dynamic, Requirement};
use crate::instruction::operand::register::{Register, SideInput};

#[test]
fn requirement() {
    assert_eq!(Dynamic::requirement(Dynamic::Constant(dynamic_number::Unsigned::Word(u16::MAX)).encode()).unwrap(), Requirement::Constant(None));
    assert_eq!(Dynamic::requirement(Dynamic::Address(Address::Add(Calculated {
        base: Register::SideInput(SideInput::First),
        offset: dynamic_number::Unsigned::Word(u16::MAX)
    })).encode()).unwrap(), Requirement::RegisterAndConstant(Some(dynamic_number::Size::Word)));
}