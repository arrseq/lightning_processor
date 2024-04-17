pub enum Code {    
    CurrentInstruction,
    ArithmeticLogicUnitResult,
    FloatingPointUnitResult,
    StackPointer,

    // Safe mode
    PageHierarchy,      // 64 bits   //
    Safe,                  // 1 bit     //

    // Binary constants
    True,
    False,
    Byte,
    Word,
    DoubleWord,
    QuadWord,

    // General purpose
    General0,           // 64 bits   //
    General1,           // 64 bits   //
    General2,           // 64 bits   //

    General4,           // 64 bits   //
    General5,           // 64 bits   //
    General6,           // 64 bits   //

    General7,           // 64 bits   //
    General8,           // 64 bits   //
    General9,           // 64 bits   //

    General10,          // 64 bits   //
    General11,          // 64 bits   //
    General12,          // 64 bits   //

    General13,          // 64 bits   //
    General14,          // 64 bits   //
    General15,          // 64 bits   //
}

pub struct Register {
    code: Code,
    allow_safe_mode_read: bool,
    allow_safe_mode_write: bool,

}

impl Register {
    pub fn new(code: Code) -> Self {
        Register {
            code
        }
    }
}