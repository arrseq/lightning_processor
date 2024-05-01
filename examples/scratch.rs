use std::io::Cursor;

use exr_p::core::decode::{firmware::{Firmware, ImmediatePresence, FirmwareEntry, RegisterPresence}, instruction::MicroInstruction};

fn main() {
    // let bin = [3, 0, 0, 0, 0, 0, 0, 0, 2];
    // let mut stream = Cursor::new(bin);

    // let res = match instruction::decode::decode_macro(&mut stream) {
    //     None => panic!("Failed to parse"),
    //     Some(ins) => ins
    // };

    // match res {
    //     MacroOperation::Safe { divert_location } => println!("safe, {}.", divert_location),
    //     _ => panic!("Fail")
    // }

    // let op = MicroInstruction::And { register_a: 0, register_b: 1 };
    // println!("OP ID: {:?}", op.into_identifier());

    // Core firmware must be initialized before the core can run.
    // If the firmware is uninitialized the CPU will not interface
    // with memory or perform instructions.
    let mut fmw = Firmware::new();
    let mut firmware_source = Cursor::new([
        0x00, // Number of entries

        // First entry
        0x04, // Address                 // ----|
        0x00, // Operation               //     |
                                         //     |
        0b00000000, // Disable all flags //     |
                                         //     |
        // First program                 //     |
        0x00, // No operation            //     |
        0x00,                            // ----|
    ]);

    // fmw.load_binary(&mut firmware_source)
    //     .expect("Failed to load firmware binary");

    let entry_0 = Firmware::read_entry(&mut firmware_source)
        .expect("Failed to parse even one entry");

    println!("Address: {}", entry_0.address);
    println!("Opcode: {}", entry_0.operation);
    println!("Flags: {}", entry_0.flags);

    // fmw.load_entries(Vec::from([
    //     InstructionEntry {
    //         operation: 0,
    //         registers_presence: RegisterPresence::A,
    //         immediate_presence: ImmediatePresence::Byte,
    //         instructions: Vec::from([
    //             MicroInstruction::ByteToRegister { target_register: 0, data: 100 },
    //             MicroInstruction::Add { register_a: 0, register_b: 0 }
    //         ])
    //     },
    //     InstructionEntry {
    //         operation: 1,
    //         registers_presence: RegisterPresence::None,
    //         immediate_presence: ImmediatePresence::None,
    //         instructions: Vec::from([
    //             MicroInstruction::Add { register_a: 0, register_b: 1 }
    //         ])
    //     }
    // ]))
}

