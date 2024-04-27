pub mod decode;
pub mod encode;
pub mod interrupt;

pub const BYTE: u8 = 1;
pub const WORD: u8 = 2;
pub const DWORD: u8 = 4;
pub const QWORD: u8 = 8;

#[deprecated]
pub enum RiscOperation {
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

    // Position diversion
    Divert,                 // dvt, s0
    DivertEqual,            // deq, s0, s1, s2
    DivertUnequal,          // duq, s0, s1, s2
    DivertGreater,          // dgr, s0, s1, s2
    DivertGreaterOrEqual,   // dge, s0, s1, s2
}

pub struct ClassARegisterOperand {
    pub destination: u8,
    pub first: u8,
    pub second: u8
}

pub struct ClassBRegisterOperand {
    pub first: u8,
    pub second: u8
}

pub struct ClassCRegisterOperand {
    pub first: u8
}

pub struct ClassDRegisterOperand {
    pub destination: u8
}

pub struct ClassERegisterOperand {
    pub destination: u8,
    pub first: u8
}

pub enum MacroOperation {
    Nothing,
    Terminate,

    /// Stop the current task and start executing another program
    /// in unsafe mode. Once the interrupt completes then it will
    /// return back to the next address after the initial interrupt
    /// call.
    Interrupt { code: u8 },
    
    /// Enter safe mode in the process and divert to a different
    /// address. Once this completes it will return to the next
    /// instruction after the safe call.
    Safe { divert_location: u64 },

    /// Load a byte sized immediate value from the instruction into 
    /// a register.
    LoadImmediateByte { target: u8, value: u8 },     

    /// Load a word sized immediate value from the instruction into 
    /// a register.
    LoadImmediateWord { target: u8, value: u16 },  

    /// Load a double word sized immediate value from the instruction 
    /// into a register.
    LoadImmediateDWord { target: u8, value: u32 },   

    /// Load a quad word sized immediate value from the instruction 
    /// into a register.
    LoadImmediateQWord { target: u8, value: u64 },          
    
    /// Load a value onto the output interconnect to be read externally. 
    LoadInterconnectOut { value: u64 },       

    /// Read a value from the input interconnect and store it into
    /// a register.
    LoadInterconnectIn { target: u8 },

    /// Clone the value from a register and insert it into a register.
    CloneRegister { target: u8, source: u8 }      
}

pub enum MicroOperation {
    
}