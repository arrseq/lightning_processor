extern crate atln_processor;

use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::thread;

use atln_processor::emulator::memory::Memory;
use atln_processor::emulator::processor::processor::{Core, ExternalContext, Ports};
use atln_processor::emulator::processor::processor::instruction::Instruction;

fn main() {
    let num_cpus = 8;
    let context = Arc::new(Mutex::new(ExternalContext {
        memory: Memory::new(vec![0u8; 1000]),
        ports: Ports::default()
    }));
    
    let mut handles = Vec::with_capacity(num_cpus);
    for cpuid in 0..num_cpus {
        let context_clone = context.clone();

        let handle = thread::spawn(move || {
            let mut cpu0 = Core::default();
            cpu0.context.registers[2] = 1;
            
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

                println!("Executed an instruction. Cpuid={}; Pc={}.", cpuid, cpu0.context.instruction_pointer);
            }
        });

        handles.push(handle);
    }
    
    for handle in handles { handle.join().expect("Core crashed."); }
}