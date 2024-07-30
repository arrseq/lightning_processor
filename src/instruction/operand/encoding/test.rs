use crate::cursor_test;
use crate::instruction::operand::{AddressingMode, ArrayAddressing, BaseAddressing, ComplexAddressing, ImmediateAddressing, Operand};
use crate::math::dynamic_number::{Signed, Size, Unsigned};

#[test]
fn decode_register() {
    // register 0 as qword
    assert_eq!(cursor_test([ 0b00_110000 ], Operand::decode).unwrap(), Operand {
        size: Size::U64,
        mode: AddressingMode::Register { register: 0 }
    });

    // register 10 as dword
    assert_eq!(cursor_test([ 0b00_101010 ], Operand::decode).unwrap(), Operand {
        size: Size::U32,
        mode: AddressingMode::Register { register: 10 }
    });
}

#[test]
fn decode_relative_immediate() {
    // +1 int_1 offset with a qword value.
    assert_eq!(cursor_test([ 0b10_110000, 0b00000001 ], Operand::decode).unwrap(), Operand {
        size: Size::U64,
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Relative {
            offset: Signed::I8(1)
        }}
    });

    // +0 int_2 offset with a qword value.
    assert_eq!(cursor_test([ 0b10_110100, 0b00000000, 0b00000000 ], Operand::decode).unwrap(), Operand {
        size: Size::U64,
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Relative {
            offset: Signed::I16(0)
        }}
    });
}

#[test]
fn decode_value_immediate() {
    // 10 uint_1 as a qword value.
    assert_eq!(cursor_test([ 0b01_110000, 0b00001010 ], Operand::decode).unwrap(), Operand {
        size: Size::U64,
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Immediate {
            immediate: Unsigned::U8(10)
        }}
    });

    // 10 uint_8 as a word value.
    assert_eq!(cursor_test([ 0b01_001100, 0b00001010, 0, 0, 0, 0, 0, 0, 0 ], Operand::decode).unwrap(), Operand {
        size: Size::U8,
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Immediate {
            immediate: Unsigned::U64(10)
        }}
    });
}

#[test]
fn decode_base() {
    // register 10 as base as a byte value.
    assert_eq!(cursor_test([ 0b11_00_1010, 0b00_0000_00 ], Operand::decode).unwrap(), Operand {
        size: Size::U8,
        mode: AddressingMode::Complex {
            mode: ComplexAddressing::Base { mode: BaseAddressing::Base },
            base: 10
        }
    });

    // register 15 with word offset of 3 as base as a qword value.
    assert_eq!(cursor_test([ 0b11_11_1111, 0b01_0000_01, 0b00000011, 0 ], Operand::decode).unwrap(), Operand {
        size: Size::U64,
        mode: AddressingMode::Complex {
            mode: ComplexAddressing::Base { mode: BaseAddressing::Offsetted { offset: Unsigned::U16(3) } },
            base: 15
        }
    });
}

#[test]
fn decode_array() {
    // base register 3 and index register 10 as a dword value.
    assert_eq!(cursor_test([ 0b11_10_0011, 0b10_1010_00 ], Operand::decode).unwrap(), Operand {
        size: Size::U32,
        mode: AddressingMode::Complex {
            mode: ComplexAddressing::ArrayAddressing { mode: ArrayAddressing::Array, index: 10 },
            base: 3
        }
    });

    // base register 3, index register 10, and with offset uint_1 255 as a dword value.
    assert_eq!(cursor_test([ 0b11_10_0011, 0b11_1010_00, u8::MAX ], Operand::decode).unwrap(), Operand {
        size: Size::U32,
        mode: AddressingMode::Complex {
            mode: ComplexAddressing::ArrayAddressing {
                mode: ArrayAddressing::Offsetted { offset: Unsigned::U8(255) },
                index: 10
            },
            base: 3
        }
    });
}