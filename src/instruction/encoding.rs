use crate::instruction::{Format, Instruction, operation, RegisterCode, SegmentCode};

pub const OPERATION_MASK: u32 = 0x0000007F;

pub const LOAD_IMMEDIATE_DESTINATION: u32 = 0b00000000_00000000_00000000_00001111;
pub const LOAD_IMMEDIATE_SEGMENT    : (u32, u32) = (4, 0b00000000_00000000_00000000_00110000);
pub const LOAD_IMMEDIATE_IMMEDIATE  : (u32, u32) = (6, 0b00000000_00111111_11111111_11000000);

impl Instruction {
    const fn decode_load_immediate(operands: u32) -> (RegisterCode, SegmentCode, u16) {
        let destination = RegisterCode::new((operands & LOAD_IMMEDIATE_DESTINATION) as u8);
        let segment = SegmentCode::new(((operands & LOAD_IMMEDIATE_SEGMENT.1) >> LOAD_IMMEDIATE_SEGMENT.0) as u8);
        let immediate = ((operands & LOAD_IMMEDIATE_IMMEDIATE.1) >> LOAD_IMMEDIATE_IMMEDIATE.0) as u16;
        (destination, segment, immediate)
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
                let (destination, segment, immediate) = Self::decode_load_immediate(operands);
                Self::LoadImmediate { destination, segment, immediate }
            },
            Format::LoadVectorComponents => todo!(),
            Format::ExtractVectorComponents => todo!(),
            Format::FlagVectorComponents => todo!(),
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