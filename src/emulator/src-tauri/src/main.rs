// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use protocol::Protocol;
pub mod protocol;

fn main() {
    println!("Starting emulator protocol server.");
    let protocol = Protocol::new();
    
    println!("Starting emulator graphical interface.");
    emulator_lib::run();

    protocol.join();
}
 