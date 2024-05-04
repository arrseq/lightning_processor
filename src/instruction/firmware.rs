// Header Section [xxxxxxxx xxxxxxxx] <- Instruction Header Length (IHL for ref)
// - Each header is 3 bytes long. Only 22 bit are used. 2 are wasted suffixes. 
// ROM ADDRESS and OPCODE| [RA,        RB,        IMM_PRES, IMM_BYTES]
// [xxxxxxxx|xxxxxxxx]   | [-------x] [------x-] [-----x--] [--xxx---]

// Code Section (Should contain any executable code)
// 0xFF 0xAF 0x4B 0x00 0x00 0x00 0xAA

use std::{collections::HashMap, io::{self, Read, Seek}};

use crate::environment::register::RegisterPresence;
use crate::utility::{Bits, Byte};

use super::{ImmediatePresence, MicroInstruction, ADD, ADD_DOUBLE, ADD_FLOAT, AND, BYTE_FROM_MEMORY, BYTE_TO_MEMORY, BYTE_TO_REGISTER, CLONE_REGISTER, DIVIDE, DIVIDE_DOUBLE, DIVIDE_FLOAT, DIVIDE_INTEGER, DOUBLE_WORD_FROM_MEMORY, DOUBLE_WORD_TO_MEMORY, DOUBLE_WORD_TO_REGISTER, EXCLUSIVE_OR, MULTIPLY, MULTIPLY_DOUBLE, MULTIPLY_FLOAT, MULTIPLY_INTEGER, NOT, NOTHING, OR, QUAD_WORD_FROM_MEMORY, QUAD_WORD_TO_MEMORY, QUAD_WORD_TO_REGISTER, SHIFT_END, SHIFT_START, SUBTRACT, SUBTRACT_DOUBLE, SUBTRACT_FLOAT, TRAILING_ZEROS, WORD_FROM_MEMORY, WORD_TO_MEMORY, WORD_TO_REGISTER};

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

#[derive(Default, Debug, Clone)]
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
        let bits = self.flags.into_bits();
        
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

// TODO TESTS

#[derive(Debug)]
pub enum Errors {
    InvalidMicroOperation,
    MissingInstructionEntry,
    TooManyInstructions,
    StreamTooShort,
    StreamError(io::Error)
}

#[derive(Debug)]
pub enum BlockErrors {
    InvalidMicroOperation,
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

