pub enum Operation {
    SetRegister,  // 0, d, r0
    CloneRegister // 1, d, r0
}

pub struct OperandPresense {
    register0: bool,
    register1: bool,
    destination: bool
}

pub struct Instruction {
    operation: Operation,
    register0: Option<u8>,
    register1: Option<u8>,
    destination: Option<u8>
}

pub enum Error {
    EndOfStream,
    OperandUnmatched
}

pub struct Parser {
    operation: Operation,
    operand_presense: OperandPresense
}

impl Parser {
    pub fn new(operation: Operation, operand_presense: OperandPresense) -> Self {
        Parser {
            operation,
            operand_presense
        }
    }

    pub fn parse(&mut self) -> Result<Instruction, Error> {
        
        Err(Error::EndOfStream)
    }
}