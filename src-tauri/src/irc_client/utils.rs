// src-tauri/src/irc_client/utils.rs

use irc::client::prelude::*;
use chrono::{Local, DateTime};

/// Parses an IRC message and extracts relevant information.
///
/// # Arguments
///
/// * `message` - A reference to an `irc::proto::Message`.
///
/// # Returns
///
/// * `Option<(String, String, String)>` - A tuple containing the sender, target, and message text.
pub fn parse_message(message: &Message) -> Option<(String, String, String)> {
    match &message.command {
        Command::PRIVMSG(target, msg) => {
            if let Some(Prefix::Nickname(nickname, _username, _hostname)) = &message.prefix {
                let sender = nickname.clone();
                return Some((sender, target.clone(), msg.clone()));
            }
        }
        Command::JOIN(channel, ..) => {
            if let Some(Prefix::Nickname(nickname, ..)) = &message.prefix {
                let sender = nickname.clone();
                return Some((sender, channel.clone(), "joined the channel".to_string()));
            }
        }
        Command::PART(channel, ..) => {
            if let Some(Prefix::Nickname(nickname, ..)) = &message.prefix {
                let sender = nickname.clone();
                return Some((sender, channel.clone(), "left the channel".to_string()));
            }
        }
        _ => {}
    }
    None
}


/// Formats a message for display in the UI.
///
/// # Arguments
///
/// * `sender` - The nickname of the message sender.
/// * `target` - The target channel or user.
/// * `message` - The message text.
///
/// # Returns
///
/// * `String` - A formatted string ready for display.
pub fn format_message(sender: &str, _target: &str, message: &str) -> String {
    let timestamp: DateTime<Local> = Local::now();
    format!(
        "[{}] <{}> {}",
        timestamp.format("%H:%M:%S"),
        sender,
        message
    )
}

/// Logs messages to a file.
///
/// # Arguments
///
/// * `log_path` - The path to the log file.
/// * `message` - The message to log.
///
/// # Returns
///
/// * `Result<(), std::io::Error>` - Indicates success or failure.
pub fn log_message(log_path: &str, message: &str) -> Result<(), std::io::Error> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;
    writeln!(file, "{}", message)?;
    Ok(())
}

/// Sanitizes user input to prevent injection attacks.
///
/// # Arguments
///
/// * `input` - The user input string.
///
/// # Returns
///
/// * `String` - A sanitized version of the input string.
pub fn sanitize_input(input: &str) -> String {
    // Remove any control characters and trim whitespace
    input
        .chars()
        .filter(|c| !c.is_control())
        .collect::<String>()
        .trim()
        .to_string()
}



