use crate::instruction::{address, Format, Instruction, OperandCode, operation, RegisterCode, SegmentCode, vector};
use crate::instruction::address::Address;
use crate::instruction::vector::ComponentCode;
use crate::num::{MaskedU32, MaskedU8};

pub const OPERATION_MASK: u32 = 0x0000007F;

pub const LOAD_IMMEDIATE_DESTINATION_FIELD                : u32        =      0b0_00000000_00000000_00001111;
pub const LOAD_IMMEDIATE_SEGMENT_FIELD                    : (u32, u32) = (4,  0b0_00000000_00000000_00110000);
pub const LOAD_IMMEDIATE_IMMEDIATE_FIELD                  : (u32, u32) = (6,  0b0_00111111_11111111_11000000);
pub const DECODE_VECTOR_DESTINATION_FIELD                 : u32        =      0b0_00000000_00000000_00001111;
pub const ENABLE_0_FIELD                                  : (u32, u32) = (4,  0b0_00000000_00000000_00010000);
pub const ENABLE_1_FIELD                                  : (u32, u32) = (5,  0b0_00000000_00000000_00100000);
pub const ENABLE_2_FIELD                                  : (u32, u32) = (6,  0b0_00000000_00000000_01000000);
pub const ENABLE_3_FIELD                                  : (u32, u32) = (7,  0b0_00000000_00000000_10000000);
pub const DECODE_VECTOR_COMPONENT_0_FIELD                 : (u32, u32) = (8,  0b0_00000000_00001111_00000000);
pub const DECODE_VECTOR_COMPONENT_1_FIELD                 : (u32, u32) = (12, 0b0_00000000_11110000_00000000);
pub const DECODE_VECTOR_COMPONENT_2_FIELD                 : (u32, u32) = (16, 0b0_00001111_00000000_00000000);
pub const DECODE_VECTOR_COMPONENT_3_FIELD                 : (u32, u32) = (20, 0b0_11110000_00000000_00000000);
pub const MAP_VECTOR_TEMPORARY_FIELD                      : u32        =      0b0_00000000_00000000_00000001;
pub const MAP_VECTOR_OPERAND_FIELD                        : (u32, u32) = (1,  0b0_00000000_00000000_00000110);
pub const MAP_VECTOR_COMPONENT_0_FIELD                    : (u32, u32) = (3,  0b0_00000000_00000000_00011000);
pub const MAP_VECTOR_COMPONENT_1_FIELD                    : (u32, u32) = (5,  0b0_00000000_00000000_01100000);
pub const MAP_VECTOR_COMPONENT_2_FIELD                    : (u32, u32) = (7,  0b0_00000000_00000001_10000000);
pub const MAP_VECTOR_COMPONENT_3_FIELD                    : (u32, u32) = (9,  0b0_00000000_00000110_00000000);

pub const ADDRESS_MODE_FIELD                              : (u32, u32) = (5,  0b0_00000000_00000000_11100000);
pub const ADDRESS_LARGE_IMMEDIATE_FIELD                   : (u32, u32) = (8,  0b1_11111111_11111111_00000000);
pub const ADDRESS_LARGE_IMMEDIATE_REGISTER_FIELD          : (u32, u32) = (8,  0b0_00000000_00001111_00000000);

pub const ADDRESS_BASE_FIELD                              : (u32, u32) = (8,  0b0_00000000_00001111_00000000);
pub const ADDRESS_MEDIUM_IMMEDIATE_FIELD                  : (u32, u32) = (12, 0b1_11111111_11110000_00000000);
pub const ADDRESS_MEDIUM_IMMEDIATE_REGISTER_FIELD         : (u32, u32) = (12, 0b0_00000000_11110000_00000000);

pub const ADDRESS_INDEX_FIELD                             : (u32, u32) = (12, 0b0_00000000_11110000_00000000);
pub const ADDRESS_SHORT_IMMEDIATE_FIELD                   : (u32, u32) = (16, 0b1_11111111_00000000_00000000);
pub const ADDRESS_SHORT_IMMEDIATE_REGISTER_FIELD          : (u32, u32) = (16, 0b0_00001111_00000000_00000000);

