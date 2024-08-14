use crate::instruction::{Format, Instruction, operation, RegisterCode, SegmentCode, vector};
use crate::num::MaskedU32;

pub const OPERATION_MASK: u32 = 0x0000007F;

pub const LOAD_IMMEDIATE_DESTINATION_FIELD      : u32        =      0b0_00000000_00000000_00001111;
pub const LOAD_IMMEDIATE_SEGMENT_FIELD          : (u32, u32) = (4,  0b0_00000000_00000000_00110000);
pub const LOAD_IMMEDIATE_IMMEDIATE_FIELD        : (u32, u32) = (6,  0b0_00111111_11111111_11000000);
pub const DECODE_VECTOR_DESTINATION_FIELD       : u32        =      0b0_00000000_00000000_00001111;
pub const DECODE_VECTOR_COMPONENT_ENABLE_0_FIELD: (u32, u32) = (4,  0b0_00000000_00000000_00010000);
pub const DECODE_VECTOR_COMPONENT_ENABLE_1_FIELD: (u32, u32) = (5,  0b0_00000000_00000000_00100000);
pub const DECODE_VECTOR_COMPONENT_ENABLE_2_FIELD: (u32, u32) = (6,  0b0_00000000_00000000_01000000);
pub const DECODE_VECTOR_COMPONENT_ENABLE_3_FIELD: (u32, u32) = (7,  0b0_00000000_00000000_10000000);
pub const DECODE_VECTOR_COMPONENT_0_FIELD       : (u32, u32) = (8,  0b0_00000000_00001111_00000000);
pub const DECODE_VECTOR_COMPONENT_1_FIELD       : (u32, u32) = (12, 0b0_00000000_11110000_00000000);
pub const DECODE_VECTOR_COMPONENT_2_FIELD       : (u32, u32) = (16, 0b0_00001111_00000000_00000000);
pub const DECODE_VECTOR_COMPONENT_3_FIELD       : (u32, u32) = (20, 0b0_11110000_00000000_00000000);
pub const FLAG_VECTOR_TEMPORARY_FIELD           : u32        =      0b0_00000000_00000000_00000001;
pub const FLAG_VECTOR_OPERAND_0_FIELD           : (u32, u32) = (1,  0b0_00000000_00000000_00000110);
pub const FLAG_VECTOR_OPERAND_1_FIELD           : (u32, u32) = (3,  0b0_00000000_00000000_00011000);
pub const FLAG_VECTOR_OPERAND_2_FIELD           : (u32, u32) = (5,  0b0_00000000_00000000_01100000);
pub const FLAG_VECTOR_OPERAND_3_FIELD           : (u32, u32) = (7,  0b0_00000000_00000001_10000000);
pub const FLAG_VECTOR_OPERAND_0_NEGATE_FIELD    : (u32, u32) = (9,  0b0_00000000_00000010_00000000);
pub const FLAG_VECTOR_OPERAND_1_NEGATE_FIELD    : (u32, u32) = (10, 0b0_00000000_00000100_00000000);
pub const FLAG_VECTOR_OPERAND_2_NEGATE_FIELD    : (u32, u32) = (11, 0b0_00000000_00001000_00000000);
pub const FLAG_VECTOR_OPERAND_3_NEGATE_FIELD    : (u32, u32) = (12, 0b0_00000000_00010000_00000000);
pub const FLAG_VECTOR_OPERAND_0_ZERO_FIELD      : (u32, u32) = (13, 0b0_00000000_00100000_00000000);
pub const FLAG_VECTOR_OPERAND_1_ZERO_FIELD      : (u32, u32) = (14, 0b0_00000000_01000000_00000000);
pub const FLAG_VECTOR_OPERAND_2_ZERO_FIELD      : (u32, u32) = (15, 0b0_00000000_10000000_00000000);
pub const FLAG_VECTOR_OPERAND_3_ZERO_FIELD      : (u32, u32) = (16, 0b0_00000001_00000000_00000000);

