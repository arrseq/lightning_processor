use std::{fmt::Display, io::Read};

pub struct OperandPresense {
    pub source0: bool,
    pub source1: bool,
    pub destination: bool
}

pub struct Instruction {
    pub operation: u8,
    pub source0: Option<u8>,
    pub source1: Option<u8>,
    pub destination: Option<u8>
}

#[derive(Debug)]
pub enum Error {
    EndOfStream,
    OperandUnmatched
}

pub struct Parser {
    operation: u8,
    operand_presense: OperandPresense
}

impl Parser {
    pub fn new(operation: u8, operand_presense: OperandPresense) -> Self {
        Parser {
            operation,
            operand_presense
        }
    }

    pub fn parse(&mut self, source: &mut dyn Read) -> Result<Instruction, Error> {
        let mut buffer = [0 as u8; 1];
        let opcode_bytes_received = match source.read(&mut buffer) {
            Err(_) => return Err(Error::EndOfStream),
            Ok(value) => value
        };

        if opcode_bytes_received == 0 {
            return Err(Error::EndOfStream);
        }

        let received_operation = buffer[0];
        if received_operation != self.operation {
            return Err(Error::OperandUnmatched);
        }

        let mut received_destination: Option<u8> = None;
        let mut received_source0: Option<u8> = None;
        let mut received_source1: Option<u8> = None;

        if self.operand_presense.destination {
            let operand_bytes_received = match source.read(&mut buffer) {
                Err(_) => return Err(Error::EndOfStream),
                Ok(value) => value
            };

            if operand_bytes_received == 0 {
                return Err(Error::EndOfStream);
            }

            received_destination = Some(buffer[0]);
        }

        if self.operand_presense.source1 {
            let operand_bytes_received = match source.read(&mut buffer) {
                Err(_) => return Err(Error::EndOfStream),
                Ok(value) => value
            };

            if operand_bytes_received == 0 {
                return Err(Error::EndOfStream);
            }

            received_source0 = Some(buffer[0]);
        }

        if self.operand_presense.source1 {
            let operand_bytes_received = match source.read(&mut buffer) {
                Err(_) => return Err(Error::EndOfStream),
                Ok(value) => value
            };

            if operand_bytes_received == 0 {
                return Err(Error::EndOfStream);
            }

            received_source1 = Some(buffer[0]);
        }

        Ok(Instruction {
            operation: received_operation,
            source0: received_source0,
            source1: received_source1,
            destination: received_destination
        })
    }
}