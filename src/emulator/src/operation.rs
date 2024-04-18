pub enum Code {
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