use crate::environment::register::{self, Operands, Selector};

pub mod firmware;
pub mod traverser;
pub const NOTHING:                 u8 = 0;
pub const CLONE_REGISTER:          u8 = 1;
pub const BYTE_TO_REGISTER:        u8 = 2;
pub const WORD_TO_REGISTER:        u8 = 3;
pub const DOUBLE_WORD_TO_REGISTER: u8 = 4;
pub const QUAD_WORD_TO_REGISTER:   u8 = 5;
pub const BYTE_TO_MEMORY:          u8 = 6;
pub const WORD_TO_MEMORY:          u8 = 7;
pub const DOUBLE_WORD_TO_MEMORY:   u8 = 8;
pub const QUAD_WORD_TO_MEMORY:     u8 = 9;
pub const BYTE_FROM_MEMORY:        u8 = 10;
pub const WORD_FROM_MEMORY:        u8 = 11;
pub const DOUBLE_WORD_FROM_MEMORY: u8 = 12;
pub const QUAD_WORD_FROM_MEMORY:   u8 = 13;
pub const ADD:                     u8 = 14;
pub const SUBTRACT:                u8 = 15;
pub const MULTIPLY:                u8 = 16;
pub const MULTIPLY_INTEGER:        u8 = 17;
pub const DIVIDE:                  u8 = 18;
pub const DIVIDE_INTEGER:          u8 = 19;
pub const ADD_DOUBLE:              u8 = 20;
pub const ADD_FLOAT:               u8 = 21;
pub const SUBTRACT_DOUBLE:         u8 = 22;
pub const SUBTRACT_FLOAT:          u8 = 23;
pub const MULTIPLY_DOUBLE:         u8 = 24;
pub const MULTIPLY_FLOAT:          u8 = 25;
pub const DIVIDE_DOUBLE:           u8 = 26;
pub const DIVIDE_FLOAT:            u8 = 27;
pub const AND:                     u8 = 28;
pub const OR:                      u8 = 29;
pub const EXCLUSIVE_OR:            u8 = 30;
pub const NOT:                     u8 = 31;
pub const SHIFT_START:             u8 = 32;
pub const SHIFT_END:               u8 = 33;
pub const TRAILING_ZEROS:          u8 = 34;
pub const DIVERT:                  u8 = 35;
pub const DIVERT_TRUE:             u8 = 36;
pub const DIVERT_EQUAL:            u8 = 37;
pub const DIVERT_UNEQUAL:          u8 = 38;
pub const DIVERT_GREATER:          u8 = 39;
pub const DIVERT_GREATER_OR_EQUAL: u8 = 40;

#[derive(Default, Debug, Clone)]
pub enum ImmediatePresence {
    #[default]
    None,
    Byte,
    Word,
    DoubleWord,
    QuadWord
}

impl ImmediatePresence {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => false,
            _ => true
        }
    }

    pub fn get_bytes_count(&self) -> u8 {
        match self {
            Self::None       => 0,
            Self::Byte       => 1,
            Self::Word       => 2,
            Self::DoubleWord => 4,
            Self::QuadWord   => 8
        }
    }
}

pub struct MacroInstruction<Immediate> {
    pub operation:  u8,
    pub register_a: Option<u8>,
    pub register_b: Option<u8>,
    pub immediate:  Option<Immediate>
}

pub enum Errors {
    InvalidRegisterPointer
}

#[derive(Debug, Clone, Default)]
pub enum MicroInstruction {
    #[default]
    Nothing,             
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
    Add                  { target: u8, source: u8 },                    
    Subtract             { target: u8, source: u8 },               
    Multiply             { target: u8, source: u8 },               
    MultiplyInteger      { target: u8, source: u8 },                 
    Divide               { target: u8, source: u8 },                 
    DivideInteger        { target: u8, source: u8 },  
    AddDouble            { target: u8, source: u8 },
    AddFloat             { target: u8, source: u8 },               
    SubtractDouble       { target: u8, source: u8 },
    SubtractFloat        { target: u8, source: u8 },          
    MultiplyDouble       { target: u8, source: u8 },
    MultiplyFloat        { target: u8, source: u8 },          
    DivideDouble         { target: u8, source: u8 },                   
    DivideFloat          { target: u8, source: u8 },            
    And                  { target: u8, source: u8 },                    
    Or                   { target: u8, source: u8 },                     
    ExclusiveOr          { target: u8, source: u8 },            
    Not                  { target: u8, source: u8 },                    
    ShiftStart           { target: u8, source: u8 },             
    ShiftEnd             { target: u8, source: u8 },               
    TrailingZeros        { target: u8, source: u8 },          // tzr, TODO: Undecided
    Divert               { diversion_address: u8 },      
    DivertTrue           { diversion_address: u8, condition: u8 },            
    DivertEqual          { diversion_address: u8, target: u8, source: u8 },            
    DivertUnequal        { diversion_address: u8, target: u8, source: u8 },          
    DivertGreater        { diversion_address: u8, target: u8, source: u8 },          
    DivertGreaterOrEqual { diversion_address: u8, target: u8, source: u8 },   
}

