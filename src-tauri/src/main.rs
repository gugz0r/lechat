// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::command;
mod commands;
use commands::irc_commands::*;
mod irc_client;
mod storage;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            connect_to_server,
            send_message,
            // Add other commands here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn connect_to_irc(server: String, port: u16, nickname: String, channel: String) {
    // Add the logic to connect to the IRC server here
    println!(
        "Connecting to {}:{} as {} to join {}",
        server, port, nickname, channel
    );
}
