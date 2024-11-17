// src/js/irc/message.js
import { invoke } from './api/tauri';
import { listen } from './api/event';

export function setupMessageHandling() {
  listen('new_message', (event) => {
    const message = event.payload;
    displayMessage(message);
  });
}

export async function sendMessage(target, message) {
  try {
    await invoke('send_message', { target, message });
  } catch (error) {
    console.error('Failed to send message:', error);
  }
}

function displayMessage(message) {
    const chatWindow = document.getElementById('chat-window');
    const messageElement = document.createElement('div');
    messageElement.textContent = message;
    chatWindow.appendChild(messageElement);
  }
  