impl MicroInstruction {
    pub fn into_identifier(&self) -> u8 {
        match self {
            Self::Nothing                                                                 => 0,
            Self::CloneRegister            { target_register: _, source_register: _ }     => 1,
            Self::ByteToRegister           { target_register: _, data: _ }                => 2,
            Self::WordToRegister           { target_register: _, data: _ }                => 3,
            Self::DoubleWordToRegister     { target_register: _, data: _ }                => 4,
            Self::QuadWordToRegister       { target_register: _, data: _ }                => 5,        
            Self::ByteToMemory             { target_address: _, source_register: _ }      => 6,
            Self::WordToMemory             { target_address: _, source_register: _ }      => 7,
            Self::DoubleWordToMemory       { target_address: _, source_register: _ }      => 8,
            Self::QuadWordToMemory         { target_address: _, source_register: _ }      => 9,       
            Self::ByteFromMemory           { target_register: _, source_address: _ }      => 10,
            Self::WordFromMemory           { target_register: _, source_address: _ }      => 11,
            Self::DoubleWordFromMemory     { target_register: _, source_address: _ }      => 12,
            Self::QuadWordFromMemory       { target_register: _, source_address: _ }      => 13,       
            Self::Add                      { target: _, source: _ }                       => 14,
            Self::Subtract                 { target: _, source: _ }                       => 15,
            Self::Multiply                 { target: _, source: _ }                       => 16,
            Self::MultiplyInteger          { target: _, source: _ }                       => 17,
            Self::Divide                   { target: _, source: _ }                       => 18,
            Self::DivideInteger            { target: _, source: _ }                       => 19,       
            Self::AddFloat                 { target: _, source: _ }                       => 20,
            Self::AddDouble                { target: _, source: _ }                       => 21,
            Self::SubtractFloat            { target: _, source: _ }                       => 22,
            Self::SubtractDouble           { target: _, source: _ }                       => 23,        
            Self::MultiplyFloat            { target: _, source: _ }                       => 24,
            Self::MultiplyDouble           { target: _, source: _ }                       => 25,
            Self::DivideFloat              { target: _, source: _ }                       => 26,
            Self::DivideDouble             { target: _, source: _ }                       => 27,      
            Self::And                      { target: _, source: _ }                       => 28,
            Self::Or                       { target: _, source: _ }                       => 29,
            Self::ExclusiveOr              { target: _, source: _ }                       => 30,
            Self::Not                      { target: _, source: _ }                       => 31,
            Self::ShiftStart               { target: _, source: _ }                       => 32,
            Self::ShiftEnd                 { target: _, source: _ }                       => 33,
            Self::TrailingZeros            { target: _, source: _ }                       => 34,   
            Self::Divert                   { diversion_address: _ }                       => 35,        
            Self::DivertTrue               { diversion_address: _, condition: _ }         => 36,
            Self::DivertEqual              { diversion_address: _, target: _, source: _ } => 37,
            Self::DivertUnequal            { diversion_address: _, target: _, source: _ } => 38,
            Self::DivertGreater            { diversion_address: _, target: _, source: _ } => 39,
            Self::DivertGreaterOrEqual     { diversion_address: _, target: _, source: _ } => 40
        }     
    }

