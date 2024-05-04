use std::io::Cursor;

use exr_p::{environment::register, instruction::{firmware::{Decoder, Encoder}, traverser::Traverser, MicroInstruction}};

fn main() {
    let mut fmw = Decoder::new();

    let bytes = MicroInstruction::ByteToRegister { target_register: register::FIRST, data: 100 }
        .into_bytes().unwrap();

    println!("Assembler Bytes: {:?}", bytes);

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
        MicroInstruction::And { target: 0, source: 0 }.into_identifier(), // No operation // 11      
        0b00000000,
        MicroInstruction::Nothing.into_identifier(), // No operation // 13        
        MicroInstruction::Nothing.into_identifier(), // No operation // 14        
        MicroInstruction::Nothing.into_identifier(), // No operation // 15   
        bytes[0],
        bytes[1],
        bytes[2],            
    ]);


    let loaded = fmw.decode_binary(&mut firmware_source)
        .expect("Failed to load firmware binary");

    println!("Detected {} entires in firmware", loaded);

    let decoded = fmw.get_entry(0x01).unwrap();

    println!("uOps: {:?}", decoded);

    let encoder = Encoder::new();

    // let reg = Register::from_pointer(0);
    // println!("{:?}", reg);

    let mut first = Traverser::new(decoded.clone());
    let mut target = MicroInstruction::default();
    
    let mut file = register::File::default();

    for _ in 0..5 {
        first.read(&mut target, &mut file);
        println!("{:?}", target);
    }
}

