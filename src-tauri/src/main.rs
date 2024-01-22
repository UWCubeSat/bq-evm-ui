// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ::memchr::memchr;
use serialport::{ErrorKind, SerialPort};
use std::{sync::Mutex, time::Duration};

struct PortState {
    port: Mutex<Option<Box<dyn SerialPort>>>,
}

// We use U32s as status codes to send back to the Host Application.
//
// 0 means Success
// 1 means Serial Open Issue (Already Open or In Use)
// 2 means Mutex Error.

#[tauri::command]
fn print_serial_ports() -> Vec<String> {
    let ports = serialport::available_ports();
    match ports {
        Ok(x) => {
            return x
                .iter()
                .map(|p| -> String {
                    println!("Port: {}", p.port_name);
                    p.port_name.clone()
                })
                .collect::<Vec<String>>();
        }
        Err(e) => return vec![e.to_string()],
    }
}

#[tauri::command]
fn open_serial_port(port_state: tauri::State<PortState>, port_name: &str, baud_rate: u32) -> u32 {
    let new_port_result = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(5000))
        .open();

    let new_port = match new_port_result {
        Ok(p) => p,
        Err(e) => match e.kind {
            ErrorKind::NoDevice => return 1,
            ErrorKind::Io(_) => return 3,
            _ => return 4,
        },
    };

    // Check the Port State.
    let port_guard_result = port_state.port.lock();
    match port_guard_result {
        // Update the current Port
        Ok(mut port) => {
            println!("Port: {} is being attached to state", port_name);
            *port = Some(new_port);
            return 0;
        }
        Err(_) => {
            return 2;
        }
    }
}

#[tauri::command]
fn close_serial_port(port_state: tauri::State<PortState>) -> String {
    // Check the Port State.
    let port_guard_result = port_state.port.lock();
    match port_guard_result {
        // Update the current Port
        Ok(mut port) => {
            println!("Port is being closed!");
            *port = None;
            return String::from("");
        }
        Err(e) => {
            return format!("{}", e);
        }
    }
}

#[tauri::command]
fn read_from_serial_port(port_state: tauri::State<PortState>) -> String {
    // Get Mutex
    let mut port_guard = match port_state.port.lock() {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            return String::from("");
        }
    };

    // Get Port from Option
    let port = match &mut *port_guard {
        Some(p) => p,
        None => {
            return String::from("");
        }
    };

    let mut serial_vec: Vec<u8> = Vec::new();

    loop {
        // Not the greatest solution for performance :3
        let mut serial_buf: [u8; 1] = [0; 1];
        port.read_exact(&mut serial_buf).expect("No Bytes!");

        // Append the Serial Buf into the Vector
        serial_buf.iter().for_each(|b| {
            serial_vec.push(b.clone());
        });

        // Send up to New Line.
        let value = memchr(b'\n', &serial_vec);
        match value {
            Some(_) => {
                return String::from_utf8(serial_vec).expect("Invalid Bytes");
            }
            None => {
                continue;
            }
        }
    }
}

#[tauri::command]
fn write_to_serial_port(port_state: tauri::State<PortState>, data: String) -> bool {
    // Get Mutex
    let mut port_guard = match port_state.port.lock() {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            return false;
        }
    };

    // Get Port from Option
    let port = match &mut *port_guard {
        Some(p) => p,
        None => {
            return false;
        }
    };

    port.write_all(data.as_bytes()).unwrap();
    true
}

fn main() {
    tauri::Builder::default()
        .manage(PortState {
            port: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            print_serial_ports,
            open_serial_port,
            close_serial_port,
            read_from_serial_port,
            write_to_serial_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
