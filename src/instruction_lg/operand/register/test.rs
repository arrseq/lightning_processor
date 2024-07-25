use crate::instruction_lg::operand::register::{GeneralPurpose, Register};

#[test]
fn encode() {
    assert_eq!(Register::GENERAL_PURPOSE_8, Register::GeneralPurpose(GeneralPurpose::G8).encode());
    assert_eq!(Register::GENERAL_PURPOSE_4, Register::GeneralPurpose(GeneralPurpose::G4).encode());
}

#[test]
fn decode() {
    assert_eq!(Register::decode(Register::GENERAL_PURPOSE_8).unwrap(), Register::GeneralPurpose(GeneralPurpose::G8));
    assert_eq!(Register::decode(Register::GENERAL_PURPOSE_4).unwrap(), Register::GeneralPurpose(GeneralPurpose::G4));
}