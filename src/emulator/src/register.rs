use crate::core::Core;

#[repr(u8)]
pub enum Codes {    
    Core,                      // 8 bits  - cre
    CurrentInstruction,        // 64 bits - cir
    ArithmeticResult,          // xx bits - artr
    FloatingResult,            // xx bits - fltr
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

    General04,                 // 64 bits - gn03
    General05,                 // 64 bits - gn04
    General06,                 // 64 bits - gn05

    General07,                 // 64 bits - gn06
    General08,                 // 64 bits - gn07
    General09,                 // 64 bits - gn08

    General10,                 // 64 bits - gn09
    General11,                 // 64 bits - gn10
    General12,                 // 64 bits - gn11

    General13,                 // 64 bits - gn12
    General14,                 // 64 bits - gn13
    General15,                 // 64 bits - gn14
}

pub struct Register {
    /// Whether this register allows safe cores to read.
    pub allow_safe_read: bool,
    /// Whether this register allows safe cores to write.
    pub allow_safe_write: bool,

    pub identifier: Codes,

    // USIZE is used because an enum variant would be less memory efficient regardless of the value size
    value: usize
}

impl Register {
    /// Returns `Result::Err` if the core is safe while this register rejects safe reads.
    pub fn get_value(&mut self, core: &Core) -> Result<usize, ()> {
        if !self.allow_safe_read && core.is_safe() {
            return Err(());
        }

        Ok(self.value)
    }

    /// Returns `Result::Err` if the core is safe while this register rejects safe writes.
    pub fn set_value(&mut self, core: &Core, value: usize) -> Result<(), ()> {
        if !self.allow_safe_read && core.is_safe() {
            return Err(());
        }

        self.value = value;
        Ok(())
    }
}

pub struct File {
    registers: Vec<Register>
}

impl File {
    pub fn new() -> Self {
        File {
            registers: Vec::new()
        }
    }
}