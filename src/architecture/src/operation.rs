/// Most function is register based.
pub enum Codes {
    Terminate,

    // Data
    LoadImmediate,
    LoadMemory,
    CloneRegister,

    // Arithmetic
    Add,
    Subtract,
    Multiply, // Needs more research and testing
    Divide,   // Needs more research and testing,

    // Interrupt
    

    // Security
    StartProtected
}