use crate::environment::register::{self, Register};

pub mod firmware;

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub enum RegisterPresence {
    #[default]
    None,
    Ab,
    A,
}

impl RegisterPresence {
    pub fn from(a: bool, b: bool) -> RegisterPresence {
        if a && b {
            RegisterPresence::Ab
        } else if a ^ b {
            RegisterPresence::A
        } else {
            RegisterPresence::None
        }
    }

    pub fn get_bytes_count(&self) -> u8 {
        match self {
            Self::None => 0,
            _ => 1
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

#[derive(Debug, Clone)]
pub enum MicroInstruction {
    Nothing,             

    // Data flow
    CloneRegister        { target_register: Register, source_register: Register }, 

    ByteToRegister       { target_register: Register, data: u8 },      
    WordToRegister       { target_register: Register, data: u16 },     
    DoubleWordToRegister { target_register: Register, data: u32 },    
    QuadWordToRegister   { target_register: Register, data: u64 },    

    ByteToMemory         { target_address: u64, source_register: Register },   
    WordToMemory         { target_address: u64, source_register: Register },   
    DoubleWordToMemory   { target_address: u64, source_register: Register },
    QuadWordToMemory     { target_address: u64, source_register: Register }, 

    ByteFromMemory       { target_register: Register, source_address: u64 }, 
    WordFromMemory       { target_register: Register, source_address: u64 }, 
    DoubleWordFromMemory { target_register: Register, source_address: u64 }, 
    QuadWordFromMemory   { target_register: Register, source_address: u64 }, 

    // Arithmetic
    Add                  { register_a: Register, register_b: Register },                    
    Subtract             { register_a: Register, register_b: Register },               
    Multiply             { register_a: Register, register_b: Register },               
    MultiplyInteger      { register_a: Register, register_b: Register },                 
    Divide               { register_a: Register, register_b: Register },                 
    DivideInteger        { register_a: Register, register_b: Register },  

    // Float
    AddFloat             { register_a: Register, register_b: Register },               
    AddDouble            { register_a: Register, register_b: Register },
    SubtractFloat        { register_a: Register, register_b: Register },          
    SubtractDouble       { register_a: Register, register_b: Register },

    MultiplyFloat        { register_a: Register, register_b: Register },          
    MultiplyDouble       { register_a: Register, register_b: Register },
    DivideFloat          { register_a: Register, register_b: Register },            
    DivideDouble         { register_a: Register, register_b: Register },                   

    // Logic
    And                  { register_a: Register, register_b: Register },                    
    Or                   { register_a: Register, register_b: Register },                     
    ExclusiveOr          { register_a: Register, register_b: Register },            
    Not                  { register_a: Register, register_b: Register },                    
    ShiftStart           { register_a: Register, register_b: Register },             
    ShiftEnd             { register_a: Register, register_b: Register },               
    TrailingZeros        { register_a: Register, register_b: Register },          // tzr, TODO: Undecided

    // Position diversion
    Divert               { diversion_address: u64 },      

    DivertTrue           { diversion_address: u64, condition: Register },            
    DivertEqual          { diversion_address: u64, register_a: Register, register_b: Register },            
    DivertUnequal        { diversion_address: u64, register_a: Register, register_b: Register },          
    DivertGreater        { diversion_address: u64, register_a: Register, register_b: Register },          
    DivertGreaterOrEqual { diversion_address: u64, register_a: Register, register_b: Register },   
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
            Self::DivertGreaterOrEqual     { diversion_address: _, register_a: _, register_b: _ } => 40,

            
        }     
    }

    pub fn from(identifier: u8, register_a: Register, register_b: Register, immediate: u64) -> MicroInstruction {
        match identifier {
            0 => MicroInstruction::Nothing,
            1 => MicroInstruction::CloneRegister { 
                target_register: register_a, 
                source_register: register_b
            },
            2 => MicroInstruction::ByteToRegister { 
                target_register: register_a, 
                data: immediate as u8
            },
            3 => MicroInstruction::WordToRegister { 
                target_register: register_a, 
                data: immediate as u16
            },
            4 => MicroInstruction::DoubleWordToRegister { 
                target_register: register_a, 
                data: immediate as u32
            },
            5 => MicroInstruction::QuadWordToRegister { 
                target_register: register_a, 
                data: immediate as u64
            },
            
            _ => todo!() // TODO
        }
    }

    pub fn into_bytes(&self) -> Result<Vec<u8>, ()> {
        let mut bytes = Vec::from([ self.into_identifier() ]);
        let fallback_register = Register::Void;

        let mut register_operands = Option::from(register::Operands { 
            register_a: fallback_register.clone(), 
            register_b: fallback_register.clone() 
        });

        let mut immediate_byte:        Option<u8>  = None;
        let mut immediate_word:        Option<u16> = None;
        let mut immediate_double_word: Option<u32> = None;
        let mut immediate_quad_word:   Option<u64> = None;

        match self {
            Self::Nothing => (),
            Self::CloneRegister { target_register, source_register } => {
                register_operands = Some(register::Operands { 
                    register_a: target_register.clone(),
                    register_b: source_register.clone()
                });
            },
            Self::ByteToRegister { target_register, data } => {
                register_operands = Some(register::Operands {
                    register_a: target_register.clone(),
                    register_b: fallback_register
                });

                immediate_byte = Some(*data);
            }
            _ => todo!() // TODO
        }

        match register_operands {
            None => (),
            Some(operands) => bytes.push(operands.into_byte())
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