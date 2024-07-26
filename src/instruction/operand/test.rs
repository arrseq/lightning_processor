use crate::instruction::operand::{ConstantMode, EncodedModes, Mode, RegisterMode, SecondMode};

#[test]
fn encode_mode() {
    assert_eq!(Mode::Register { mode: RegisterMode::Register, register: 2 }.encode_mode(), EncodedModes(Mode::REGISTER_MODE, None));
    assert_eq!(Mode::Second { 
        mode: SecondMode::ConstantBased { mode: ConstantMode::Relative, constant: 2, mask: u8::MAX as u64 },
        base_register: 4,
        index_register: 8
    }.encode_mode(), EncodedModes(Mode::SECOND_MODE, Some(Mode::RELATIVE_SECOND_MODE)));
}