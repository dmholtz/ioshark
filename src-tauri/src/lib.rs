use std::time::Duration;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let mut port = serialport::new("/dev/cu.usbmodem1101", 115_200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    port.write("motor 2 left 34\n".as_bytes());


    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn off() {
    let mut port = serialport::new("/dev/cu.usbmodem1101", 115_200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    port.write("motor 2 off\n".as_bytes());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, off])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
