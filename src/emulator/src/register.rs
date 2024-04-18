pub enum Code {    
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

// pub struct Register {
//     code: Code,
//     allow_safe_mode_read: bool,
//     allow_safe_mode_write: bool,

// }

// impl Register {
//     pub fn new(code: Code) -> Self {
//         Register {
//             code
//         }
//     }
// }