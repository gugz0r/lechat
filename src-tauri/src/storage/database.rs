// src-tauri/src/storage/database.rs

use sled::{Db, IVec};
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

pub struct Database {
    db: Db,
}

impl Database {
    /// Creates a new instance of the Database.
    pub fn new(app_handle: &AppHandle) -> Self {
        // Construct the path where the database should be stored
        let mut path = app_handle
            .path()
            .app_data_dir()
            .unwrap_or_else(|_| PathBuf::from("."));
        path.push("database");

        // Open the sled database at the given path
        let db = sled::open(path).expect("Failed to open sled database");
        Database { db }
    }

    /// Inserts a key-value pair into the database.
    ///
    /// # Arguments
    ///
    /// * `key` - The key under which the value will be stored.
    /// * `value` - The value to store.
    ///
    /// # Returns
    ///
    /// * `sled::Result<Option<IVec>>` - Result indicating success or failure.
    pub fn insert(&self, key: &str, value: &[u8]) -> sled::Result<Option<IVec>> {
        self.db.insert(key, value)
    }

    /// Fetches a value from the database by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the value to retrieve.
    ///
    /// # Returns
    ///
    /// * `Option<sled::IVec>` - The value, if it exists.
    pub fn get(&self, key: &str) -> Option<IVec> {
        self.db.get(key).ok().flatten()
    }

    /// Removes a value from the database by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the value to remove.
    ///
    /// # Returns
    ///
    /// * `sled::Result<Option<IVec>>` - Result indicating success or failure.
    pub fn remove(&self, key: &str) -> sled::Result<Option<IVec>> {
        self.db.remove(key)
    }
}

/// Converts `irc_commands::ServerConfig` to `connection::ServerConfig`
///
/// # Arguments
///
/// * `config` - The `irc_commands::ServerConfig` to convert.
///
/// # Returns
///
/// * `connection::ServerConfig` - The converted server config.
pub fn convert_server_config(
    config: &crate::commands::irc_commands::ServerConfig,
) -> crate::irc_client::connection::ServerConfig {
    crate::irc_client::connection::ServerConfig {
        nickname: config.nickname.clone(),
        server: config.server.clone(),
        port: config.port,
    }
}

/// Resolves the path for storing application data.
fn resolve_path(app_handle: &AppHandle, folder_name: &str) -> PathBuf {
    let mut path = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    path.push(folder_name);
    path
}

