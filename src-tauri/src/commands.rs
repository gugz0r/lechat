// src-tauri/src/commands.rs
use crate::storage::filesystem;
use tauri::AppHandle;


pub mod irc_commands {
    use crate::irc_client::connection::IRCClient;
    use serde::Deserialize;
    use crate::storage::database::convert_server_config;
   
 
    use tauri::State;
  

    #[derive(Deserialize)]
    pub struct ServerConfig {
        pub nickname: String,
        pub server: String,
        pub port: u16,
        // Add other fields as needed
    }

    #[tauri::command]
    pub async fn connect_to_server(
        server_config: ServerConfig,
        state: State<'_, IRCClient>,
    ) -> Result<(), String> {
        //state.connect(server_config).await.map_err(|e| e.to_string())
        let connection_config = convert_server_config(&server_config);
        state.connect(connection_config).await.map_err(|e| e.to_string())

    }

    #[tauri::command]
    pub async fn send_message(
        target: String,
        message: String,
        state: State<'_, IRCClient>,
    ) -> Result<(), String> {
        state.send_message(&target, &message).await.map_err(|e| e.to_string())
    }

    // Define other commands
}


#[tauri::command]
pub async fn save_pasted_image(app_handle: AppHandle, image_data: Vec<u8>, filename: String) -> Result<(), String> {
    filesystem::save_image(&app_handle, &image_data, &filename)
        .await
        .map_err(|e| e.to_string())
}