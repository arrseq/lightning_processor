pub enum Registers {    
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