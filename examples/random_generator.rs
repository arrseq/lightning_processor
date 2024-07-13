use std::io::Cursor;
use arrseq_instruction::Instruction;

fn instruction_str(instruction: Instruction) -> String {
    format!("o={:?} {:?}, {:?}", instruction.operation, instruction.operands.register, instruction.operands.dynamic).to_string()
}

fn main() {
    let mut time = 0usize;
    
    loop {
        time += 1;
        
        let mut cursor = Cursor::new(time.to_le_bytes());
        
        if let Ok(i) = Instruction::decode(&mut cursor) {
            println!("{}", instruction_str(i));
        }
    }
}