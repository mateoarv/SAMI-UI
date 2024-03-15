// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ports, connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_ports() -> Vec<String> {
    println!("get_ports");

    if let Ok(ports) = serialport::available_ports() {
        return ports.iter().map(|x| x.port_name.clone()).collect();
    }
    return Vec::new();
}

#[tauri::command]
fn connect(port: String) -> bool {
    println!("connect");
    serialport::new(port, 115_200).open().is_ok()
}