use self::register::Register;
use self::dynamic::Dynamic;
use utility::{Encode, MaxCode, Partitioned, ToCode, TryCoded, TryFromCode};
use strum_macros::{EnumCount, FromRepr};
use strum::{EnumCount};

pub mod dynamic;
pub mod register;
pub mod registers;

pub enum OpCodeSize {
    Byte,
    Word
}

// region: Escape code.
#[derive(EnumCount, FromRepr)]
#[repr(u8)]
pub enum Escape {
    ByteOpcode,
    WordOpcode
}

impl TryFromCode for Escape {
    type Code = u8;

    fn try_from_code(code: Self::Code) -> Option<Self> {
        todo!()
    }
}

impl ToCode for Escape {
    type Code = u8;

    fn to_code(&self) -> Self::Code {
        todo!()
    }
}

impl MaxCode for Escape {
    type Code = u8;

    fn max_code() -> Self::Code {
        todo!()
    }

    fn codes() -> Self::Code {
        todo!()
    }
}

impl TryCoded for Escape {
    type Code = u8;
}
// endregion

pub struct Instruction<OpCode: TryCoded<Code=u16>, PrCode: TryCoded<Code=u8> + MaxCode<Code=u8>> {
    pub prefixes: Vec<Partitioned<u8, PrCode, Escape>>,
    pub operation: OpCode,
    pub static_operand: Option<Register>,
    pub dynamic_operand: Option<Dynamic>
}

#[derive(Debug)]
pub enum EncodeError {
    /// The prefixes support too many codes. There must be enough space in a byte to allow for encoding for the prefix
    /// and the escape codes. Ensure you leave enough paddings for variants of [Escape].
    /// 
    /// The prefixes and escape codes must be representable with a byte and in the decoder are undistinguished.
    TooManyPrefixVariants
}

impl<OpCode: TryCoded<Code=u16>, PrCode: TryCoded<Code=u8> + MaxCode<Code=u8>> Instruction<OpCode, PrCode> {
    pub fn encode(&self) -> Result<Self, EncodeError> {
        let operation = self.operation.to_code();
        
        // Check to see if there is enough space for [Escape] and [PrCode]'s variants.
        let shared: Option<Partitioned<u8, PrCode, Escape>> = Some(Partitioned::First(PrCode::try_from_code(0).unwrap()));
        
        todo!()
    }
}