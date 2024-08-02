use std::io::{Cursor, Read};
use arrseq_lightning::instruction::Instruction;
use arrseq_lightning::instruction::operand::{AddressingMode, ArrayAddressing, BaseAddressing, ComplexAddressing, ImmediateAddressing, Operand};
use arrseq_lightning::instruction::operation::{Destination, DestinationAndDualInput, DestinationAndInput, DualInput, Input, Operation};
use arrseq_lightning::math::dynamic_number::{Signed, Size};

fn size_to_str<'a>(size: Size) -> &'a str {
    match size {
        Size::X8 => "x8",
        Size::X16 => "x16",
        Size::X32 => "x32",
        Size::X64 => "x64"
    }
}

fn operation_to_str<'a>(operation: Operation) -> &'a str {
    match operation {
        Operation::None => "none",
        Operation::Destination { operation, .. } => match operation { Destination::Unstack => "unstack" },
        Operation::Input { operation, .. } => match operation { Input::Stack => "stack" },
        Operation::DestinationAndInput { operation, .. } => match operation { DestinationAndInput::Copy => "copy" },
        Operation::DualInput { operation, .. } => match operation {
            DualInput::Compare => "compare",
            DualInput::SignedCompare => "signed_compare"
        },
        Operation::DestinationAndDualInput { operation, .. } => match operation {
            DestinationAndDualInput::Add => "add",
            DestinationAndDualInput::FloatingAdd => "floating_add",
            DestinationAndDualInput::Subtract => "subtract",
            DestinationAndDualInput::FloatingSubtract => "floating_subtract",
            DestinationAndDualInput::Multiply => "multiply",
            DestinationAndDualInput::FloatingMultiply => "floating_multiply",
            DestinationAndDualInput::Divide => "divide",
            DestinationAndDualInput::FloatingDivide => "floating_divide"
        },
        Operation::Lock => "lock",
        Operation::VectorOperands => "vec_op",
        Operation::MapVector { .. } => "map_vec",
        Operation::OverrideBranch => "ov_branch"
    }
}

fn register_to_str(register: u8) -> String {
    format!("g{}", register)
}

fn format_offset(offset: i64) -> String {
    if offset.is_negative() {
        format!("[self - {}]", offset.abs())
    } else {
        format!("[self + {}]", offset)
    }
}

fn operand_to_str(operand: Operand) -> String {
    let size_spec = size_to_str(operand.size);

    let body = match operand.mode {
        AddressingMode::Register { register } => &format!("g{}", register),
        AddressingMode::Immediate { mode } => match mode {
            ImmediateAddressing::Immediate { immediate } => &immediate.value.to_string(),
            ImmediateAddressing::Relative { offset } => &format_offset(i64::from(offset))
        },
        AddressingMode::Complex { mode, base } => match mode {
            ComplexAddressing::Base { mode } => match mode {
                BaseAddressing::Base => &format!("[{}]", register_to_str(base)),
                BaseAddressing::Offsetted { offset } => &format!("[{} + {}]", register_to_str(base), offset.value)
            },
            ComplexAddressing::ArrayAddressing { mode, index } => match mode {
                ArrayAddressing::Array => &format!("[{} + {} * {}]", register_to_str(base), register_to_str(index), operand.size.size()),
                ArrayAddressing::Offsetted { offset } => &format!("[{} + {} * {} + {}]", register_to_str(base), register_to_str(index), operand.size.size(), offset.value)
            }
        }
    };

    size_spec.to_string() + " " + body
}

fn operands_to_str(operands: &[Operand]) -> String {
    let mut output = String::new();
    for operand in operands { output += &(operand_to_str(*operand) + " "); }
    output
}

fn disassemble(instruction: Instruction) -> String {
    let operation = operation_to_str(instruction.operation);
    let operands: &[Operand] = match instruction.operation {
        Operation::Destination             { destination, .. }                     => &[ destination ],
        Operation::Input                   { input, .. }                           => &[input],
        Operation::DestinationAndInput     { destination, input, .. }     => &[destination, input],
        Operation::DualInput               { inputs, .. }                       => &[inputs[0], inputs[1]],
        Operation::DestinationAndDualInput { destination, inputs, .. } => &[destination, inputs[0], inputs[1]],
        _ => &[]
    };

    operation.to_string() + " " + &operands_to_str(operands)
}

fn main() {
    let mut rom = Cursor::new(include_bytes!("./dasm.img"));

    loop {
        let instruction = match Instruction::decode(&mut rom) {
            Ok(value) => value,
            Err(_) => continue
        };

        let asm_instruction = disassemble(instruction);
        println!("{}", asm_instruction);
    }

}