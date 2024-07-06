use instruction;
use utility::{ToCode, TryCoded, TryFromCode};

// region: Operation code.
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl TryFromCode for Operation {
    type Code = u16;

    fn try_from_code(code: Self::Code) -> Option<Self> {
        Some(match code {
            1 => Self::Add,
            _ => return None
        })
    }
}

impl ToCode for Operation {
    type Code = ();

    fn to_code(&self) -> Self::Code {
        todo!()
    }
}

impl TryCoded for Operation {}
// region

pub type Instruction = instruction::Instruction<Operation, Operation>;