use self::register::Register;
use self::dynamic::Dynamic;
use utility::{Encode, TryCoded};

pub mod dynamic;
pub mod register;
pub mod registers;

pub struct Instruction<OpCode: TryCoded, PrCode: TryCoded, > {
    pub prefixes: Vec<PrCode>,
    pub operation: OpCode,
    pub static_operand: Option<Register>,
    pub dynamic_operand: Option<Dynamic>
}

impl<OpCode: TryCoded, PrCode: TryCoded> Encode for Instruction<OpCode, PrCode> {
    type Output = Vec<u8>;

    fn encode(&self) -> Self::Output {
        todo!()
    }
}