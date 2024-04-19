use std::io::Read;

pub const BYTE: u8 = 1;
pub const WORD: u8 = 2;
pub const DWORD: u8 = 4;
pub const QWORD: u8 = 8;
pub type Byte = u8;
pub type Word = u16;
pub type Dword = u32;
pub type Qword = u64;

#[repr(u8)]
pub enum Operations {
    Terminate,              // trm
    Interrupt,              // int, s0
    Safe,                   // sfe, s0
    
    // Data flow
    LoadImmediateByte,      // lib, s0, bt
    LoadImmediateWord,      // liw, s0, wd
    LoadImmediateDWord,     // ldw, s0, dw
    LoadImmediateQWord,     // lqw, s0, qw
    LoadInterconnect,       // lic, s0
    CloneRegister,          // cln, s0, s1

    // Random access memory
    LoadFromMemory,         // lfm, s0, s1, s2
    LoadToMemory,           // ltm, s0, s1

    // Arithmetic
    Add,                    // add, s0, s1, s2
    AddFloat,               // aft, s0, s1, s2
    AddDouble,              // adb, s0, s1, s2

    Subtract,               // sub, s0, s1, s2
    SubtractFloat,          // sft, s0, s1, s2
    SubtractDouble,         // sdb, s0, s1, s2

    Multiply,               // mul, s0, s1, s2
    MultiplyInteger,        // mit, s0, s1, s2
    MultiplyFloat,          // mft, s0, s1, s2
    MultiplyDouble,         // mdb, s0, s1, s2

    Divide,                 // div, s0, s1, s2
    DivideInteger,          // dit, s0, s1, s2
    DivideFloat,            // dft, s0, s1, s2
    DivideDouble,           // ddb, s0, s1, s2

    And,                    // and, s0, s1, s2
    Or,                     // or , s0, s1, s2
    ExclusiveOr,            // xor, s0, s1, s2
    Not,                    // not, s0, s1, s2
    ShiftStart,             // shs, s0, s1, s2
    ShiftEnd,               // she, s0, s1, s2
    TrailingZeros,          // tzr, TODO: Undecided

    // Branching
    Branch,                 // bch, s0
    BranchEqual,            // beq, s0, s1, s2
    BranchUnequal,          // buq, s0, s1, s2
    BranchGreater,          // bgr, s0, s1, s2
    BranchGreaterOrEqual,   // bge, s0, s1, s2
}

#[derive(Clone)]
pub enum MultiSizedData {
    Byte(u8),
    Word(u16),
    DWord(u32),
    QWord(u64)
}

pub fn multi_sized_to_bytes(multi: MultiSizedData) -> u8 {
    match multi {
        MultiSizedData::Byte(_) => BYTE,
        MultiSizedData::Word(_) => WORD,
        MultiSizedData::DWord(_) => DWORD,
        MultiSizedData::QWord(_) => QWORD
    }
}

pub fn multi_sized_to_usize(multi: MultiSizedData) -> usize {
    match multi {
        MultiSizedData::Byte(number) => number as usize,
        MultiSizedData::Word(number) => number as usize,
        MultiSizedData::DWord(number) => number as usize,
        MultiSizedData::QWord(number) => number as usize
    }
}

pub struct OperandsPresence {
    pub source0: bool,
    pub source1: bool,
    pub destination: bool,
    /// If this is Some() then the value provided in multi-sized data will be ignored
    pub immediate: Option<MultiSizedData>
}

pub struct Instruction {
    pub operation: u8,
    pub source0: Option<u8>,
    pub source1: Option<u8>,
    pub destination: Option<u8>,
    pub immediate: Option<MultiSizedData>
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

pub enum ParserErrors {
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