pub type EncodedOperands = MaskedU32<0x1FFFFFF>;

impl Instruction {
    const fn decode_load_immediate(operands: EncodedOperands) -> (RegisterCode, SegmentCode, u16) {
        let operands = operands.get();
        let destination = RegisterCode::new((operands & LOAD_IMMEDIATE_DESTINATION_FIELD) as u8);
        let segment = SegmentCode::new(((operands & LOAD_IMMEDIATE_SEGMENT_FIELD.1) >> LOAD_IMMEDIATE_SEGMENT_FIELD.0) as u8);
        let immediate = ((operands & LOAD_IMMEDIATE_IMMEDIATE_FIELD.1) >> LOAD_IMMEDIATE_IMMEDIATE_FIELD.0) as u16;
        (destination, segment, immediate)
    }

    fn decode_vector_components(operands: EncodedOperands) -> (RegisterCode, [Option<RegisterCode>; vector::SIZE]) {
        let operands = operands.get();
        let primary = RegisterCode::new((operands & DECODE_VECTOR_DESTINATION_FIELD) as u8);

        let enable_0 = ((operands & DECODE_VECTOR_COMPONENT_ENABLE_0_FIELD.1) >> DECODE_VECTOR_COMPONENT_ENABLE_0_FIELD.0) > 0;
        let enable_1 = ((operands & DECODE_VECTOR_COMPONENT_ENABLE_1_FIELD.1) >> DECODE_VECTOR_COMPONENT_ENABLE_1_FIELD.0) > 0;
        let enable_2 = ((operands & DECODE_VECTOR_COMPONENT_ENABLE_2_FIELD.1) >> DECODE_VECTOR_COMPONENT_ENABLE_2_FIELD.0) > 0;
        let enable_3 = ((operands & DECODE_VECTOR_COMPONENT_ENABLE_3_FIELD.1) >> DECODE_VECTOR_COMPONENT_ENABLE_3_FIELD.0) > 0;

        let component_1 = RegisterCode::new(((operands & DECODE_VECTOR_COMPONENT_1_FIELD.1) >> DECODE_VECTOR_COMPONENT_1_FIELD.0) as u8);
        let component_2 = RegisterCode::new(((operands & DECODE_VECTOR_COMPONENT_2_FIELD.1) >> DECODE_VECTOR_COMPONENT_2_FIELD.0) as u8);
        let component_3 = RegisterCode::new(((operands & DECODE_VECTOR_COMPONENT_3_FIELD.1) >> DECODE_VECTOR_COMPONENT_3_FIELD.0) as u8);
        let component_0 = RegisterCode::new(((operands & DECODE_VECTOR_COMPONENT_0_FIELD.1) >> DECODE_VECTOR_COMPONENT_0_FIELD.0) as u8);

        (primary, [
            enable_0.then_some(component_0),
            enable_1.then_some(component_1),
            enable_2.then_some(component_2),
            enable_3.then_some(component_3)
        ])
    }

    pub fn decode(encoded: u32) -> Self {
        let operation = encoded & OPERATION_MASK;
        
        let format = operation::MAPPINGS
            .get(operation as usize)
            .unwrap_or(&operation::MAPPINGS[0])
            .format;
        
        let operands = EncodedOperands::new((encoded & !OPERATION_MASK) >> 7);
        
        match format {
            Format::WaitForInterrupt => Self::WaitForInterrupt,
            Format::LoadImmediate => {
                let (destination, segment, immediate) = Self::decode_load_immediate(operands);
                Self::LoadImmediate { destination, segment, immediate }
            },
            Format::LoadVectorComponents => {
                let (destination, components) = Self::decode_vector_components(operands);
                Self::LoadVectorComponents {destination, components}
            },
            Format::ExtractVectorComponents => {
                let (source, destinations) = Self::decode_vector_components(operands);
                Self::ExtractVectorComponents { source, destinations }
            },
            Format::FlagVectorComponents => { 
                let operand_0 = 
                todo!()
            },
            Format::MapVector => todo!(),
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