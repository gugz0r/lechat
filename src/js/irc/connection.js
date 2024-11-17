// src/js/irc/connection.js
import { invoke } from './api/tauri';

export async function setupConnection() {
  const serverConfig = {
    nickname: 'YowrN1ckn',
    server: 'irc.efnet.org',
    port: 6667,
    // Add other configurations
  };

  try {
    await invoke('connect_to_server', { serverConfig });
    console.log('Connected to IRC server');
  } catch (error) {
    console.error('Failed to connect:', error);
  }
}
