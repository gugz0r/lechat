// src-tauri/src/irc_client/connection.rs
use irc::client::prelude::*;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct IRCClient {
    client: Arc<Mutex<Client>>,
}

impl IRCClient {
    pub async fn new(config: Config) -> Self {
        let client = Client::from_config(config).await.unwrap();
        IRCClient {
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn connect(&self, _server_config: ServerConfig) -> irc::error::Result<()> {
        let client = self.client.lock().await;
        client.identify()?;
        Ok(())
    }

    pub async fn send_message(&self, target: &str, message: &str) -> irc::error::Result<()> {
        let client = self.client.lock().await;
        client.send_privmsg(target, message)?;
        Ok(())
    }

    // Add other methods as needed
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub nickname: String,
    pub server: String,
    pub port: u16,
    // Add other fields
}