    /// Returns an error if failed to parse
    pub fn parse(&mut self, target: &mut Instruction, source: &mut dyn Read) -> Result<(), ParserErrors> {
        let mut buffer = [0 as u8; 1];

        let mut byte_index = 0;
        let expected = 1
            + self.operands_presence.destination as u8
            + self.operands_presence.source0 as u8
            + self.operands_presence.source1 as u8;

        for _ in 0..expected {
            let bytes_received = match source.read(&mut buffer) {
                Err(_) => return Err(ParserErrors::EndOfStream),
                Ok(value) => value
            };

            if bytes_received == 0 {
                return Err(ParserErrors::EndOfStream);
            }

            let value = buffer[0];
            match byte_index {
                0 => {
                    if value != self.operation {
                        return Err(ParserErrors::OperationUnmatched);
                    }

                    target.operation = value;
                },
                1 => target.destination = Some(value),
                2 => target.source0 = Some(value),
                3 => target.source1 = Some(value),
                _ => unreachable!()
            };

            byte_index += 1;
        }

        if let Some(immediate_config) = &self.operands_presence.immediate {
            match immediate_config {
                MultiSizedData::Byte(_) => {
                    let mut immediate_buffer = [0 as u8; BYTE as usize];
                    match source.read(&mut immediate_buffer) {
                        Err(_) => return Err(ParserErrors::EndOfStream),
                        Ok(bytes_read) => {
                            if bytes_read == 0 || bytes_read != BYTE as usize {
                                return Err(ParserErrors::EndOfStream);
                            }
                        }
                    }

                    target.immediate = Some(MultiSizedData::Byte(immediate_buffer[0]));
                },
                MultiSizedData::Word(_) => {
                    let mut immediate_buffer = [0 as u8; WORD as usize];
                    match source.read(&mut immediate_buffer) {
                        Err(_) => return Err(ParserErrors::EndOfStream),
                        Ok(bytes_read) => {
                            if bytes_read == 0 || bytes_read != WORD as usize {
                                return Err(ParserErrors::EndOfStream);
                            }
                        }
                    }

                    let mut imm_store: Word = 0;
                    for &byte in immediate_buffer.iter() {
                        imm_store = (imm_store << 8) | byte as Word;
                    }

                    target.immediate = Some(MultiSizedData::Word(imm_store));
                },
                MultiSizedData::DWord(_) => {
                    let mut immediate_buffer = [0 as u8; DWORD as usize];
                    match source.read(&mut immediate_buffer) {
                        Err(_) => return Err(ParserErrors::EndOfStream),
                        Ok(bytes_read) => {
                            if bytes_read == 0 || bytes_read != DWORD as usize {
                                return Err(ParserErrors::EndOfStream);
                            }
                        }
                    }

                    let mut imm_store: Dword = 0;
                    for &byte in immediate_buffer.iter() {
                        imm_store = (imm_store << 8) | byte as Dword;
                    }

                    target.immediate = Some(MultiSizedData::DWord(imm_store));
                },
                MultiSizedData::QWord(_) => {
                    let mut immediate_buffer = [0 as u8; QWORD as usize];
                    match source.read(&mut immediate_buffer) {
                        Err(_) => return Err(ParserErrors::EndOfStream),
                        Ok(bytes_read) => {
                            if bytes_read == 0 || bytes_read != QWORD as usize {
                                return Err(ParserErrors::EndOfStream);
                            }
                        }
                    }

                    let mut imm_store: Qword = 0;
                    for &byte in immediate_buffer.iter() {
                        imm_store = (imm_store << 8) | byte as Qword;
                    }

                    target.immediate = Some(MultiSizedData::QWord(imm_store));
                }
            }
        }

        Ok(())
    }
}

pub fn read_sized_unit(byte_stream: &mut dyn Read) -> Result<Vec<u8>, ()> {
    let mut buffer = [0 as u8; 1];
    match byte_stream.read(&mut buffer) {
        Err(_) => return Err(()),
        Ok(bytes_received) => {
            if bytes_received == 0 {
                return Err(());
            }
        }
    };
    let value_size = buffer[0];

    let mut bytes: Vec<u8> = Vec::new();
    for _ in 0..value_size {
        match byte_stream.read(&mut buffer) {
            Err(_) => return Err(()),
            Ok(length) => {
                if length == 0 {
                    return Err(());
                }
            }
        };

        bytes.push(buffer[0]);
    }
    
    Ok(bytes) 
}