    pub fn from(identifier: u8, register_a: u8, register_b: u8, immediate: u64) -> Result<MicroInstruction, ()> {
        Ok(match identifier {
            NOTHING                 => MicroInstruction::Nothing,
            CLONE_REGISTER          => MicroInstruction::CloneRegister        { target_register: register_a, source_register: register_b },
            BYTE_TO_REGISTER        => MicroInstruction::ByteToRegister       { target_register: register_a, data: immediate as u8 },
            WORD_TO_REGISTER        => MicroInstruction::WordToRegister       { target_register: register_a, data: immediate as u16 },
            DOUBLE_WORD_TO_REGISTER => MicroInstruction::DoubleWordToRegister { target_register: register_a, data: immediate as u32 },
            QUAD_WORD_TO_REGISTER   => MicroInstruction::QuadWordToRegister   { target_register: register_a, data: immediate as u64 },
            BYTE_TO_MEMORY          => MicroInstruction::ByteToMemory         { target_address: immediate, source_register: register_a },      
            WORD_TO_MEMORY          => MicroInstruction::WordToMemory         { target_address: immediate, source_register: register_a },         
            DOUBLE_WORD_TO_MEMORY   => MicroInstruction::DoubleWordToMemory   { target_address: immediate, source_register: register_a },
            QUAD_WORD_TO_MEMORY     => MicroInstruction::QuadWordToMemory     { target_address: immediate, source_register: register_a }, 
            BYTE_FROM_MEMORY        => MicroInstruction::ByteFromMemory       { target_register: register_a, source_address: immediate },       
            WORD_FROM_MEMORY        => MicroInstruction::WordFromMemory       { target_register: register_a, source_address: immediate },       
            DOUBLE_WORD_FROM_MEMORY => MicroInstruction::DoubleWordFromMemory { target_register: register_a, source_address: immediate },
            QUAD_WORD_FROM_MEMORY   => MicroInstruction::QuadWordFromMemory   { target_register: register_a, source_address: immediate },  
            ADD                     => MicroInstruction::Add                  { target: register_a, source: register_b },         
            SUBTRACT                => MicroInstruction::Subtract             { target: register_a, source: register_b },
            MULTIPLY                => MicroInstruction::Multiply             { target: register_a, source: register_b },
            MULTIPLY_INTEGER        => MicroInstruction::MultiplyInteger      { target: register_a, source: register_b }, 
            DIVIDE                  => MicroInstruction::Divide               { target: register_a, source: register_b }, 
            DIVIDE_INTEGER          => MicroInstruction::DivideInteger        { target: register_a, source: register_b }, 
            ADD_DOUBLE              => MicroInstruction::AddDouble            { target: register_a, source: register_b }, 
            ADD_FLOAT               => MicroInstruction::AddFloat             { target: register_a, source: register_b },
            SUBTRACT_DOUBLE         => MicroInstruction::SubtractDouble       { target: register_a, source: register_b },
            SUBTRACT_FLOAT          => MicroInstruction::SubtractFloat        { target: register_a, source: register_b },
            MULTIPLY_DOUBLE         => MicroInstruction::MultiplyDouble       { target: register_a, source: register_b },
            MULTIPLY_FLOAT          => MicroInstruction::MultiplyFloat        { target: register_a, source: register_b },
            DIVIDE_DOUBLE           => MicroInstruction::DivideDouble         { target: register_a, source: register_b },
            DIVIDE_FLOAT            => MicroInstruction::DivideFloat          { target: register_a, source: register_b },
            AND                     => MicroInstruction::And                  { target: register_a, source: register_b },
            OR                      => MicroInstruction::Or                   { target: register_a, source: register_b },
            EXCLUSIVE_OR            => MicroInstruction::ExclusiveOr          { target: register_a, source: register_b },
            NOT                     => MicroInstruction::Not                  { target: register_a, source: register_b },
            SHIFT_START             => MicroInstruction::ShiftStart           { target: register_a, source: register_b },
            SHIFT_END               => MicroInstruction::ShiftEnd             { target: register_a, source: register_b },
            TRAILING_ZEROS          => MicroInstruction::TrailingZeros        { target: register_a, source: register_b },
            DIVERT                  => MicroInstruction::Divert               { diversion_address: immediate as u8 },
            DIVERT_TRUE             => MicroInstruction::DivertTrue           { diversion_address: immediate as u8, condition: register_a },
            DIVERT_EQUAL            => MicroInstruction::DivertEqual          { diversion_address: immediate as u8, target: register_a, source: register_b },
            DIVERT_UNEQUAL          => MicroInstruction::DivertUnequal        { diversion_address: immediate as u8, target: register_a, source: register_b },
            DIVERT_GREATER          => MicroInstruction::DivertGreater        { diversion_address: immediate as u8, target: register_a, source: register_b },
            DIVERT_GREATER_OR_EQUAL => MicroInstruction::DivertGreaterOrEqual { diversion_address: immediate as u8, target: register_a, source: register_b },
            _ => return Err(())
        })
    }

    pub fn into_bytes(&self) -> Result<Vec<u8>, Selector> {
        let mut bytes = Vec::from([ self.into_identifier() ]);
        let fallback_register = 0;

        let mut register_operands: Option<Result<Operands, register::Selector>> = None;

        let mut immediate_byte:        Option<u8>  = None;
        let mut immediate_word:        Option<u16> = None;
        let mut immediate_double_word: Option<u32> = None;
        let mut immediate_quad_word:   Option<u64> = None;

        match self {
            Self::Nothing => (),
            Self::CloneRegister { target_register, source_register } => {
                register_operands = Some(register::Operands::new(target_register.clone(), source_register.clone()));
            },
            Self::ByteToRegister { target_register, data } => {
                register_operands = Some(register::Operands::new(target_register.clone(), fallback_register));
                immediate_byte = Some(*data);
            }
            _ => todo!() // TODO
        }

        match register_operands {
            None => (),
            Some(operands) => {
                let ok = match operands {
                    Err(error) => return Err(error),
                    Ok(result) => result 
                };

                bytes.push(ok.into_byte())
            }
        }

        if let Some(data) = immediate_byte {
            bytes.push(data);
        } else if let Some(data) = immediate_word {
            bytes.extend(u16::to_le_bytes(data))
        } else if let Some(data) = immediate_double_word {
            bytes.extend(u32::to_le_bytes(data))
        } else if let Some(data) = immediate_quad_word {
            bytes.extend(u64::to_le_bytes(data))
        }

        Ok(bytes)
    }   
}