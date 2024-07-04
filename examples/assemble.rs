extern crate atln_processor;

use atln_processor::emulator::memory::Memory;
use atln_processor::emulator::processor::processor::{Core, ExternalContext, instruction, Ports};
use atln_processor::emulator::processor::processor::instruction::Instruction;
use atln_processor::emulator::processor::processor::instruction::operand::{AllPresent, Destination, Dynamic, Operands};
use atln_processor::emulator::processor::processor::instruction::operation::arithmetic::Arithmetic;
use atln_processor::emulator::processor::processor::instruction::operation::Extension;
use atln_processor::number::{Data, Size};
use atln_processor::utility::Encodable;
use std::{env, io};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

fn make_program(total_bytes: usize, code_instances: usize) -> Vec<u8> {
    // add[u8] r0 <- {100}
    let instruction = Instruction::new(Extension::Arithmetic(Arithmetic::Add), Some(instruction::Data {
        width: Size::Byte,
        destination: Destination::Static,
        synchronous: false,
        operands: Operands::AllPresent(AllPresent {
            x_static: 0,
            x_dynamic: Dynamic::Constant(Data::Byte(100))
        })
    })).unwrap().encode();

    let mut bytes = Vec::with_capacity(total_bytes);

    for _ in 0..code_instances {
        bytes.extend(instruction.clone());
    }

    // let bytes_for_instructions = instruction.len() * code_instances;
    // let remaining_padding = total_bytes - bytes_for_instructions;
    //
    // for _ in remaining_padding-1..total_bytes {
    //     bytes.push(0);
    // }

    bytes
}

fn main() -> io::Result<()> {
    println!("Compiling...");
    let compiled = make_program(0, 1000);

    println!("Accessing file at {}", path);

    let mut file = OpenOptions::new().write(true).open(path)?;
    let buffer = &compiled;
    file.write_all(buffer)?;

    Ok(())
}