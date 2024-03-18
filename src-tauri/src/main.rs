// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::Duration;
use tauri::AppHandle;
use tauri::State;

//https://crates.io/crates/crossbeam-channel

struct GlobalContext {
    tx: Sender<Message>
}

struct Message{
    tx_resp: Sender<Response>,
    data: u8,
}

struct Response {
    data: u8,
}

struct Packet<M, R> {
    tx_resp: Sender<R>,
    data: M,
}

struct EventSender<M,R> {
    tx: Sender<Packet<M,R>>
}

impl<M,R> EventSender<M,R> {
    fn send_event(&self, data: M) -> Result<R, ()>{
        let (tx_resp,rx) = mpsc::channel::<R>();
        self.tx.send(Packet {
            tx_resp,
            data
        }).unwrap();
        rx.recv_timeout(Duration::from_millis(100)).or(Err(()))
    }
}

struct EventReceiver<M,R> {
    rx: Receiver<Packet<M,R>>
}

impl<M,R> EventReceiver<M,R> {
    fn receive_event<F: Fn(M) -> R>(&self, f: F){
        if let Ok(p) = self.rx.try_recv() {
            let resp = f(p.data);
            p.tx_resp.send(resp).unwrap();
        }
    }
}



/*
Puede haber un canal que sea para pasar comandos al thread principal.
Para comandos que requieren respuesta, se puede crear otro canal en el momento
de mandar el comando y mandar el tx junto con el comando, de esta forma se puede
bloquear esperando la respuesta.
 */

//use std::time::Duration;
fn main() {
    let (tx,rx) = mpsc::channel::<Message>();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ports, connect])
        .manage(GlobalContext {tx})
        .setup(|app| {
            let handle = app.handle();
            thread::spawn(move || {
                main_thread(handle, rx);
            });
           Ok(()) 
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main_thread(app: AppHandle, rx: Receiver<Message>) {
    loop {
        if let Ok(msg) = rx.try_recv() {
            println!("Received: {}",msg.data);
            msg.tx_resp.send(Response {data: msg.data + 1}).unwrap();
        }
        thread::sleep(Duration::from_millis(50));
        //println!("Thread");
    }
}

fn send_message(tx: &Sender<Message>, data: u8) -> Result<u8,()> {
    let (tx_resp,rx_resp) = mpsc::channel::<Response>();
    let msg = Message {tx_resp, data};
    tx.send(msg).unwrap();
    let data = (rx_resp.recv_timeout(Duration::from_millis(100)).or(Err(()))?).data;
    Ok(data)
}

#[tauri::command]
fn get_ports(ctx: State<GlobalContext>) -> Vec<String> {
    println!("get_ports");
    if let Ok(resp_data) = send_message(&ctx.tx, 69) {
        println!("Response: {}", resp_data);
    } else {
        println!("Response timeout");
    }

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