use crate::{read_cursor, write_cursor};
use crate::instruction::operand::{Operand, ArrayAddressing, BaseAddressing, ComplexAddressing, ImmediateAddressing, AddressingMode};
use crate::math::dynamic_number::{Signed, Size, Unsigned};

#[test]
fn decode_register() {
    // register 0 as qword
    assert_eq!(read_cursor([ 0b00_11_0000 ], Operand::decode).unwrap(), Operand {
        size: Size::X64,
        mode: AddressingMode::Register { register: 0 }
    });

    // register 10 as dword
    assert_eq!(read_cursor([ 0b00_10_1010 ], Operand::decode).unwrap(), Operand {
        size: Size::X32,
        mode: AddressingMode::Register { register: 10 }
    });
}

#[test]
fn encode_register() {
    // register 10 as dword
    assert_eq!(write_cursor(vec![0u8; 0], |cursor| Operand {
        mode: AddressingMode::Register { register: 10 },
        size: Size::X32
    }.encode(cursor)), [0b00_10_1010]);

    // immediate 5 as word
    assert_eq!(write_cursor(vec![0u8; 0], |cursor| Operand {
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Immediate { immediate: Unsigned::new(5) }},
        size: Size::X16
    }.encode(cursor)), [0b01_01_0000, 5]);
}

#[test]
fn decode_value_immediate() {
    // 10 uint_1 as a qword value.
    assert_eq!(read_cursor([ 0b01_11_0000, 0b00001010 ], Operand::decode).unwrap(), Operand {
        size: Size::X64,
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Immediate {
            immediate: Unsigned {
                value: 10,
                size: Size::X8
            }
        }}
    });

    // 10 uint_8 as a word value.
    assert_eq!(read_cursor([ 0b01_00_1100, 0b00001010, 0, 0, 0, 0, 0, 0, 0 ], Operand::decode).unwrap(), Operand {
        size: Size::X8,
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Immediate {
            immediate: Unsigned {
                value: 10,
                size: Size::X64
            }
        }}
    });
}

#[test]
fn encode_value_immediate() {
    // u16::MAX uint_2 as a byte value.
    assert_eq!(write_cursor(vec![0u8; 0], |cursor| Operand {
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Immediate { immediate: Unsigned::new(u16::MAX as u64) }},
        size: Size::X8
    }.encode(cursor)), [ 0b01_00_0001, 255, 255 ]);

    // u8::MAX uint_2 as a qword value.
    assert_eq!(write_cursor(vec![0u8; 0], |cursor| Operand {
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Immediate { immediate: Unsigned {
            value: u8::MAX as u64,
            size: Size::X16
        }}},
        size: Size::X64
    }.encode(cursor)), [ 0b01_11_0001, 255, 0 ]);
}

#[test]
fn decode_relative_immediate() {
    // +1 int_1 offset with a qword value.
    assert_eq!(read_cursor([ 0b10_11_0000, 0b00000001 ], Operand::decode).unwrap(), Operand {
        size: Size::X64,
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Relative {
            offset: Signed {
                value: 1,
                size: Size::X8
            }
        }}
    });

    // +0 int_2 offset with a qword value.
    assert_eq!(read_cursor([ 0b10_11_0100, 0b00000000, 0b00000000 ], Operand::decode).unwrap(), Operand {
        size: Size::X64,
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Relative {
            offset: Signed {
                value: 0,
                size: Size::X16
            }
        }}
    });
}

#[test]
fn encode_relative_immediate() {
    // -10 int_1 offset with dword value.
    assert_eq!(write_cursor(vec![0u8; 0], |cursor| Operand {
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Relative { offset: Signed::new(-10) }},
        size: Size::X32
    }.encode(cursor)), [ 0b10_10_0000, -10i8 as u8 ]);

    // 5 int_2 offset with word value.
    assert_eq!(write_cursor(vec![0u8; 0], |cursor| Operand {
        mode: AddressingMode::Immediate { mode: ImmediateAddressing::Relative { offset: Signed {
            value: 5,
            size: Size::X16
        }}},
        size: Size::X16
    }.encode(cursor)), [ 0b10_01_0001, 5i8 as u8, 0 ])
}

