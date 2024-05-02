// Header Section [xxxxxxxx xxxxxxxx] <- Instruction Header Length (IHL for ref)
// - Each header is 3 bytes long. Only 22 bit are used. 2 are wasted suffixes. 
// ROM ADDRESS and OPCODE| [RA,        RB,        IMM_PRES, IMM_BYTES]
// [xxxxxxxx|xxxxxxxx]   | [x-------] [-x------] [--x-----] [---xxx--]

// Code Section (Should contain any executable code)
// 0xFF 0xAF 0x4B 0x00 0x00 0x00 0xAA

use std::io::{self, Read};

use super::instruction::MicroInstruction;

#[derive(Default)]
pub enum ImmediatePresence {
    #[default]
    None,
    Byte,
    Word,
    DoubleWord,
    QuadWord
}

impl ImmediatePresence {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => false,
            _ => true
        }
    }
}

#[derive(Default)]
pub enum RegisterPresence {
    #[default]
    None,
    AB,
    A,
}

#[derive(Default)]
pub struct FirmwareEntry {
    pub operation:          u8,
    pub registers_presence: RegisterPresence,
    pub immediate_presence: ImmediatePresence,
    pub instructions:       Vec<MicroInstruction>
} 

#[derive(Default, Clone)]
pub struct RawEntry {
    pub address:   u8,
    pub operation: u8,
    pub flags:     u8
}

pub struct Firmware {
    entries: Vec<FirmwareEntry>
}

#[derive(Debug)]
pub enum Errors {
    MissingInstructionEntry,
    TooManyInstructions,
    StreamTooShort,
    StreamError(io::Error)
}

#[derive(Debug)]
pub enum EntryErrors {
    StreamError(io::Error),
    StreamTooShort
}

impl Firmware {
    pub fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }

    pub fn load_entries(&mut self, entires: Vec<FirmwareEntry>) {
        self.entries = entires;
    }

    pub fn read_entry(microcode: &mut impl Read, target: &mut RawEntry) -> Result<(), EntryErrors> {
        let mut buffer: [u8; 3] = [0, 0, 0];

        match microcode.read(&mut buffer) {
            Err(error) => return Err(EntryErrors::StreamError(error)),
            Ok(bytes_read) => {
                if bytes_read != buffer.len() {
                    return Err(EntryErrors::StreamTooShort)
                }

                target.address = buffer[0];
                target.operation = buffer[1];
                target.flags = buffer[2];
                return Ok(());
            }
        };
    }

    pub fn read_entries(microcode: &mut impl Read) -> Result<Vec<RawEntry>, EntryErrors> {
        let length = {
            let mut length_buffer: [u8; 1] = [0];
            match microcode.read(&mut length_buffer) {
                Err(error) => return Err(EntryErrors::StreamError(error)),
                Ok(bytes_read) => {
                    if bytes_read != length_buffer.len() {
                        return Err(EntryErrors::StreamTooShort);
                    }
                }
            }

            length_buffer[0]
        };

        let mut entries = Vec::new();
        let mut raw_entry: RawEntry = RawEntry { address: 0, operation: 0, flags: 0 };

        for _ in 0..length {
            match Self::read_entry(microcode, &mut raw_entry) {
                Err(error) => return Err(error),
                Ok(_) => {} 
            }

            entries.push(raw_entry.clone());
        }

        Ok(entries)
    }

    /// Load the microcode firmware onto this firmware interface.
    /// If ok, then it resolves with the number of detected operations
    pub fn load_binary(&mut self, microcode: &mut impl Read) -> Result<u16, Errors> {
        self.entries.clear();
        let mut buffer: [u8; 1] = [0];

        loop {
            match microcode.read(&mut buffer) {
                Err(error) => return Err(Errors::StreamError(error)),
                Ok(bytes_read) => {
                    if bytes_read != buffer.len() {
                        return Err(Errors::StreamTooShort)
                    }
                }
            };


        }

        Err(Errors::StreamTooShort)
    }

    pub fn get_micro_instructions() {

    }
}