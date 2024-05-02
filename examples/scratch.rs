use std::io::Cursor;

use exr_p::core::decode::{firmware::{Firmware, FirmwareEntry, RawEntry}, instruction::MicroInstruction};

fn main() {
    let mut fmw = Firmware::new();
    let mut firmware_source = Cursor::new([
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
        0x01,       // operation
        0b11110010, // registers ab
        0x00
    ]);

    let block = Firmware::read_block(&mut instruction, RawEntry {
        operation: 0,
        address: 0,
        length: 2,
        flags: 0b00000000
    });

    println!("{:?}", block);

}

