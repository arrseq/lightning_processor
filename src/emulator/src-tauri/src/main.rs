// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::{Manager, State};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(x: State<Mutex<usize>>, name: &str) -> usize {
    format!("Hello, {}! You've been greeted from Rust!", name);

    *x.lock().unwrap() += 10;
    x.lock().unwrap().clone()
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(100usize));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
