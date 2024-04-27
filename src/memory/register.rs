/// These registers are available to everything.
pub enum MacroRegisters {
    StackPointer(u64),
    BasePointer(u64)
}

/// These registers are for use by the microcode engine.
pub enum MicroRegisters {
    Register0(u64),
    Register1(u64),
    Register2(u64),
    Register3(u64),
    Register4(u64),
    Register5(u64),
    Register6(u64),
    Register7(u64)
}