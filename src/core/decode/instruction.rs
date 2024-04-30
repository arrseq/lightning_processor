/// This instruction is indeterminate and dynamic. The behavior 
/// and parameters are determined by the firmware.
pub struct MacroInstruction<Immediate> {
    operation: u8,
    register_a: Option<u8>,
    register_b: Option<u8>,
    immediate: Option<Immediate>
}

pub enum MicroInstruction {
    // Data flow
    CloneRegister { target_register: u8, source_register: u8 }, 

    ByteToRegister { target_register: u8, data: u8 },      
    WordToRegister { target_register: u8, data: u16 },     
    DoubleWordToRegister { target_register: u8, data: u32 },    
    QuadWordToRegister { target_register: u8, data: u64 },    

    ByteToMemory { target_address: u64, source_register: u8 },   
    WordToMemory { target_address: u64, source_register: u8 },   
    DoubleWordToMemory { target_address: u64, source_register: u8 },
    QuadWordToMemory { target_address: u64, source_register: u8 }, 

    ByteFromMemory { target_register: u8, source_address: u64 }, 
    WordFromMemory { target_register: u8, source_address: u64 }, 
    DoubleWordFromMemory { target_register: u8, source_address: u64 }, 
    QuadWordFromMemory { target_register: u8, source_address: u64 }, 

    // Arithmetic
    Add { register_a: u8, register_b: u8 },                    
    Subtract { register_a: u8, register_b: u8 },               
    Multiply { register_a: u8, register_b: u8 },               
    MultiplyInteger { register_a: u8, register_b: u8 },                 
    Divide { register_a: u8, register_b: u8 },                 
    DivideInteger { register_a: u8, register_b: u8 },  

    // Float
    AddFloat { register_a: u8, register_b: u8 },               
    AddDouble { register_a: u8, register_b: u8 },
    SubtractFloat { register_a: u8, register_b: u8 },          
    SubtractDouble { register_a: u8, register_b: u8 },

    MultiplyFloat { register_a: u8, register_b: u8 },          
    MultiplyDouble { register_a: u8, register_b: u8 },
    DivideFloat { register_a: u8, register_b: u8 },            
    DivideDouble { register_a: u8, register_b: u8 },                   

    // Logic
    And { register_a: u8, register_b: u8 },                    
    Or { register_a: u8, register_b: u8 },                     
    ExclusiveOr { register_a: u8, register_b: u8 },            
    Not { register_a: u8, register_b: u8 },                    
    ShiftStart { register_a: u8, register_b: u8 },             
    ShiftEnd { register_a: u8, register_b: u8 },               
    TrailingZeros { register_a: u8, register_b: u8 },          // tzr, TODO: Undecided

    // Position diversion
    Divert { diversion_address: u64 },      

    DivertTrue { diversion_address: u64, condition: u8 },            
    DivertEqual { diversion_address: u64, register_a: u8, register_b: u8 },            
    DivertUnequal { diversion_address: u64, register_a: u8, register_b: u8 },          
    DivertGreater { diversion_address: u64, register_a: u8, register_b: u8 },          
    DivertGreaterOrEqual { diversion_address: u64, register_a: u8, register_b: u8 },   
}

impl MicroInstruction {
    pub fn into_identifier(&self) -> Option<u8> {
        match self {
            Self::CloneRegister { target_register, source_register }                       => Some(0),
            
            Self::ByteToRegister { target_register, data }                                 => Some(1),
            Self::WordToRegister { target_register, data }                                => Some(2),
            Self::DoubleWordToRegister { target_register, data }                          => Some(3),
            Self::QuadWordToRegister { target_register, data }                            => Some(4),

            Self::ByteToMemory { target_address, source_register }                        => Some(5),
            Self::WordToMemory { target_address, source_register }                        => Some(6),
            Self::DoubleWordToMemory { target_address, source_register }                  => Some(7),
            Self::QuadWordToMemory { target_address, source_register }                    => Some(8),

            Self::ByteFromMemory { target_register, source_address }                      => Some(9),
            Self::WordFromMemory { target_register, source_address }                      => Some(10),
            Self::DoubleWordFromMemory { target_register, source_address }                => Some(11),
            Self::QuadWordFromMemory { target_register, source_address }                  => Some(12),

            Self::Add { register_a, register_b }                                           => Some(13),
            Self::Subtract { register_a, register_b }                                      => Some(14),
            Self::Multiply { register_a, register_b }                                      => Some(15),
            Self::MultiplyInteger { register_a, register_b }                               => Some(16),
            Self::Divide { register_a, register_b }                                        => Some(17),
            Self::DivideInteger { register_a, register_b }                                 => Some(18),

            Self::AddFloat { register_a, register_b }                                      => Some(19),
            Self::AddDouble { register_a, register_b }                                     => Some(20),
            Self::SubtractFloat { register_a, register_b }                                 => Some(21),
            Self::SubtractDouble { register_a, register_b }                                => Some(22),
            
            Self::MultiplyFloat { register_a, register_b }                                 => Some(23),
            Self::MultiplyDouble { register_a, register_b }                                => Some(24),
            Self::DivideFloat { register_a, register_b }                                   => Some(25),
            Self::DivideDouble { register_a, register_b }                                  => Some(26),

            Self::And { register_a, register_b }                                           => Some(27),
            Self::Or { register_a, register_b }                                            => Some(28),
            Self::ExclusiveOr { register_a, register_b }                                   => Some(29),
            Self::Not { register_a, register_b }                                           => Some(30),
            Self::ShiftStart { register_a, register_b }                                    => Some(31),
            Self::ShiftEnd { register_a, register_b }                                      => Some(32),
            Self::TrailingZeros { register_a, register_b }                                 => Some(33),

            Self::Divert { diversion_address }                                                 => Some(34),
            
            Self::DivertTrue { diversion_address, condition }                             => Some(35),
            Self::DivertEqual { diversion_address, register_a, register_b }          => Some(36),
            Self::DivertUnequal { diversion_address, register_a, register_b }        => Some(37),
            Self::DivertGreater { diversion_address, register_a, register_b }        => Some(38),
            Self::DivertGreaterOrEqual { diversion_address, register_a, register_b } => Some(39),

            _ => None
        }
    }
}