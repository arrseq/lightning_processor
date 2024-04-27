use exr_p::instruction::{self, encode::{self, encode_class_c_register_operand}, interrupt::{self, Interrupt}, ClassCRegisterOperand, MacroOperation};

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

    let i_test = MacroOperation::Interrupt { code: Interrupt::Terminate as u8 };
    let ops = encode_class_c_register_operand(ClassCRegisterOperand {
        first: 10
    });

    println!("{:?}", ops);

    // FIXME Remember that the assembler will produce memory 
    // addresses on its own based on the length of instructions.
    // This does not cause shift errors because the assembler
    // can fill in memory references easily without shifting due
    // to the operands being allocated to a fixed size.
}