#[test]
fn decode_base() {
    // register 10 as base as a byte value.
    assert_eq!(read_cursor([ 0b11_00_1010, 0 ], Operand::decode).unwrap(), Operand {
        size: Size::X8,
        mode: AddressingMode::Complex {
            mode: ComplexAddressing::Base { mode: BaseAddressing::Base },
            base: 10
        }
    });

    // register 15 with word offset of 3 as base as a qword value.
    assert_eq!(read_cursor([ 0b11_11_1111, 0b01_0000_01, 0b00000011, 0 ], Operand::decode).unwrap(), Operand {
        size: Size::X64,
        mode: AddressingMode::Complex {
            mode: ComplexAddressing::Base { mode: BaseAddressing::Offsetted { offset: Unsigned {
                value: 3,
                size: Size::X16
            }}},
            base: 15
        }
    });
}

#[test]
fn encode_base() {
    // register 5 as base as qword value.
    assert_eq!(write_cursor(vec![0u8; 0], |cursor| Operand {
        mode: AddressingMode::Complex { 
            mode: ComplexAddressing::Base { mode: BaseAddressing::Base },
            base: 5
        },
        size: Size::X64
    }.encode(cursor)), [0b11_11_0101, 0]);

    // register 15 as base with byte offset of 1 as dword value.
    assert_eq!(write_cursor(vec![0u8; 0], |cursor| Operand {
        mode: AddressingMode::Complex {
            mode: ComplexAddressing::Base { mode: BaseAddressing::Offsetted { offset: Unsigned::new(1) }},
            base: 15
        },
        size: Size::X32
    }.encode(cursor)), [0b11_10_1111, 0b01_0000_00, 1]);
}

#[test]
fn decode_array() {
    // base register 3 and index register 10 as a dword value.
    assert_eq!(read_cursor([ 0b11_10_0011, 0b10_1010_00 ], Operand::decode).unwrap(), Operand {
        size: Size::X32,
        mode: AddressingMode::Complex {
            mode: ComplexAddressing::ArrayAddressing { mode: ArrayAddressing::Array, index: 10 },
            base: 3
        }
    });

    // base register 3, index register 10, and with offset uint_1 255 as a dword value.
    assert_eq!(read_cursor([ 0b11_10_0011, 0b11_1010_00, u8::MAX ], Operand::decode).unwrap(), Operand {
        size: Size::X32,
        mode: AddressingMode::Complex {
            mode: ComplexAddressing::ArrayAddressing {
                mode: ArrayAddressing::Offsetted { offset: Unsigned::new(255) },
                index: 10
            },
            base: 3
        }
    });
}

#[test]
fn encode_array() {
    // base register 5 and index 15 as qword value.
    assert_eq!(write_cursor(vec![0u8; 0], |cursor| Operand {
        mode: AddressingMode::Complex { 
            mode: ComplexAddressing::ArrayAddressing {
                mode: ArrayAddressing::Array,
                index: 15
            },
            base: 5
        },
        size: Size::X64
    }.encode(cursor)), [0b11_11_0101, 0b10_1111_00]);

    // base register 3 and index 5 with word offset of 10 as word value.
    assert_eq!(write_cursor(vec![0u8; 0], |cursor| Operand {
        mode: AddressingMode::Complex {
            mode: ComplexAddressing::ArrayAddressing {
                mode: ArrayAddressing::Offsetted { offset: Unsigned {
                    value: 10,
                    size: Size::X16
                }},
                index: 5
            },
            base: 3
        },
        size: Size::X16
    }.encode(cursor)), [0b11_01_0011, 0b11_0101_01, 10, 0]);
}