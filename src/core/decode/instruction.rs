/// This instruction is indeterminate and dynamic. The behavior 
/// and parameters are determined by the firmware.
pub struct MacroInstruction<Immediate> {
    operation:  u8,
    register_a: Option<u8>,
    register_b: Option<u8>,
    immediate:  Option<Immediate>
}

pub enum MicroInstruction {
    Nothing,             

    // Data flow
    CloneRegister        { target_register: u8, source_register: u8 }, 

    ByteToRegister       { target_register: u8, data: u8 },      
    WordToRegister       { target_register: u8, data: u16 },     
    DoubleWordToRegister { target_register: u8, data: u32 },    
    QuadWordToRegister   { target_register: u8, data: u64 },    

    ByteToMemory         { target_address: u64, source_register: u8 },   
    WordToMemory         { target_address: u64, source_register: u8 },   
    DoubleWordToMemory   { target_address: u64, source_register: u8 },
    QuadWordToMemory     { target_address: u64, source_register: u8 }, 

    ByteFromMemory       { target_register: u8, source_address: u64 }, 
    WordFromMemory       { target_register: u8, source_address: u64 }, 
    DoubleWordFromMemory { target_register: u8, source_address: u64 }, 
    QuadWordFromMemory   { target_register: u8, source_address: u64 }, 

    // Arithmetic
    Add                  { register_a: u8, register_b: u8 },                    
    Subtract             { register_a: u8, register_b: u8 },               
    Multiply             { register_a: u8, register_b: u8 },               
    MultiplyInteger      { register_a: u8, register_b: u8 },                 
    Divide               { register_a: u8, register_b: u8 },                 
    DivideInteger        { register_a: u8, register_b: u8 },  

    // Float
    AddFloat             { register_a: u8, register_b: u8 },               
    AddDouble            { register_a: u8, register_b: u8 },
    SubtractFloat        { register_a: u8, register_b: u8 },          
    SubtractDouble       { register_a: u8, register_b: u8 },

    MultiplyFloat        { register_a: u8, register_b: u8 },          
    MultiplyDouble       { register_a: u8, register_b: u8 },
    DivideFloat          { register_a: u8, register_b: u8 },            
    DivideDouble         { register_a: u8, register_b: u8 },                   

    // Logic
    And                  { register_a: u8, register_b: u8 },                    
    Or                   { register_a: u8, register_b: u8 },                     
    ExclusiveOr          { register_a: u8, register_b: u8 },            
    Not                  { register_a: u8, register_b: u8 },                    
    ShiftStart           { register_a: u8, register_b: u8 },             
    ShiftEnd             { register_a: u8, register_b: u8 },               
    TrailingZeros        { register_a: u8, register_b: u8 },          // tzr, TODO: Undecided

    // Position diversion
    Divert               { diversion_address: u64 },      

    DivertTrue           { diversion_address: u64, condition: u8 },            
    DivertEqual          { diversion_address: u64, register_a: u8, register_b: u8 },            
    DivertUnequal        { diversion_address: u64, register_a: u8, register_b: u8 },          
    DivertGreater        { diversion_address: u64, register_a: u8, register_b: u8 },          
    DivertGreaterOrEqual { diversion_address: u64, register_a: u8, register_b: u8 },   
}

impl MicroInstruction {
    pub fn into_identifier(&self) -> u8 {
        match self {
            Self::Nothing                                                                         => 0,
 
            Self::CloneRegister            { target_register: _, source_register: _ }             => 1,
            
            Self::ByteToRegister           { target_register: _, data: _ }                        => 2,
            Self::WordToRegister           { target_register: _, data: _ }                        => 3,
            Self::DoubleWordToRegister     { target_register: _, data: _ }                        => 4,
            Self::QuadWordToRegister       { target_register: _, data: _ }                        => 5,
        
            Self::ByteToMemory             { target_address: _, source_register: _ }              => 6,
            Self::WordToMemory             { target_address: _, source_register: _ }              => 7,
            Self::DoubleWordToMemory       { target_address: _, source_register: _ }              => 8,
            Self::QuadWordToMemory         { target_address: _, source_register: _ }              => 9,
        
            Self::ByteFromMemory           { target_register: _, source_address: _ }              => 10,
            Self::WordFromMemory           { target_register: _, source_address: _ }              => 11,
            Self::DoubleWordFromMemory     { target_register: _, source_address: _ }              => 12,
            Self::QuadWordFromMemory       { target_register: _, source_address: _ }              => 13,
        
            Self::Add                      { register_a: _, register_b: _ }                       => 14,
            Self::Subtract                 { register_a: _, register_b: _ }                       => 15,
            Self::Multiply                 { register_a: _, register_b: _ }                       => 16,
            Self::MultiplyInteger          { register_a: _, register_b: _ }                       => 17,
            Self::Divide                   { register_a: _, register_b: _ }                       => 18,
            Self::DivideInteger            { register_a: _, register_b: _ }                       => 19,
        
            Self::AddFloat                 { register_a: _, register_b: _ }                       => 20,
            Self::AddDouble                { register_a: _, register_b: _ }                       => 21,
            Self::SubtractFloat            { register_a: _, register_b: _ }                       => 22,
            Self::SubtractDouble           { register_a: _, register_b: _ }                       => 23,
            
            Self::MultiplyFloat            { register_a: _, register_b: _ }                       => 24,
            Self::MultiplyDouble           { register_a: _, register_b: _ }                       => 25,
            Self::DivideFloat              { register_a: _, register_b: _ }                       => 26,
            Self::DivideDouble             { register_a: _, register_b: _ }                       => 27,
        
            Self::And                      { register_a: _, register_b: _ }                       => 28,
            Self::Or                       { register_a: _, register_b: _ }                       => 29,
            Self::ExclusiveOr              { register_a: _, register_b: _ }                       => 30,
            Self::Not                      { register_a: _, register_b: _ }                       => 31,
            Self::ShiftStart               { register_a: _, register_b: _ }                       => 32,
            Self::ShiftEnd                 { register_a: _, register_b: _ }                       => 33,
            Self::TrailingZeros            { register_a: _, register_b: _ }                       => 34,
        
            Self::Divert                   { diversion_address: _ }                               => 35,
            
            Self::DivertTrue               { diversion_address: _, condition: _ }                 => 36,
            Self::DivertEqual              { diversion_address: _, register_a: _, register_b: _ } => 37,
            Self::DivertUnequal            { diversion_address: _, register_a: _, register_b: _ } => 38,
            Self::DivertGreater            { diversion_address: _, register_a: _, register_b: _ } => 39,
            Self::DivertGreaterOrEqual     { diversion_address: _, register_a: _, register_b: _ } => 40
        }        
    }
}