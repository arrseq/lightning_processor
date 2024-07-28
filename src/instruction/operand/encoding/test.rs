use std::io::Cursor;
use crate::instruction::operand::{ConstantMode, EncodedModes, Mode, Operand, RegisterMode, SecondMode};
use crate::instruction::operand::encoding::encode_first_mode_byte;
use crate::math::dynamic_number::{Unsigned, Size};

#[test]
fn encode_mode() {
    assert_eq!(Mode::Register { mode: RegisterMode::Register, register: 2 }.encode_mode(), EncodedModes(Mode::REGISTER_MODE, None));
    assert_eq!(Mode::Second {
        mode: SecondMode::ConstantBased { mode: ConstantMode::Relative, constant: Unsigned::U8(2) },
        base_register: 4,
        index_register: 8
    }.encode_mode(), EncodedModes(Mode::SECOND_MODE, Some(Mode::RELATIVE_SECOND_MODE)));
}

#[test]
fn test_encode_first_mode_byte() {
    assert_eq!(encode_first_mode_byte(5, 3, Size::U16), 0b0101_11_01);
    assert_eq!(encode_first_mode_byte(15, 2, Size::U8), 0b1111_10_00);
}

#[test]
fn encode() {
    {
        let mut output = Cursor::new(vec![0u8; 0]);
        let operand = Operand {
            mode: Mode::Constant { constant: 5 },
            data_size: Size::U16
        };

        operand.encode(&mut output).unwrap();
        assert_eq!(output.get_ref().as_slice(), &[0b00001001]);
    }

    {
        let mut output = Cursor::new(vec![0u8; 0]);
        let operand = Operand {
            mode: Mode::Register { mode: RegisterMode::Register, register: 10 },
            data_size: Size::U16
        };

        operand.encode(&mut output).unwrap();
        assert_eq!(output.get_ref().as_slice(), &[0b10100001]);
    }
} 