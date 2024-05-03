// Header Section [xxxxxxxx xxxxxxxx] <- Instruction Header Length (IHL for ref)
// - Each header is 3 bytes long. Only 22 bit are used. 2 are wasted suffixes. 
// ROM ADDRESS and OPCODE| [RA,        RB,        IMM_PRES, IMM_BYTES]
// [xxxxxxxx|xxxxxxxx]   | [-------x] [------x-] [-----x--] [--xxx---]

// Code Section (Should contain any executable code)
// 0xFF 0xAF 0x4B 0x00 0x00 0x00 0xAA

use std::{collections::HashMap, io::{self, Read, Seek}};

use crate::environment::register::Register;

use super::{ImmediatePresence, MicroInstruction, RegisterPresence};

pub const ADDRESS_BYTE:      u8 = 0;
pub const LENGTH_BYTE:       u8 = 1;
pub const OPERATION_BYTE:    u8 = 2;
pub const FLAGS_BYTE:        u8 = 3;
pub const ENTRY_BUFFER_SIZE: u8 = 4;

#[repr(u8)]
pub enum FlagIndexes {
    NoneA,
    NoneB,
    ImmediateBitA,
    ImmediateBitB,
    ImmediateBitC,
    ImmediatePresent,
    RegisterB,
    RegisterA
}

#[derive(Default, Debug)]
pub struct Entry {
    pub operation:          u8,
    pub registers_presence: RegisterPresence,
    pub immediate_presence: ImmediatePresence,
    pub instructions:       Vec<MicroInstruction>
} 

#[derive(Default, Clone, Debug)]
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
                bits[FlagIndexes::RegisterA as usize], 
                bits[FlagIndexes::RegisterB as usize]
            ),
            // TODO Implement this
            ImmediatePresence::None
        )
    }
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

pub struct Decoder {
    /// Key is the macro operation and value is the entry
    entries: HashMap<u8, Entry>
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new()
        }
    }

    pub fn load_entries(&mut self, entires: HashMap<u8, Entry>) {
        self.entries = entires;
    }

    pub fn read_raw_entry(microcode: &mut impl Read, target: &mut RawEntry) -> Result<(), EntryErrors> {
        let mut buffer: [u8; ENTRY_BUFFER_SIZE as usize] = [0, 0, 0, 0];

        match microcode.read(&mut buffer) {
            Err(error) => return Err(EntryErrors::StreamError(error)),
            Ok(bytes_read) => {
                if bytes_read != buffer.len() {
                    return Err(EntryErrors::StreamTooShort)
                }


                target.address   = buffer[ADDRESS_BYTE   as usize];
                target.length    = buffer[LENGTH_BYTE    as usize];
                target.operation = buffer[OPERATION_BYTE as usize];
                target.flags     = buffer[FLAGS_BYTE     as usize];

                return Ok(());
            }
        };
    }

    pub fn read_raw_entries(microcode: &mut impl Read) -> Result<Vec<RawEntry>, EntryErrors> {
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
            match Self::read_raw_entry(microcode, &mut raw_entry) {
                Err(error) => return Err(error),
                Ok(_) => () 
            }

            entries.push(raw_entry.clone());
        }

        Ok(entries)
    }

    pub fn read_block(microcode: &mut impl Read, entry: &RawEntry) -> Result<Vec<MicroInstruction>, EntryErrors> {
        // Read buffer which is used to read the instruction bytes
        let mut buffer: [u8; 1] = [0];

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
                0 => (),
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
                _ => todo!() // TODO
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

            let dead_register = Register::from_pointer(0).unwrap();

            instructions.push(MicroInstruction::from(
                operation, 
                Register::from_pointer(register_a.unwrap_or_default()).unwrap_or(dead_register.clone()),
                Register::from_pointer(register_b.unwrap_or_default()).unwrap_or(dead_register.clone()),
                immediate.unwrap_or(0)
            ));
        }

        Ok(instructions)
    } 

    pub fn read_entry(microcode: &mut (impl Read + Seek), entry: &RawEntry) -> Result<Entry, EntryErrors> {
        match microcode.seek(io::SeekFrom::Start(entry.address as u64)) {
            Err(error) => return Err(EntryErrors::StreamError(error)),
            Ok(_) => ()
        };

        let block = match Decoder::read_block(microcode, &entry) {
            Err(error) => return Err(error),
            Ok(result) => result
        };

        let flags = entry.decode_flags();
        let registers_presence = flags.0;
        let immediate_presence = flags.1;

        Ok(Entry {
            operation: entry.operation,
            registers_presence, 
            immediate_presence,
            instructions: block
        })
    }

    /// Load the microcode firmware onto this firmware interface.
    /// If ok, then it resolves with the number of detected operations
    pub fn load_binary(&mut self, microcode: &mut (impl Read + Seek)) -> Result<u8, Errors> {
        self.entries.clear();

        let raw_entires = match Decoder::read_raw_entries(microcode) {
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

        for raw_entry in &raw_entires {
            let entry = match Decoder::read_entry(microcode, raw_entry) {
                Err(error) => return Err(match error {
                    EntryErrors::StreamError(io_error) => Errors::StreamError(io_error),
                    EntryErrors::StreamTooShort => Errors::StreamTooShort
                }),
                Ok(result) => result
            };

            self.entries.insert(entry.operation, entry);
        }

        Ok(raw_entires.len() as u8)
    }

    pub fn get_entries(&self) -> &HashMap<u8, Entry> {
        &self.entries
    }

    // /// Decode a macro operation
    pub fn decode_macro(&self, operation: u8) -> Option<&Vec<MicroInstruction>> {
        let entry = match self.entries.get(&operation) {
            None => return None,
            Some(e) => e
        };

        Some(&entry.instructions)
    }
}

pub struct Block {

}

pub struct Encoder {

}

impl Encoder {
    pub fn new() -> Self {
        Self {}
    }
}