    pub fn decode_block(microcode: &mut impl Read, entry: &RawEntry) -> Result<Vec<MicroInstruction>, BlockErrors> {
        // Read buffer which is used to read the instruction bytes
        let mut buffer: [u8; 1] = [0];

        let mut instructions = Vec::new();
        for _ in 0..entry.length {
            let operation = {
                match microcode.read(&mut buffer) {
                    Err(error) => return Err(BlockErrors::StreamError(error)),
                    Ok(bytes_read)    => {
                        if bytes_read != buffer.len() {
                            return Err(BlockErrors::StreamTooShort);
                        }
                    }
                }
    
                buffer[0]
            };

            let mut register_presence = RegisterPresence::None;
            let mut immediate_presence = ImmediatePresence::None;
    
            match operation {
                NOTHING => (),
                CLONE_REGISTER => {
                    // Clone register
                    register_presence = RegisterPresence::Ab;
                },
                BYTE_TO_REGISTER          
                | WORD_TO_REGISTER 
                | DOUBLE_WORD_TO_REGISTER 
                | QUAD_WORD_TO_REGISTER
                | BYTE_TO_MEMORY          
                | WORD_TO_MEMORY 
                | DOUBLE_WORD_TO_MEMORY   
                | QUAD_WORD_TO_MEMORY
                | BYTE_FROM_MEMORY        
                | WORD_FROM_MEMORY
                | DOUBLE_WORD_FROM_MEMORY 
                | QUAD_WORD_FROM_MEMORY => {
                    // [size] to register
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
                },
                ADD              
                | SUBTRACT      
                | MULTIPLY        
                | MULTIPLY_INTEGER
                | DIVIDE          
                | DIVIDE_INTEGER
                | ADD_DOUBLE      
                | ADD_FLOAT     
                | SUBTRACT_DOUBLE 
                | SUBTRACT_FLOAT
                | MULTIPLY_DOUBLE 
                | MULTIPLY_FLOAT
                | DIVIDE_DOUBLE   
                | DIVIDE_FLOAT  
                | AND             
                | OR            
                | EXCLUSIVE_OR    
                | NOT           
                | SHIFT_START     
                | SHIFT_END     
                | TRAILING_ZEROS => {
                    register_presence = RegisterPresence::Ab;
                }
                _ => todo!() // TODO
            };

            let mut register_a: Option<u8> = None;
            let mut register_b: Option<u8> = None;
            let mut immediate: Option<u64> = None;

            if register_presence.get_bytes_count() > 0 {
                let register_byte = {
                    match microcode.read(&mut buffer) {
                        Err(error) => return Err(BlockErrors::StreamError(error)),
                        Ok(bytes_read)    => {
                            if bytes_read != buffer.len() {
                                return Err(BlockErrors::StreamTooShort);
                            }
                        }
                    }
        
                    buffer[0]
                };

                let bits = register_byte.into_bits();

                register_a = Some([bits[0], bits[1], bits[2], bits[3], false, false, false, false].into_byte());
                register_b = Some([bits[4], bits[5], bits[6], bits[7], false, false, false, false].into_byte());
            }

            let mut immediate_bytes: [u8; 8] = [
                0x00, 0x00, 0x00, 0x00, 
                0x00, 0x00, 0x00, 0x00
            ];

            let immediate_bytes_count = immediate_presence.get_bytes_count();

            for index in 0..immediate_bytes_count {
                match microcode.read(&mut buffer) {
                    Err(error) => return Err(BlockErrors::StreamError(error)),
                    Ok(bytes_read)    => {
                        if bytes_read != buffer.len() {
                            return Err(BlockErrors::StreamTooShort);
                        }
                    }
                }
    
                immediate_bytes[index as usize] = buffer[0];
            }

            if immediate_bytes_count > 0 {
                immediate = Some(u64::from_le_bytes(immediate_bytes));
            }

            let instruction = match MicroInstruction::from(
                operation, 
                register_a.unwrap_or_default(),
                register_b.unwrap_or_default(),
                immediate.unwrap_or(0)
            ) {
                Err(_) => return Err(BlockErrors::InvalidMicroOperation),
                Ok(result) => result
            };

            instructions.push(instruction);
        }

        Ok(instructions)
    } 

    pub fn decode_entry(microcode: &mut (impl Read + Seek), entry: &RawEntry) -> Result<Entry, BlockErrors> {
        match microcode.seek(io::SeekFrom::Start(entry.address as u64)) {
            Err(error) => return Err(BlockErrors::StreamError(error)),
            Ok(_) => ()
        };

        let block = match Decoder::decode_block(microcode, &entry) {
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
    pub fn decode_binary(&mut self, microcode: &mut (impl Read + Seek)) -> Result<u8, Errors> {
        self.entries.clear();

        let raw_entires = match Decoder::read_raw_entries(microcode) {
            // Translate the error to allow for better error mitigation.
            Err(error) => return Err(match error {
                EntryErrors::StreamError(io_error) => Errors::StreamError(io_error),
                EntryErrors::StreamTooShort               => Errors::StreamTooShort
            }),
            Ok(entires) => entires
        };

        if raw_entires.len() == 0 {
            return Ok(0);
        }

        for raw_entry in &raw_entires {
            let entry = match Decoder::decode_entry(microcode, raw_entry) {
                Err(error) => return Err(match error {
                    BlockErrors::StreamError(io_error) => Errors::StreamError(io_error),
                    BlockErrors::StreamTooShort               => Errors::StreamTooShort,
                    BlockErrors::InvalidMicroOperation        => Errors::InvalidMicroOperation,
                    BlockErrors::MissingInstructionEntry      => Errors::MissingInstructionEntry,
                    BlockErrors::TooManyInstructions          => Errors::TooManyInstructions
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

    /// Get a specific entry
    pub fn get_entry(&self, operation: u8) -> Option<&Entry> {
        let entry = match self.entries.get(&operation) {
            None => return None,
            Some(e) => e
        };

        Some(&entry)
    }
}

pub struct Encoder {

}

impl Encoder {
    pub fn new() -> Self {
        Self {}
    }
}