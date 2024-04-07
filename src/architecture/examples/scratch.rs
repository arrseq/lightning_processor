use std::io::Cursor;

use architecture::instruction::{OperandPresense, Parser};

fn main() {
    let bytes = vec![0, 0, 1];
    let mut rom = Cursor::new(bytes);

    let mut parser = Parser::new(0, OperandPresense {
        source0: true,
        source1: false,
        destination: true,
    });

    let instruction = match parser.parse(&mut rom) {
        Err(err) => return println!("Failed to parse to instruction {:?}", err),
        Ok(ins) => ins
    };

    println!("Parsed to instruction, opcode {}", instruction.operation);
}