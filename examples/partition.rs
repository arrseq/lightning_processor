extern crate atln_processor;
extern crate strum_macros;
extern crate strum;

use strum_macros::{EnumCount, FromRepr};
use strum::{EnumCount};
use atln_processor::{instruction::register::Register, processor::instruction::{Instruction, Operation}, utility::TryFromCode};
use atln_processor::utility::{Encode, MaxCode, Partitioned, ToCode, TryCoded};

#[derive(Debug, FromRepr, EnumCount, Copy, Clone)]
#[repr(u8)]
enum Pref {
    AcFXN,
    B,
    C
}

#[derive(Debug, FromRepr, EnumCount, Copy, Clone)]
#[repr(u8)]
enum Esc {
    N7X,
    F
}

impl TryFromCode for Pref {
    type Code = u8;

    fn try_from_code(code: Self::Code) -> Option<Self> {
        Self::from_repr(code)
    }
}

impl ToCode for Pref {
    type Code = u8;

    fn to_code(&self) -> Self::Code {
        *self as Self::Code
    }
}

impl MaxCode for Pref {
    type Code = u8;

    fn max_code() -> Self::Code {
        Self::COUNT as Self::Code - 1
    }

    fn codes() -> Self::Code {
        Self::COUNT as Self::Code
    }
}

impl TryCoded for Pref {
    type Code = u8;
}

impl TryFromCode for Esc {
    type Code = u8;

    fn try_from_code(code: Self::Code) -> Option<Self> {
        Self::from_repr(code)
    }
}

impl ToCode for Esc {
    type Code = u8;

    fn to_code(&self) -> Self::Code {
        *self as Self::Code
    }
}

impl MaxCode for Esc {
    type Code = u8;

    fn max_code() -> Self::Code {
        Self::COUNT as Self::Code - 1
    }

    fn codes() -> Self::Code {
        Self::COUNT as Self::Code
    }
}

impl TryCoded for Esc {
    type Code = u8;
}

fn main() {
    let part: Partitioned<u8, Pref, Esc> = Partitioned::try_from_code(4).unwrap();
    
    dbg!(Partitioned::<u8, Pref, Esc>::max_code());
    dbg!(part, part.to_code());
}