use std::io::{Error, Read, Seek};

pub const BYTE: u8 = 1;
pub const WORD: u8 = 2;
pub const DWORD: u8 = 4;
pub const QWORD: u8 = 8;
pub type Byte = u8;
pub type Word = u16;
pub type Dword = u32;
pub type Qword = u64;

/// The number of bytes an instruction can be while excluding the immediate
pub const MAX_BYTES_EXCLUDING_IMM: u8 = 1  // Operand
                                      + 1  // Destination
                                      + 1  // Source 0
                                      + 1; // Source 1

#[repr(u8)]
pub enum Operations {
    Nothing,                // ntg
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
    ByteToMemory,           // btm, s0, s1
    WordToMemory,           // wtm, s0, s1
    DWordToMemory,          // dtm, s0, s1
    QWordToMemory,          // qtm, s0, s1
    ByteFromMemory,         // bfm, s0, s1
    WordFromMemory,         // wfm, s0, s1
    DWordFromMemory,        // dfm, s0, s1
    QWordFromMemory,        // qfm, s0, s1

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
    /// Register write location, this cannot be used for anything beyond the register scope.
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

pub enum Errors {
    EndOfStream,
    OperationUnmatched,
    Seek(Error)
}

pub struct SingleParser {
    pub operation: u8,
    pub operands_presence: OperandsPresence
}

impl SingleParser {
    /// Returns an error if failed to parse
    pub fn parse(&self, target: &mut Instruction, source: &mut dyn Read) -> Result<(), Errors> {
        let mut buffer = [0 as u8; 1];

        let mut byte_index = 0;
        let mut bytes_received;

        for _ in 0..MAX_BYTES_EXCLUDING_IMM {
            match byte_index {
                0 => {
                    bytes_received = match source.read(&mut buffer) {
                        Err(_) => return Err(Errors::EndOfStream),
                        Ok(value) => value
                    };
                    if bytes_received == 0 {
                        return Err(Errors::EndOfStream);
                    }

                    if buffer[0] != self.operation {
                        return Err(Errors::OperationUnmatched);
                    }

                    target.operation = buffer[0];
                },
                1 => {
                    if self.operands_presence.destination {
                        bytes_received = match source.read(&mut buffer) {
                            Err(_) => return Err(Errors::EndOfStream),
                            Ok(value) => value
                        };
                        if bytes_received == 0 {
                            return Err(Errors::EndOfStream);
                        }

                        target.destination = Some(buffer[0]);
                    } else {
                        target.destination = None;
                    }
                },
                2 => {
                    if self.operands_presence.source0 {
                        bytes_received = match source.read(&mut buffer) {
                            Err(_) => return Err(Errors::EndOfStream),
                            Ok(value) => value
                        };
                        if bytes_received == 0 {
                            return Err(Errors::EndOfStream);
                        }

                        target.source0 = Some(buffer[0]);
                    } else {
                        target.source0 = None;
                    }
                },
                3 => {
                    if self.operands_presence.source1 {
                        bytes_received = match source.read(&mut buffer) {
                            Err(_) => return Err(Errors::EndOfStream),
                            Ok(value) => value
                        };
                        if bytes_received == 0 {
                            return Err(Errors::EndOfStream);
                        }

                        target.source1 = Some(buffer[0]);
                    } else {
                        target.source1 = None;
                    }
                },
                _ => unreachable!()
            };

            byte_index += 1;
        }

        if let Some(immediate_config) = &self.operands_presence.immediate {
            match immediate_config {
                MultiSizedData::Byte(_) => {
                    let mut immediate_buffer = [0 as u8; BYTE as usize];
                    match source.read(&mut immediate_buffer) {
                        Err(_) => return Err(Errors::EndOfStream),
                        Ok(bytes_read) => {
                            if bytes_read == 0 || bytes_read != BYTE as usize {
                                return Err(Errors::EndOfStream);
                            }
                        }
                    }

                    target.immediate = Some(MultiSizedData::Byte(immediate_buffer[0]));
                },
                MultiSizedData::Word(_) => {
                    let mut immediate_buffer = [0 as u8; WORD as usize];
                    match source.read(&mut immediate_buffer) {
                        Err(_) => return Err(Errors::EndOfStream),
                        Ok(bytes_read) => {
                            if bytes_read == 0 || bytes_read != WORD as usize {
                                return Err(Errors::EndOfStream);
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
                        Err(_) => return Err(Errors::EndOfStream),
                        Ok(bytes_read) => {
                            if bytes_read == 0 || bytes_read != DWORD as usize {
                                return Err(Errors::EndOfStream);
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
                        Err(_) => return Err(Errors::EndOfStream),
                        Ok(bytes_read) => {
                            if bytes_read == 0 || bytes_read != QWORD as usize {
                                return Err(Errors::EndOfStream);
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
        } else {
            target.immediate = None;
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

// TODO: Implement instruction stream with `Read` trait.
pub struct Stream {

}

pub struct Parser {
    singles: Vec<SingleParser>
}

impl Parser {
    pub fn new() -> Self {
        let mut singles = Vec::new();

        singles.push(SingleParser {
            operation: Operations::Nothing as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: false,
                source1: false,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::Terminate as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: false,
                source1: false,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::Interrupt as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: false,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::Safe as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: false,
                immediate: None
            }
        });
        
        singles.push(SingleParser {
            operation: Operations::LoadImmediateByte as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: false,
                source1: false,
                immediate: Some(MultiSizedData::Byte(0))
            }
        });
        singles.push(SingleParser {
            operation: Operations::LoadImmediateWord as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: false,
                source1: false,
                immediate: Some(MultiSizedData::Word(0))
            }
        });
        singles.push(SingleParser {
            operation: Operations::LoadImmediateDWord as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: false,
                source1: false,
                immediate: Some(MultiSizedData::DWord(0))
            }
        });
        singles.push(SingleParser {
            operation: Operations::LoadImmediateQWord as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: false,
                source1: false,
                immediate: Some(MultiSizedData::QWord(0))
            }
        });
        singles.push(SingleParser {
            operation: Operations::LoadInterconnect as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: false,
                immediate: Some(MultiSizedData::QWord(0))
            }
        });
        singles.push(SingleParser {
            operation: Operations::CloneRegister as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: false,
                immediate: None
            }
        });

        singles.push(SingleParser {
            operation: Operations::ByteFromMemory as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::WordFromMemory as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::DWordFromMemory as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::QWordFromMemory as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::ByteToMemory as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::WordToMemory as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::DWordToMemory as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::QWordToMemory as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: true,
                immediate: None
            }
        });

        singles.push(SingleParser {
            operation: Operations::Add as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::AddFloat as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::AddDouble as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::Subtract as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::SubtractFloat as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::SubtractDouble as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::Multiply as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::MultiplyInteger as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::MultiplyFloat as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::MultiplyDouble as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::Divide as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::DivideInteger as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::DivideFloat as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::DivideDouble as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::And as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::Or as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::ExclusiveOr as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::Not as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::ShiftStart as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::ShiftEnd as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        // TODO: Implement trailing zeros when the instruction specs are determined.

        singles.push(SingleParser {
            operation: Operations::Branch as u8,
            operands_presence: OperandsPresence {
                destination: false,
                source0: true,
                source1: false,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::BranchEqual as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::BranchUnequal as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::BranchGreater as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });
        singles.push(SingleParser {
            operation: Operations::BranchGreaterOrEqual as u8,
            operands_presence: OperandsPresence {
                destination: true,
                source0: true,
                source1: true,
                immediate: None
            }
        });

        Self {
            singles
        }
    }

    pub fn parse<Source: Read + Seek>(&self, target: &mut Instruction, source: &mut Source) -> Result<(), Errors> {
        for single in &self.singles {
            let starting_position = match source.stream_position() {
                Ok(pos) => pos,
                Err(err) => return Err(Errors::Seek(err))
            };

            match single.parse(target, source) {
                Err(err) => {
                    match err {
                        Errors::EndOfStream => return Err(err),
                        Errors::OperationUnmatched => {
                            match source.seek(std::io::SeekFrom::Start(starting_position)) {
                                Err(se) => return Err(Errors::Seek(se)),
                                Ok(_) => {}
                            };
                        },
                        Errors::Seek(se) => return Err(Errors::Seek(se))
                    }
                },
                Ok(_) => return Ok(())
            };
        }
        
        Err(Errors::OperationUnmatched)
    }
}