// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use tauri::AppHandle;
use tauri::State;

//https://crates.io/crates/crossbeam-channel

struct GlobalContext {

}

struct Message;

/*
Puede haber un canal que sea para pasar comandos al thread principal.
Para comandos que requieren respuesta, se puede crear otro canal en el momento
de mandar el comando y mandar el tx junto con el comando, de esta forma se puede
bloquear esperando la respuesta.
 */

//use std::time::Duration;
fn main() {
    let (tx,rx) = mpsc::channel();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ports, connect])
        .manage(GlobalContext {})
        //.manage()
        .setup(|app| {
            let handle = app.handle();
            thread::spawn(move || {
                main_thread(handle);
            });
           Ok(()) 
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main_thread(app: AppHandle) {
    
    loop {
        thread::sleep(Duration::from_millis(2000));
        println!("Thread");
    }
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
fn connect(port: String, ctx: State<GlobalContext>) -> bool {
    println!("connect");
    serialport::new(port, 115_200).open().is_ok()
}