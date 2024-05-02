use std::io::Cursor;

use exr_p::core::decode::{firmware::{Firmware, FirmwareEntry, ImmediatePresence, RawEntry, RegisterPresence}, instruction::MicroInstruction};

fn main() {
    let mut fmw = Firmware::new();
    let mut firmware_source = Cursor::new([
        0x02, // Number of entries

        // First entry
        0x04, // Address                 
        0x91, // Operation               
                                         
        0b00000000, // Disable all flags 

        // Second entry
        0x00, // Address
        0x01, // Operation

        0b00000000, // Disable all flags
                                         
        // First program                 
        0x00, // No operation            
        0x00,   

        // second program                 
        MicroInstruction::Nothing.into_identifier(), // No operation            
        MicroInstruction::Nothing.into_identifier(), // No operation            
        MicroInstruction::Nothing.into_identifier(), // No operation            
        MicroInstruction::Nothing.into_identifier(), // No operation            
        MicroInstruction::Nothing.into_identifier(), // No operation                                       
    ]);

    // fmw.load_binary(&mut firmware_source)
    //     .expect("Failed to load firmware binary");

    let x = RawEntry {
        flags: 0b00000010,
        operation: 0
    };

    let flags = x.decode_flags();
    println!("Registers: {:?}", flags.0)
}

