// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use atln_processor::memory::{Memory, Frame};
use atln_processor::number::{Data, Size};
use tauri::Manager;
use tauri::State;

#[tauri::command]
fn read_memory_byte(memory: State<Mutex<Memory>>, address: u64, translate: bool) -> Option<u8> {
    match memory.lock().unwrap().get(Frame {
        address,
        size: Size::Byte
    }, translate) {
        Ok(data) => if let Data::Byte(byte) = data {
            return Some(byte);       
        } else { unreachable!() },
        Err(_) => None
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Emulator system memory.
            app.manage(Mutex::new(Memory::from(vec![0u8; 100])));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_memory_byte])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
