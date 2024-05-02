use std::io::Cursor;

use exr_p::core::decode::{firmware::{Firmware}, instruction::MicroInstruction};

fn main() {
    let mut fmw = Firmware::new();
    let mut firmware_source = Cursor::new([  // Address
        0x02, // Number of entries                                   // 0

        // First entry
        0x09, // Address                                             // 1
        0x02, // length                                              // 2
        0x91, // Operation                                           // 3
                                         
        0b00000001, // Disable all flags                             // 4

        // Second entry
        0x0B, // Address                                             // 5
        0x05, // length                                              // 6
        0x01, // Operation                                           // 7

        0b00000001, // Disable all flags                             // 8
                                         
        // First program                 
        0x00, // No operation                                        // 9
        0x00,                                                        // 10
                
        // second program                                            
        MicroInstruction::Nothing.into_identifier(), // No operation // 11      
        MicroInstruction::Nothing.into_identifier(), // No operation // 12         
        MicroInstruction::Nothing.into_identifier(), // No operation // 13        
        MicroInstruction::Nothing.into_identifier(), // No operation // 14        
        MicroInstruction::Nothing.into_identifier(), // No operation // 15                                   
    ]);

    let loaded = fmw.load_binary(&mut firmware_source)
        .expect("Failed to load firmware binary");

    println!("Detected {} entires in firmware", loaded);

    // println!("Entry: {:?}", entries);
}

