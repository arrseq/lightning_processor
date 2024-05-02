use std::io::Cursor;

use exr_p::core::decode::{firmware::{Firmware, RawEntry}, instruction::MicroInstruction};

fn main() {
    let _fmw = Firmware::new();
    let _firmware_source = Cursor::new([
        0x02, // Number of entries

        // First entry
        0x04, // Address                 
        0x91, // Operation               
                                         
        0b00000001, // Disable all flags 

        // Second entry
        0x00, // Address
        0x01, // Operation

        0b00000001, // Disable all flags
                                         
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

    let mut instruction = Cursor::new([
        0x03,       // operation
        0b11110010, // registers ab
        0xFF, 0xFF // Imm
    ]);

    let entry = Firmware::read_entry(&mut instruction, &RawEntry {
        address: 0,
        length: 1,
        operation: 0,
        flags: 0,
    });

    println!("Entry: {:?}", entry);
}

