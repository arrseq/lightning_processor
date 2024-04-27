use exr_p::instruction::{self, encode, interrupt, MacroOperation};

pub fn main() {
    let int_term = encode::Block {
        instructions: Vec::new(),
        interrupt: Some(interrupt::Interrupt::Terminate)
    };

    let start = encode::Block {
        instructions: Vec::from([
            MacroOperation::Nothing,  // noth
            MacroOperation::Nothing,  // noth
            MacroOperation::Nothing,  // noth
            MacroOperation::Nothing,  // noth

            MacroOperation::Nothing,  // noth
            MacroOperation::Nothing,  // noth
            MacroOperation::Nothing,  // noth
            MacroOperation::Terminate // term
        ]),
        interrupt: None
    };
}