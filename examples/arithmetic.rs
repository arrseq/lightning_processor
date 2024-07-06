extern crate atln_processor;

#[feature(ptr_metadata)]

use std::fs::{self, OpenOptions};
use std::io::{Cursor, Read, Write};
use std::sync::{Arc, Mutex};
use std::{env, thread, fs::File};

use atln_processor::emulator::memory::Memory;
use atln_processor::emulator::processor::processor::{Core, ExternalContext, instruction, Ports};
use atln_processor::emulator::processor::processor::instruction::Instruction;
use atln_processor::emulator::processor::processor::instruction::operand::{AllPresent, Destination, Dynamic, Operands};
use atln_processor::emulator::processor::processor::instruction::operation::arithmetic::Arithmetic;
use atln_processor::emulator::processor::processor::instruction::operation::Extension;
use atln_processor::number::{Number, Size};
use atln_processor::utility::Encodable;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();
    
    println!("Accessing file at {}", path);

    let mut file = File::open(path.clone())?;
    let metadata = fs::metadata(path).expect("unable to read metadata");
    let mut mem_buf = vec![0u8; metadata.len() as usize];

    file.read(&mut mem_buf);

    let num_cpus = 20;
    let context = Arc::new(Mutex::new(ExternalContext {
        memory: Memory::new(mem_buf),
        ports: Ports::default()
    }));
    
    let mut handles = Vec::with_capacity(num_cpus);
    for cpuid in 0..num_cpus {
        let context_clone = context.clone();

        let handle = thread::spawn(move || {
            let mut cpu0 = Core::default();
            cpu0.context.registers[0] = 0;
            
            loop {
                let (instruction, instruction_length) = {
                    let binding = &context_clone.lock().unwrap().memory.bytes;
                    let mut cursor = Cursor::new(binding);
                    cursor.set_position(cpu0.context.instruction_pointer);
                    // Get the length by calculating the difference between the new cursor position and the last instruction 
                    // pointer.
                    (Instruction::decode(&mut cursor).expect("Invalid metal in memory"), cursor.position() - cpu0.context.instruction_pointer)
                };
            
                cpu0.execute(&instruction, &mut context_clone.lock().unwrap());
                cpu0.context.instruction_pointer += instruction_length;

                println!("Executed an instruction. Cpuid={}; Pc={}; r0={}; Regrouping={}", cpuid, cpu0.context.instruction_pointer, cpu0.context.registers[0], cpu0.context.arithmetic_flags.regrouping);
            }
        });

        handles.push(handle);
    }
    
    for handle in handles { handle.join().expect("Core crashed."); }
    Ok(())
}