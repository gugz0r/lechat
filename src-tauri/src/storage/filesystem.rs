// src-tauri/src/storage/filesystem.rs

use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use tauri::AppHandle;

/// Saves an image to the application data directory.
///
/// # Arguments
///
/// * `app_handle` - Reference to Tauri's `AppHandle`.
/// * `data` - The binary data of the image.
/// * `filename` - The name of the file to be saved.
///
/// # Returns
///
/// * `Result<(), std::io::Error>` - Indicates success or failure.
pub async fn save_image(app_handle: &AppHandle, data: &[u8], filename: &str) -> Result<(), std::io::Error> {
    let mut path = app_handle
    .path()
    .app_data_dir()
    .unwrap_or_else(|_| PathBuf::from("."));
    
    path.push("images");
    fs::create_dir_all(&path)?;
    path.push(filename);
    
    fs::write(path, data)?;
    Ok(())
}
