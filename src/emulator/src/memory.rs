use std::{cmp::min, io::{Error, ErrorKind, Read, Seek}, mem::discriminant};

use crate::{core::Permission, instruction::ArchSize};

pub struct Memory {
    memory: Vec<u8>,
    size: ArchSize,
    max_size: Option<ArchSize>,
    pointer: ArchSize
}

impl Memory {
    pub fn new(max_size: Option<ArchSize>) -> Self {
        Self {
            memory: vec![0, 1, 2],
            size: 0,
            max_size,
            pointer: 0
        }
    }
}

impl Read for Memory {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // TODO: Reconsider? Probably not. Remove this. let len = min(buf.len(), self.memory.len() - self.pointer as usize);
        let original_pointer = self.pointer;

        for index in 0..buf.len() {
            match self.memory.get(self.pointer as usize) {
                None => buf[index] = 0,
                Some(value) => {
                    buf[index] = *value;
                    self.pointer += 1;
                }
            }
        }

        Ok((self.pointer - original_pointer) as usize)
    }
}

impl Seek for Memory {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<ArchSize> {
        let old = self.pointer;
        match pos {
            std::io::SeekFrom::Current(rel) => self.pointer = (self.pointer as i64 + rel) as ArchSize,
            std::io::SeekFrom::End(rel) => self.pointer = (self.memory.len() as i64 - rel) as ArchSize,
            std::io::SeekFrom::Start(rel) => self.pointer = rel as ArchSize
        }

        if self.pointer as usize > self.memory.len() - 1 {
            return Err(Error::new(ErrorKind::UnexpectedEof, ""));
        }

        Ok(self.pointer)
    }
}

#[repr(u8)]
pub enum RegisterCodes {    
    Core,                      // 8 bits  - cre
    CurrentInstruction,        // 64 bits - cir
    ArithmeticSideEffect,      // xx bits - artr
    FloatingSideEffect,        // xx bits - fltr
    StackPointer,              // 64 bits - stpr

    // Safe mode
    PageHierarchy, // 64 bits  // 64 bits - phir
    Safe,          // 1 bit    // 1 bit   - safe

    // Binary constants
    True,                      // 1 bit   - true
    False,                     // 1 bit   - flse
    Byte,                      // 3 bits  - byte
    Word,                      // 4 bits  - word
    DoubleWord,                // 5 bits  - dwrd
    QuadWord,                  // 6 bits  - qwrd

    // Interrupt purpose
    Interrupt0,                // 64 bits - int0
    Interrupt1,                // 64 bits - int1
    Interrupt2,                // 64 bits - int2

    // General purpose
    General00,                 // 64 bits - gn00
    General01,                 // 64 bits - gn01
    General02,                 // 64 bits - gn02
    General03,                 // 64 bits - gn03
    General04,                 // 64 bits - gn04
    General05,                 // 64 bits - gn05
    General06,                 // 64 bits - gn06
    General07,                 // 64 bits - gn07
    General08,                 // 64 bits - gn08
    General09,                 // 64 bits - gn09
    General10,                 // 64 bits - gn10
    General11,                 // 64 bits - gn11
    General12,                 // 64 bits - gn12
    General13,                 // 64 bits - gn13
    General14,                 // 64 bits - gn14
    General15,                 // 64 bits - gn15
}

pub struct Register {
    pub identifier: RegisterCodes,

    /// Whether this register allows safe cores to read.
    pub allow_safe_read: bool,
    /// Permission that a core writer has when modifying this register.
    pub core_write: Permission,

    /// USIZE is used because an enum variant would be less memory efficient regardless of the value size
    /// This is public to allow direct writes.
    pub value: usize
}

impl Register {
    /// Returns `Result::Err` if writing is forbidden based on either safety or permission.
    pub fn get_value(&self, safe: bool) -> Option<usize> {
        if !self.allow_safe_read && safe {
            return None;
        }

        Some(self.value)
    }

    /// Returns `Result::Err` if writing is forbidden based on either safety or permission.
    pub fn set_value(&mut self, safe: bool, value: usize) -> Result<(), ()> {
        match self.core_write {
            Permission::None => return Err(()),
            Permission::All => {},
            Permission::NonSafe => {
                if safe {
                    return Err(());
                }
            }
        };

        self.value = value;
        Ok(())
    }
}

pub struct File {
    registers: Vec<Register>
}

impl File {
    pub fn new() -> Self {
        let mut registers = Vec::new();

        registers.push(Register {
            identifier: RegisterCodes::Core,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 0
        });
        registers.push(Register {
            identifier: RegisterCodes::CurrentInstruction,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 0
        });
        registers.push(Register {
            identifier: RegisterCodes::ArithmeticSideEffect,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 0
        });
        registers.push(Register {
            identifier: RegisterCodes::FloatingSideEffect,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 0
        });

        registers.push(Register {
            identifier: RegisterCodes::StackPointer,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 0
        });
        registers.push(Register {
            identifier: RegisterCodes::PageHierarchy,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 0
        });

        registers.push(Register {
            identifier: RegisterCodes::True,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 1
        });
        registers.push(Register {
            identifier: RegisterCodes::False,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 1
        });
        registers.push(Register {
            identifier: RegisterCodes::Byte,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 1
        });
        registers.push(Register {
            identifier: RegisterCodes::Word,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 2
        });
        registers.push(Register {
            identifier: RegisterCodes::DoubleWord,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 4
        });
        registers.push(Register {
            identifier: RegisterCodes::QuadWord,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 8
        });

        registers.push(Register {
            identifier: RegisterCodes::Interrupt0,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::Interrupt1,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::Interrupt2,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });

        registers.push(Register {
            identifier: RegisterCodes::General00,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General01,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General02,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General03,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General04,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General05,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General06,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General07,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General08,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General09,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General10,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General11,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General12,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General13,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General14,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: RegisterCodes::General15,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });        

        File {
            registers
        }
    }

    /// Returns `Result::Err` if the register by the ID specified doesn't exist.
    pub fn find(&self, identifier: RegisterCodes) -> Option<&Register> {
        self.registers.iter().find(|pred| discriminant(&pred.identifier) == discriminant(&identifier))
    }

    /// Returns `Result::Err` if the register by the ID specified doesn't exist.
    pub fn find_mut(&mut self, identifier: RegisterCodes) -> Option<&mut Register> {
        self.registers.iter_mut().find(|pred| discriminant(&pred.identifier) == discriminant(&identifier))
    }
}