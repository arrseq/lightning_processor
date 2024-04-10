use std::io::Read;

// TODO: Remove
pub const IMMEDIATE_BYTES: u8 = 8;
pub type ImmediateType = u64;

pub enum MultiSizedData {
    Byte(u8),
    Word(u16),
    DWord(u32),
    QWord(u64)
}

// pub struct ImmediatePresence {
//     pub bytes_acceptance
// }

pub struct OperandsPresence {
    pub source0: bool,
    pub source1: bool,
    pub destination: bool,
    pub immediate: bool
}

pub struct Instruction {
    pub operation: u8,
    pub source0: Option<u8>,
    pub source1: Option<u8>,
    pub destination: Option<u8>,
    pub immediate: Option<ImmediateType>
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            operation: 0,
            source0: None,
            source1: None,
            destination: None,
            immediate: None
        }
    }
}

pub enum InstructionParseError {
    EndOfStream,
    OperationUnmatched
}

pub struct Parser {
    operation: u8,
    operands_presence: OperandsPresence
}

impl Parser {
    pub fn new(operation: u8, operand_presence: OperandsPresence) -> Self {
        Parser {
            operation,
            operands_presence: operand_presence
        }
    }

    // Returns an error if failed to parse
    pub fn parse(&mut self, target: &mut Instruction, source: &mut dyn Read) -> Option<InstructionParseError> {
        let mut buffer = [0 as u8; 1];

        let mut received_operation: u8 = 0;
        let mut received_source0: Option<u8> = None;
        let mut received_source1: Option<u8> = None;
        let mut received_destination: Option<u8> = None;
        let mut received_immediate: Option<ImmediateType> = None;

        let mut byte_index = 0;
        let expected = 1
            + self.operands_presence.destination as u8
            + self.operands_presence.source0 as u8
            + self.operands_presence.source1 as u8;

        for _ in 0..expected {
            let bytes_received = match source.read(&mut buffer) {
                Err(_) => return Some(InstructionParseError::EndOfStream),
                Ok(value) => value
            };

            if bytes_received == 0 {
                return Some(InstructionParseError::EndOfStream);
            }

            let value = buffer[0];
            match byte_index {
                0 => {
                    if value != self.operation {
                        return Some(InstructionParseError::OperationUnmatched);
                    }

                    received_operation = value;
                },
                1 => received_destination = Some(value),
                2 => received_source0 = Some(value),
                3 => received_source1 = Some(value),
                _ => unreachable!()
            };

            byte_index += 1;
        }

        if self.operands_presence.immediate {
            let mut immediate_buffer = [0 as u8; IMMEDIATE_BYTES as usize];
            match source.read(&mut immediate_buffer) {
                Err(_) => return Some(InstructionParseError::EndOfStream),
                Ok(bytes_read) => {
                    if bytes_read == 0 || bytes_read != IMMEDIATE_BYTES as usize {
                        return Some(InstructionParseError::EndOfStream);
                    }
                }
            }

            let mut imm_store: ImmediateType = 0;
            for &byte in immediate_buffer.iter() {
                imm_store = (imm_store << 8) | byte as ImmediateType;
            }

            received_immediate = Some(imm_store);
        }

        *target = Instruction {
            operation: received_operation,
            source0: received_source0,
            source1: received_source1,
            destination: received_destination,
            immediate: received_immediate
        };

        None
    }
}

pub fn read_sized_unit(byte_stream: &mut dyn Read) -> Option<Vec<u8>> {
    let mut buffer = [0 as u8; 1];
    match byte_stream.read(&mut buffer) {
        Err(_) => return None,
        Ok(bytes_received) => {
            if bytes_received == 0 {
                return None;
            }
        }
    };
    let value_size = buffer[0];

    let mut bytes: Vec<u8> = Vec::new();
    for _ in 0..value_size {
        match byte_stream.read(&mut buffer) {
            Err(_) => return None,
            Ok(length) => {
                if length == 0 {
                    return None;
                }
            }
        };

        bytes.push(buffer[0]);
    }
    
    Some(bytes) 
}