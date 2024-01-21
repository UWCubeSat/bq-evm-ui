// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ::memchr::memchr;
use serialport::SerialPort;
use std::{sync::Mutex, time::Duration};

struct PortState {
    port: Mutex<Option<Box<dyn SerialPort>>>,
}

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
fn open_serial_port(port_state: tauri::State<PortState>, port_name: &str, baud_rate: u32) -> bool {
    let new_port_result = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open();

    let new_port = match new_port_result {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            return false;
        }
    };

    // Check the Port State.
    let port_guard_result = port_state.port.lock();
    match port_guard_result {
        // Update the current Port
        Ok(mut port) => {
            println!("Port is being assigned to State");
            *port = Some(new_port);
            return true;
        }
        Err(e) => {
            println!("{}", e);
            return false;
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

    let mut serial_buf: [u8; 64] = [0; 64];
    port.read(&mut serial_buf).expect("No Bytes!");

    // Send up to New Line.
    let value = memchr(b'\n', &serial_buf);
    match value {
        Some(index) => {
            return String::from_utf8(serial_buf[0..index].to_vec()).expect("Invalid Bytes");
        }
        None => {
            return String::from_utf8(serial_buf.to_vec()).expect("Invalid Bytes!");
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(PortState {
            port: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            print_serial_ports,
            open_serial_port,
            read_from_serial_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