impl Instruction {
    const fn decode_load_immediate_instruction_operands(operands: u32) -> (RegisterCode, SegmentCode, u16) {
        let destination = RegisterCode::new((operands & LOAD_IMMEDIATE_DESTINATION_FIELD) as u8);
        let segment = SegmentCode::new(((operands & LOAD_IMMEDIATE_SEGMENT_FIELD.1) >> LOAD_IMMEDIATE_SEGMENT_FIELD.0) as u8);
        let immediate = ((operands & LOAD_IMMEDIATE_IMMEDIATE_FIELD.1) >> LOAD_IMMEDIATE_IMMEDIATE_FIELD.0) as u16;
        (destination, segment, immediate)
    }
    
    const fn decode_enable_fields(operands: u32) -> [bool; 4] {
        let enable_0 = ((operands & ENABLE_0_FIELD.1) >> ENABLE_0_FIELD.0) > 0;
        let enable_1 = ((operands & ENABLE_1_FIELD.1) >> ENABLE_1_FIELD.0) > 0;
        let enable_2 = ((operands & ENABLE_2_FIELD.1) >> ENABLE_2_FIELD.0) > 0;
        let enable_3 = ((operands & ENABLE_3_FIELD.1) >> ENABLE_3_FIELD.0) > 0;
        [enable_0, enable_1, enable_2, enable_3]
    }

    fn decode_vector_components_instruction_operands(operands: u32) -> (RegisterCode, [Option<RegisterCode>; vector::SIZE]) {
        let primary = RegisterCode::new((operands & DECODE_VECTOR_DESTINATION_FIELD) as u8);

        let component_1 = RegisterCode::new(((operands & DECODE_VECTOR_COMPONENT_1_FIELD.1) >> DECODE_VECTOR_COMPONENT_1_FIELD.0) as u8);
        let component_2 = RegisterCode::new(((operands & DECODE_VECTOR_COMPONENT_2_FIELD.1) >> DECODE_VECTOR_COMPONENT_2_FIELD.0) as u8);
        let component_3 = RegisterCode::new(((operands & DECODE_VECTOR_COMPONENT_3_FIELD.1) >> DECODE_VECTOR_COMPONENT_3_FIELD.0) as u8);
        let component_0 = RegisterCode::new(((operands & DECODE_VECTOR_COMPONENT_0_FIELD.1) >> DECODE_VECTOR_COMPONENT_0_FIELD.0) as u8);

        let enable = Self::decode_enable_fields(operands);
        
        (primary, [
            enable[0].then_some(component_0),
            enable[1].then_some(component_1),
            enable[2].then_some(component_2),
            enable[3].then_some(component_3)
        ])
    }

    const fn decode_map_vector_instruction_operands(operands: u32) -> (bool, OperandCode, [ComponentCode; vector::SIZE]) {
        let temporary = operands & MAP_VECTOR_TEMPORARY_FIELD > 0;
        let operand = OperandCode::new(((operands & MAP_VECTOR_OPERAND_FIELD.1) >> MAP_VECTOR_OPERAND_FIELD.0) as u8);

        let component_0 = ComponentCode::new(((operands & MAP_VECTOR_COMPONENT_0_FIELD.1) >> MAP_VECTOR_COMPONENT_0_FIELD.0) as u8);
        let component_1 = ComponentCode::new(((operands & MAP_VECTOR_COMPONENT_1_FIELD.1) >> MAP_VECTOR_COMPONENT_1_FIELD.0) as u8);
        let component_2 = ComponentCode::new(((operands & MAP_VECTOR_COMPONENT_2_FIELD.1) >> MAP_VECTOR_COMPONENT_2_FIELD.0) as u8);
        let component_3 = ComponentCode::new(((operands & MAP_VECTOR_COMPONENT_3_FIELD.1) >> MAP_VECTOR_COMPONENT_3_FIELD.0) as u8);

        (temporary, operand, [
            component_0,
            component_1,
            component_2,
            component_3
        ])
    }

