#![feature(core_intrinsics)]

use std::collections::HashMap;
use std::fs::{File, write};
use std::io::{Cursor, Write};
use std::path::Path;
use atln_processor::Core;
use atln_processor::instruction::Instruction;
use atln_processor::instruction::operand::{AllPresent, Destination, Dynamic, Operands, Static};
use atln_processor::instruction::operation::arithmetic::Arithmetic;
use atln_processor::instruction::operation::Extension;
use atln_processor::memory::{Frame, Memory, PAGE_BYTES_COUNT};
use atln_processor::number::{Data, Size};

struct Control {
    pub synchronous: bool,
    pub width: Size,
    pub destination: Destination
}

impl Control {
    pub fn data(&self, operands: Operands) -> atln_processor::instruction::Data {
        atln_processor::instruction::Data {
            synchronous: self.synchronous,
            width: self.width.clone(),
            destination: self.destination.clone(),
            operands
        }
    }
}

fn add_instruction(control: Control, x_static: Static, x_dynamic: Dynamic) -> Instruction {
    Instruction {
        extension: Extension::Arithmetic(Arithmetic::Add),
        data: Some(control.data(Operands::AllPresent(AllPresent { x_static, x_dynamic }))),
    }
}

fn subtract_instruction(control: Control, x_static: Static, x_dynamic: Dynamic) -> Instruction {
    Instruction {
        extension: Extension::Arithmetic(Arithmetic::Subtract),
        data: Some(control.data(Operands::AllPresent(AllPresent { x_static, x_dynamic }))),
    }
}

fn main() {
    let bytes = add_instruction(Control { synchronous: false, destination: Destination::Static, width: Size::Quad }, 0, Dynamic::Register(1)).encode();
    println!("{:?}", Instruction::new(&mut Cursor::new(bytes)));
}