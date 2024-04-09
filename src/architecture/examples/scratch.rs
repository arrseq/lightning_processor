use std::io::Cursor;

use architecture::instruction::{read_sized_block, OperandPresense, Parser};

fn main() {
    let bytes = vec![0, 20, 15, 0, 100, 47];
    let mut rom = Cursor::new(bytes);

    let data = vec![2, 5, 10];
    let mut data_rom = Cursor::new(data);

    let parsed = read_sized_block(&mut data_rom);
    if parsed.is_some() {
        for el in parsed.unwrap() {
            println!("Byte: {}", el);
        }
    } else {
        println!("Err None received");
    }

    // let mut parser = Parser::new(0, OperandPresense {
    //     source0: true,
    //     source1: false,
    //     destination: true,
    // });

    // let instruction = match parser.parse(&mut rom) {
    //     Err(err) => panic!("Could not parse"),
    //     Ok(ins) => ins
    // };

    // println!("Parsed to instruction, opcode {}, dest {:?}, source 0 {:?}, source 1 {:?}", instruction.operation, instruction.destination, instruction.source0, instruction.source1);

    // let instruction = match parser.parse(&mut rom) {
    //     Err(err) => panic!("Could not parse"),
    //     Ok(ins) => ins
    // };

    // println!("Parsed to instruction, opcode {}, dest {:?}, source 0 {:?}, source 1 {:?}", instruction.operation, instruction.destination, instruction.source0, instruction.source1);
}