    fn decode_address_field(operands: u32) -> Address {
        let mode  = (operands & ADDRESS_MODE_FIELD.1) >> ADDRESS_MODE_FIELD.0;
        let immediate = mode == 0
            || mode == 2
            || mode == 6;

        match mode {
            0..4 => {
                let mode = match mode {
                    0 | 2 => address::Mode::Absolute,
                    1 | 3 => address::Mode::Relative,
                    _ => unreachable!(),
                };

                if immediate {
                    let immediate = address::LargeImmediate::new((operands & ADDRESS_LARGE_IMMEDIATE_FIELD.1) >> ADDRESS_LARGE_IMMEDIATE_FIELD.0);
                    Address::Immediate { mode, immediate }
                } else {
                    let register = RegisterCode::new(((operands & ADDRESS_LARGE_IMMEDIATE_REGISTER_FIELD.1) >> ADDRESS_LARGE_IMMEDIATE_REGISTER_FIELD.0) as u8);
                    Address::Register { mode, register }
                }
            },
            4..8 => {
                let base = RegisterCode::new(((operands & ADDRESS_BASE_FIELD.1) >> ADDRESS_BASE_FIELD.0) as u8);
                let mode = match mode {
                    4 => {
                        let offset = address::MediumImmediate::new(((operands & ADDRESS_MEDIUM_IMMEDIATE_FIELD.1) >> ADDRESS_MEDIUM_IMMEDIATE_FIELD.0) as u16);
                        address::BaseMode::Offset(offset)
                    },
                    5 => {
                        let offset = RegisterCode::new(((operands & ADDRESS_MEDIUM_IMMEDIATE_REGISTER_FIELD.1) >> ADDRESS_MEDIUM_IMMEDIATE_REGISTER_FIELD.0) as u8);
                        address::BaseMode::RegisterOffset(offset)
                    },
                    6 | 7 => {
                        let index = RegisterCode::new(((operands & ADDRESS_SHORT_IMMEDIATE_FIELD.1) >> ADDRESS_SHORT_IMMEDIATE_FIELD.0) as u8);
                        let offset = if immediate {
                            let immediate = address::ShortImmediate::new(((operands & ADDRESS_SHORT_IMMEDIATE_FIELD.1) >> ADDRESS_SHORT_IMMEDIATE_FIELD.0) as u16);
                            address::IndexedBaseOffsetMode::Immediate(immediate)
                        } else {
                            let register = RegisterCode::new(((operands & ADDRESS_SHORT_IMMEDIATE_REGISTER_FIELD.1) >> ADDRESS_SHORT_IMMEDIATE_REGISTER_FIELD.0) as u8);
                            address::IndexedBaseOffsetMode::Register(register)
                        };

                        address::BaseMode::Indexed {
                            index,
                            offset
                        }
                    },
                    _ => unreachable!()
                };

                Address::Base { mode, base }
            },
            _ => unreachable!()
        }
    }

    pub fn decode(encoded: u32) -> Self {
        let operation = encoded & OPERATION_MASK;
        let format = operation::MAPPINGS
            .get(operation as usize)
            .unwrap_or(&operation::MAPPINGS[0])
            .format;
        
        let operands = (encoded & !OPERATION_MASK) >> 7;
        
        match format {
            Format::WaitForInterrupt => Self::WaitForInterrupt,
            Format::LoadImmediate => {
                let (destination, segment, immediate) = Self::decode_load_immediate_instruction_operands(operands);
                Self::LoadImmediate { destination, segment, immediate }
            },
            Format::LoadVectorComponents => {
                let (destination, components) = Self::decode_vector_components_instruction_operands(operands);
                Self::LoadVectorComponents {destination, components}
            },
            Format::ExtractVectorComponents => {
                let (source, destinations) = Self::decode_vector_components_instruction_operands(operands);
                Self::ExtractVectorComponents { source, destinations }
            },
            Format::MapVector => {
                let (temporary, operand, components) = Self::decode_map_vector_instruction_operands(operands);
                Self::MapVector { temporary, operand, mappings: components, }
            },
            Format::Branch => todo!(),
            Format::DualSource(operation) => todo!(),
            Format::Destination(operation) => todo!(),
            Format::DestinationSource(operation) => todo!(),
            Format::DestinationDualSource(operation) => todo!(),
            Format::DestinationTripleSource(operation) => todo!(),
            Format::DualDestinationDualSource(operation) => todo!(),
            Format::Memory(operation) => {
                let address = Self::decode_address_field(operands);
                Self::Memory { operation, address }
            },
            Format::SourceMemory(operation) => todo!(),
            Format::DestinationMemory(operation) => todo!()
        }
    }
}