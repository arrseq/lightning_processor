use crate::instruction::{Format, Instruction, OperandCode, operation, RegisterCode, SegmentCode, vector};
use crate::instruction::vector::ComponentCode;
use crate::num::MaskedU32;

pub const OPERATION_MASK: u32 = 0x0000007F;

pub const LOAD_IMMEDIATE_DESTINATION_FIELD      : u32        =      0b0_00000000_00000000_00001111;
pub const LOAD_IMMEDIATE_SEGMENT_FIELD          : (u32, u32) = (4,  0b0_00000000_00000000_00110000);
pub const LOAD_IMMEDIATE_IMMEDIATE_FIELD        : (u32, u32) = (6,  0b0_00111111_11111111_11000000);
pub const DECODE_VECTOR_DESTINATION_FIELD       : u32        =      0b0_00000000_00000000_00001111;
pub const ENABLE_0_FIELD: (u32, u32) = (4,  0b0_00000000_00000000_00010000);
pub const ENABLE_1_FIELD: (u32, u32) = (5,  0b0_00000000_00000000_00100000);
pub const ENABLE_2_FIELD: (u32, u32) = (6,  0b0_00000000_00000000_01000000);
pub const ENABLE_3_FIELD: (u32, u32) = (7,  0b0_00000000_00000000_10000000);
pub const DECODE_VECTOR_COMPONENT_0_FIELD       : (u32, u32) = (8,  0b0_00000000_00001111_00000000);
pub const DECODE_VECTOR_COMPONENT_1_FIELD       : (u32, u32) = (12, 0b0_00000000_11110000_00000000);
pub const DECODE_VECTOR_COMPONENT_2_FIELD       : (u32, u32) = (16, 0b0_00001111_00000000_00000000);
pub const DECODE_VECTOR_COMPONENT_3_FIELD       : (u32, u32) = (20, 0b0_11110000_00000000_00000000);
pub const MAP_VECTOR_TEMPORARY_FIELD            : u32        =      0b0_00000000_00000000_00000001;
pub const MAP_VECTOR_OPERAND_FIELD              : (u32, u32) = (1,  0b0_00000000_00000000_00000110);
pub const MAP_VECTOR_COMPONENT_0_FIELD          : (u32, u32) = (3,  0b0_00000000_00000000_00011000);
pub const MAP_VECTOR_COMPONENT_1_FIELD          : (u32, u32) = (5,  0b0_00000000_00000000_01100000);
pub const MAP_VECTOR_COMPONENT_2_FIELD          : (u32, u32) = (7,  0b0_00000000_00000001_10000000);
pub const MAP_VECTOR_COMPONENT_3_FIELD          : (u32, u32) = (9,  0b0_00000000_00000110_00000000);

impl Instruction {
    const fn decode_load_immediate_instruction_operands(operands: u32) -> (RegisterCode, SegmentCode, u16) {
        let destination = RegisterCode::new((operands & LOAD_IMMEDIATE_DESTINATION_FIELD) as u8);
        let segment = SegmentCode::new(((operands & LOAD_IMMEDIATE_SEGMENT_FIELD.1) >> LOAD_IMMEDIATE_SEGMENT_FIELD.0) as u8);
        let immediate = ((operands & LOAD_IMMEDIATE_IMMEDIATE_FIELD.1) >> LOAD_IMMEDIATE_IMMEDIATE_FIELD.0) as u16;
        (destination, segment, immediate)
    }
    
    fn decode_enable_fields(operands: u32) -> [bool; 4] {
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
    
    fn decode_map_vector_instruction_operands(operands: u32) -> (bool, OperandCode, [ComponentCode; vector::SIZE]) {
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
            Format::DualSource => todo!(),
            Format::Destination => todo!(),
            Format::DestinationSource => todo!(), 
            Format::DestinationDualSource => todo!(),
            Format::DestinationTripleSource => todo!(),
            Format::DualDestinationDualSource => todo!(),
            Format::Memory => todo!(),
            Format::SourceMemory => todo!(),
            Format::DestinationMemory => todo!()
        }
    }
}