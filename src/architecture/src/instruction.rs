use crate::data::{ByteStream};

pub struct Parser<'a> {
    pub byte_stream: &'a mut dyn ByteStream,
    pub opcode: u8,
    pub r0_expected: bool,
    pub r1_expected: bool,
    pub imm_expected: bool,
    pub imm_size: u8 // Quantity of bytes
}

pub enum ParserError {
    // Failed attempt at reading bytes from the stream
    EndOfStream,
    // Opcode does not match the one specificed in this instruction parser
    InvalidOpcode
}

pub struct Instruction {
    pub opcode: u8,
    pub r0: Option<u8>,
    pub r1: Option<u8>,
    pub imm: Option<Vec<u8>>
}

impl<'a> Parser<'a> {
    pub fn try_parse_queued(&mut self) -> Result<Instruction, ParserError> {
        let opcode = match self.byte_stream.get_queued() {
            Ok(opcode) => opcode,
            Err(()) => return Err(ParserError::EndOfStream)
        };

        if opcode != self.opcode { return Err(ParserError::InvalidOpcode); }

        let mut r0: Option<u8> = None;
        if self.r0_expected {
            r0 = match self.byte_stream.get_queued() {
                Ok(r0) => Some(r0),
                Err(()) => return Err(ParserError::EndOfStream)
            }
        }

        let mut r1: Option<u8> = None;
        if self.r1_expected {
            r1 = match self.byte_stream.get_queued() {
                Ok(r1) => Some(r1),
                Err(()) => return Err(ParserError::EndOfStream)
            }
        }

        let mut imm: Option<Vec<u8>> = None;
        if self.imm_expected {
            imm = Some(Vec::new());
            let bunch = self.byte_stream.get_bunch(self.imm_size as usize);

            // Check for clipping. This results in lost cycles, to avoid undefined behavior
            // in bad programs or poorly written compilers, its better to sanitize.
            for byte in bunch {
                if byte == Err(()) {
                    return Err(ParserError::EndOfStream);
                } else {
                    imm.as_mut().unwrap().push(byte.unwrap());
                }
            }
        }

        let ins = Instruction {
            opcode,
            r0,
            r1,
            imm,
        };

        Ok(ins)
    }
}