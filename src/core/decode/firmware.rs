// Header Section [xxxxxxxx xxxxxxxx] <- Instruction Header Length (IHL for ref)
// - Each header is 3 bytes long. Only 22 bit are used. 2 are wasted suffixes. 
// ROM ADDRESS and OPCODE| [RA,        RB,        IMM_PRES, IMM_BYTES]
// [xxxxxxxx|xxxxxxxx]   | [-------x] [------x-] [-----x--] [--xxx---]

// Code Section (Should contain any executable code)
// 0xFF 0xAF 0x4B 0x00 0x00 0x00 0xAA

use std::{fmt::Display, io::{self, Read}};

use super::instruction::{ImmediatePresence, MicroInstruction, RegisterPresence, MAX_INSTRUCTION_BYTES};

#[repr(u8)]
pub enum FlagPositions {
    NoneA,
    NoneB,
    ImmediateBitA,
    ImmediateBitB,
    ImmediateBitC,
    ImmediatePresent,
    RegisterB,
    RegisterA
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
    pub flags:     u8,
    /// The number of bytes this entry took up in the firmware file.
    pub bytes:     u8
}

impl RawEntry {
    pub fn decode_flags(&self) -> (RegisterPresence, ImmediatePresence) {
        // [RA,        RB,        IMM_PRES, IMM_BYTES]
        // [-------x] [------x-] [-----x--] [--xxx---]
        let bits = get_bits_of_byte(self.flags);
        
        (
            RegisterPresence::from(
                bits[FlagPositions::RegisterA as usize] == 1, 
                bits[FlagPositions::RegisterB as usize] == 1
            ),
            ImmediatePresence::None
        )
    }
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

pub fn get_bits_of_byte(byte: u8) -> [u8; 8] {
    let mut bits = [0u8; 8];
    for i in 0..=7 {
        let shifted_byte = byte >> i;
        // Get the rightmost bit of the shifted byte (least significant bit)
        let cur_bit      = shifted_byte & 1;
        // For the first iteration, the cur_bit is the
        // least significant bit and therefore we place
        // that bit at index 7 of the array (rightmost bit)
        bits[7 - i]          = cur_bit;
    }
    bits
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

                target.address   = buffer[0];
                target.operation = buffer[1];
                target.flags     = buffer[2];
                target.bytes     = buffer.len() as u8;

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
        let mut raw_entry: RawEntry    = RawEntry { address: 0, operation: 0, flags: 0, bytes: 0 };

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
        let raw_entires = match Firmware::read_entries(microcode) {
            // Translate the error to allow for better error mitigation.
            Err(error) => return Err(match error {
                EntryErrors::StreamError(io_error) => Errors::StreamError(io_error),
                EntryErrors::StreamTooShort => Errors::StreamTooShort
            }),
            Ok(entires) => entires
        };

        if raw_entires.len() == 0 {
            return Ok(0);
        }
        
        // Read buffer which is used to read the instruction bytes
        let mut buffer: [u8; 1] = [0];

        for entry in raw_entires {
            for byte_index in 0..MAX_INSTRUCTION_BYTES {

            }

            let flags = entry.decode_flags();
            let register_presence   = flags.0;
            let immediate_presence = flags.1;
        }

        Err(Errors::StreamTooShort)
    }

    pub fn get_micro_instructions() {

    }
}