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

        let mut received_operation: u8 = 0;
        let mut received_source0: Option<u8> = None;
        let mut received_source1: Option<u8> = None;
        let mut received_destination: Option<u8> = None;

        let mut byte_index = 0;
        let expected = 1
            + self.operand_presense.destination as u8
            + self.operand_presense.source0 as u8
            + self.operand_presense.source1 as u8;

        for _ in 0..expected {
            let bytes_received = match source.read(&mut buffer) {
                Err(_) => return Err(Error::EndOfStream),
                Ok(value) => value
            };

            if bytes_received == 0 {
                return Err(Error::EndOfStream);
            }

            let value = buffer[0];
            match byte_index {
                0 => {
                    if value != self.operation {
                        return Err(Error::OperandUnmatched);
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

        Ok(Instruction {
            operation: received_operation,
            source0: received_source0,
            source1: received_source1,
            destination: received_destination
        })
    }
}