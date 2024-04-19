use std::mem::discriminant;

use crate::core::{Permission};

#[repr(u8)]
pub enum Codes {    
    Core,                      // 8 bits  - cre
    CurrentInstruction,        // 64 bits - cir
    ArithmeticSideEffect,          // xx bits - artr
    FloatingSideEffect,            // xx bits - fltr
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
    pub identifier: Codes,

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
            identifier: Codes::Core,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 0
        });
        registers.push(Register {
            identifier: Codes::CurrentInstruction,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 0
        });
        registers.push(Register {
            identifier: Codes::ArithmeticSideEffect,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 0
        });
        registers.push(Register {
            identifier: Codes::FloatingSideEffect,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 0
        });

        registers.push(Register {
            identifier: Codes::StackPointer,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 0
        });
        registers.push(Register {
            identifier: Codes::PageHierarchy,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 0
        });

        registers.push(Register {
            identifier: Codes::True,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 1
        });
        registers.push(Register {
            identifier: Codes::False,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 1
        });
        registers.push(Register {
            identifier: Codes::Byte,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 1
        });
        registers.push(Register {
            identifier: Codes::Word,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 2
        });
        registers.push(Register {
            identifier: Codes::DoubleWord,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 4
        });
        registers.push(Register {
            identifier: Codes::QuadWord,
            allow_safe_read: true,
            core_write: Permission::None,
            value: 8
        });

        registers.push(Register {
            identifier: Codes::Interrupt0,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::Interrupt1,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::Interrupt2,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });

        registers.push(Register {
            identifier: Codes::General00,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General01,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General02,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General03,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General04,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General05,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General06,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General07,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General08,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General09,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General10,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General11,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General12,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General13,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General14,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });
        registers.push(Register {
            identifier: Codes::General15,
            allow_safe_read: true,
            core_write: Permission::All,
            value: 8
        });        

        File {
            registers
        }
    }

    /// Returns `Result::Err` if the register by the ID specified doesn't exist.
    pub fn find(&self, identifier: Codes) -> Option<&Register> {
        self.registers.iter().find(|pred| discriminant(&pred.identifier) == discriminant(&identifier))
    }

    /// Returns `Result::Err` if the register by the ID specified doesn't exist.
    pub fn find_mut(&mut self, identifier: Codes) -> Option<&mut Register> {
        self.registers.iter_mut().find(|pred| discriminant(&pred.identifier) == discriminant(&identifier))
    }
}