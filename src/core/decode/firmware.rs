// Header Section [xxxxxxxx xxxxxxxx] <- Instruction Header Length (IHL for ref)
// - Each header is 3 bytes long. Only 22 bit are used. 2 are wasted suffixes. 
// ROM ADDRESS and OPCODE| [RA,        RB,        IMM_PRES, IMM_BYTES]
// [xxxxxxxx|xxxxxxxx]   | [-------x] [------x-] [-----x--] [--xxx---]

// Code Section (Should contain any executable code)
// 0xFF 0xAF 0x4B 0x00 0x00 0x00 0xAA

use std::{fmt::Display, io::{self, Read, Seek}};

use super::instruction::{ImmediatePresence, MicroInstruction, RegisterPresence, MAX_INSTRUCTION_BYTES, OPERATION_BYTES};

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
    pub length:    u8,
    pub operation: u8,
    pub flags:     u8
}

impl RawEntry {
    pub fn decode_flags(&self) -> (RegisterPresence, ImmediatePresence) {
        // [RA,        RB,        IMM_PRES, IMM_BYTES]
        // [-------x] [------x-] [-----x--] [--xxx---]
        let bits = get_bits_of_byte(self.flags);
        
        (
            RegisterPresence::from(
                bits[FlagPositions::RegisterA as usize], 
                bits[FlagPositions::RegisterB as usize]
            ),
            // TODO Implement this
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

pub fn get_bits_of_byte(byte: u8) -> [bool; 8] {
    let mut bits = [false; 8];
    for i in 0..=7 {
        let shifted_byte = byte >> i;
        // Get the rightmost bit of the shifted byte (least significant bit)
        let cur_bit      = shifted_byte & 1;
        // For the first iteration, the cur_bit is the
        // least significant bit and therefore we place
        // that bit at index 7 of the array (rightmost bit)
        bits[7 - i]          = cur_bit == 1;
    }
    
    bits
}

pub fn bits_to_u8(slice: &[bool]) -> Option<u8> {
    if slice.len() != 4 {
        return None;
    }

    let mut result = 0;

    for &bit in slice {
        result <<= 1; // Shift the result left by 1 bit
        result |= bit as u8; // Set the least significant bit to the current bit
    }

    Some(result)
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
        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        match microcode.read(&mut buffer) {
            Err(error) => return Err(EntryErrors::StreamError(error)),
            Ok(bytes_read) => {
                if bytes_read != buffer.len() {
                    return Err(EntryErrors::StreamTooShort)
                }

                target.address   = buffer[0];
                target.length    = buffer[1];
                target.operation = buffer[2];
                target.flags     = buffer[3];

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
        let mut raw_entry: RawEntry    = RawEntry { address: 0, length: 0, operation: 0, flags: 0 };

        for _ in 0..length {
            match Self::read_entry(microcode, &mut raw_entry) {
                Err(error) => return Err(error),
                Ok(_) => {} 
            }

            entries.push(raw_entry.clone());
        }

        Ok(entries)
    }

    pub fn read_block(microcode: &mut (impl Read + Seek), entry: RawEntry) -> Result<Vec<MicroInstruction>, EntryErrors> {
        // Read buffer which is used to read the instruction bytes
        let mut buffer: [u8; 1] = [0];

        match microcode.seek(io::SeekFrom::Start(entry.address as u64)) {
            Err(error) => return Err(EntryErrors::StreamError(error)),
            Ok(_)             => {}
        }

        let mut instructions = Vec::new();
        for _ in 0..entry.length {
            let operation = {
                match microcode.read(&mut buffer) {
                    Err(error) => return Err(EntryErrors::StreamError(error)),
                    Ok(bytes_read)    => {
                        if bytes_read != buffer.len() {
                            return Err(EntryErrors::StreamTooShort);
                        }
                    }
                }
    
                buffer[0]
            };

            let mut register_presence = RegisterPresence::None;
            let mut immediate_presence = ImmediatePresence::None;
    
            match operation {
                0 => {},
                1 => {
                    // Clone register
                    register_presence = RegisterPresence::Ab;
                },
                2 | 3 | 4 | 5 => {
                    // ??? to register
                    register_presence = RegisterPresence::A;

                    if operation == 2 {
                        // Byte
                        immediate_presence = ImmediatePresence::Byte;
                    } else if operation == 3 {
                        immediate_presence = ImmediatePresence::Word;
                    } else if operation == 4 {
                        immediate_presence = ImmediatePresence::DoubleWord;
                    } else {
                        immediate_presence = ImmediatePresence::QuadWord;
                    }
                }
                _ => todo!()
            };

            let mut register_a: Option<u8> = None;
            let mut register_b: Option<u8> = None;
            let mut immediate: Option<u64> = None;

            if register_presence.get_bytes_count() > 0 {
                let register_byte = {
                    match microcode.read(&mut buffer) {
                        Err(error) => return Err(EntryErrors::StreamError(error)),
                        Ok(bytes_read)    => {
                            if bytes_read != buffer.len() {
                                return Err(EntryErrors::StreamTooShort);
                            }
                        }
                    }
        
                    buffer[0]
                };

                let bits = get_bits_of_byte(register_byte);

                register_a = Some(bits_to_u8(&bits[0..4]).unwrap_or(0));
                register_b = Some(bits_to_u8(&bits[4..8]).unwrap_or(0));
            }

            let mut immediate_bytes: [u8; 8] = [
                0x00, 0x00, 0x00, 0x00, 
                0x00, 0x00, 0x00, 0x00
            ];

            let immediate_bytes_count = immediate_presence.get_bytes_count();

            for index in 0..immediate_bytes_count {
                match microcode.read(&mut buffer) {
                    Err(error) => return Err(EntryErrors::StreamError(error)),
                    Ok(bytes_read)    => {
                        if bytes_read != buffer.len() {
                            return Err(EntryErrors::StreamTooShort);
                        }
                    }
                }
    
                immediate_bytes[index as usize] = buffer[0];
            }

            if immediate_bytes_count > 0 {
                immediate = Some(u64::from_le_bytes(immediate_bytes));
            }

            instructions.push(MicroInstruction::from(
                operation, 
                register_a.unwrap_or(0),
                register_b.unwrap_or(0),
                immediate.unwrap_or(0)
            ));
        }

        Ok(instructions)
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

        Err(Errors::StreamTooShort)
    }
}