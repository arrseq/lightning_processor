#[tauri::command]
async fn get_red_noise(width: usize, height: usize) -> tauri::ipc::Response {
    let frame = vec![200u8; width * height * 4];
    tauri::ipc::Response::new(frame)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_red_noise])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
