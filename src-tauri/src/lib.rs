use std::{env, io::Write, os::unix::net::UnixStream};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct VendorInfo {
    vendor_name: String,
    password: String,
    file_mappings: String,
    file_bytes: Vec<u8>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn setup_new_user(payload: VendorInfo) -> Result<String, String> {
    println!("{:#?}", payload);
    let socket_path = env::var("SOCKET_LOCATION").map_err(|e| e.to_string())?;
    let mut stream = UnixStream::connect(&socket_path).map_err(|e| e.to_string())?;
    let msg = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    stream.write_all(msg.as_bytes()).map_err(|e| e.to_string())?;
    Ok(String::from("Gotten into the rust side!!"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, setup_new_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
