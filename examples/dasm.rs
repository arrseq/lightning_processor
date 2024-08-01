use std::io::Read;
use arrseq_lightning::instruction::Instruction;
use arrseq_lightning::instruction::operation::{Destination, DestinationAndDualInput, DestinationAndInput, DualInput, Input, Operation};
use arrseq_lightning::math::dynamic_number::Size;

fn size_to_str<'a>(size: Size) -> &'a str {
    match size {
        Size::X8 => "byte",
        Size::X16 => "word",
        Size::X32 => "dword",
        Size::X64 => "qword"
    }
}

fn operation_to_str<'a>(operation: Operation) -> &'a str {
    match operation {
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
    }
}

fn decode_instruction(input: &mut impl Read) -> Instruction {
    todo!()
}

fn main() {

}