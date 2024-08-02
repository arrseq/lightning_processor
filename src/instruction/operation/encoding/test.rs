use crate::{read_cursor, write_cursor};
use crate::instruction::operand::{AddressingMode, ImmediateAddressing, Operand};
use crate::instruction::operation::{Destination, DestinationAndDualInput, Input, Operation, VectorComponent};
use crate::math::dynamic_number::{Size, Unsigned};

#[test]
fn decode() {
    // stack register 0 as byte
    assert_eq!(read_cursor([Operation::STACK.code as u8, 0], Operation::decode).unwrap(), Operation::Input {
        operation: Input::Stack,
        input: Operand {
            mode: AddressingMode::Register { register: 0 },
            size: Size::X8
        }
    });

    // Add register 1 as byte to immediate value 5 as word then Store result in register 0 as a qword.
    assert_eq!(read_cursor([Operation::ADD.code as u8, 0b00110000, 0b00000001, 0b01010000, 5, 0], Operation::decode).unwrap(), Operation::DestinationAndDualInput {
        operation: DestinationAndDualInput::Add,
        destination: Operand { 
            mode: AddressingMode::Register { register: 0 },
            size: Size::X64
        },
        inputs: [
            Operand {
                mode: AddressingMode::Register { register: 1 },
                size: Size::X8
            },
            Operand {
                mode: AddressingMode::Immediate { mode: ImmediateAddressing::Immediate { immediate: Unsigned::new(5) }},
                size: Size::X16
            }
        ]
    });
}

#[test]
fn encode() {
    // none
    assert_eq!(write_cursor(vec![0u8; 1], |cursor| Operation::None.encode(cursor)), [0]);
    // lock modifier
    assert_eq!(write_cursor(vec![0u8; 1], |cursor| Operation::Lock.encode(cursor)), [1]);
    // vec remap
    assert_eq!(write_cursor(vec![0u8; 1], |cursor| Operation::MapVector {
        mappings: [
            None,
            Some(VectorComponent::X3),
            Some(VectorComponent::X2),
            Some(VectorComponent::X1),
        ],
        operand: 1
    }.encode(cursor)), [3, 0b01_000_100, 0b011_010_00]);
}