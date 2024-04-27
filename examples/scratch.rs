use std::io::Cursor;

use exr_p::instruction::{self, MacroOperation};

fn main() {
    let bin = [3, 0, 0, 0, 0, 0, 0, 0, 2];
    let mut stream = Cursor::new(bin);

    let res = match instruction::decode::decode_macro(&mut stream) {
        None => panic!("Failed to parse"),
        Some(ins) => ins
    };

    match res {
        MacroOperation::Safe { divert_location } => println!("safe, {}.", divert_location),
        _ => panic!("Fail")
    }
}

