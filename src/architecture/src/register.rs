pub enum Codes {    
    CurrentInstructionAddress,
    ArithmeticLogicUnitResultFlags,
    StackPointer,

    // Safe mode
    PageHierarchyAddress,      // 64 bits   //
    SafeMode,                  // 1 bit     //

    // Binary constants
    True,
    False,
    Byte,
    Word,
    DoubleWord,
    QuadWord,

    // General purpose
    GeneralPurpose0,           // 64 bits   //
    GeneralPurpose1,           // 64 bits   //
    GeneralPurpose2,           // 64 bits   //

    GeneralPurpose4,           // 64 bits   //
    GeneralPurpose5,           // 64 bits   //
    GeneralPurpose6,           // 64 bits   //

    GeneralPurpose7,           // 64 bits   //
    GeneralPurpose8,           // 64 bits   //
    GeneralPurpose9,           // 64 bits   //

    GeneralPurpose10,          // 64 bits   //
    GeneralPurpose11,          // 64 bits   //
    GeneralPurpose12,          // 64 bits   //

    GeneralPurpose13,          // 64 bits   //
    GeneralPurpose14,          // 64 bits   //
    GeneralPurpose15,          // 64 bits   //
}

pub struct Register {

}