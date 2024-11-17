// src-tauri/src/irc_client/message_handler.rs
use irc::client::prelude::*;
use crate::irc_client::utils::parse_message;
use tauri::Emitter;
use crate::irc_client::utils::format_message;

use futures_util::StreamExt;

pub async fn handle_incoming_messages(client: &mut Client, app_handle: tauri::AppHandle) {
    let mut stream = client.stream().unwrap();

    while let Some(message) = stream.next().await {
        if let Ok(message) = message {
            if let Some((sender, target, text)) = parse_message(&message) {
                let formatted_message = format_message(&sender, &target, &text);

                // Emit the message to the frontend
                app_handle.emit("new_message", formatted_message).unwrap();
            }
        }
    }
}
