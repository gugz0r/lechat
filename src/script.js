// script.js

import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';

let useTLS = false;

// TLS Toggle Button
const tlsToggleButton = document.getElementById('tls-toggle');
tlsToggleButton.addEventListener('click', function () {
  useTLS = !useTLS;
  tlsToggleButton.textContent = useTLS ? 'TLS' : 'Non-TLS';
  tlsToggleButton.style.backgroundColor = useTLS ? '#8bc34a' : '#f44336'; // Green for TLS, Red for Non-TLS
});

// Connect Button
document.getElementById('connect').addEventListener('click', async function () {
  const server = document.getElementById('server').value;
  const port = parseInt(document.getElementById('port').value);
  const nickname = document.getElementById('nickname').value;
  const channel = document.getElementById('channel').value;

  if (server && port && nickname && channel) {
    try {
      await invoke('connect_to_irc', {
        server,
        port,
        nickname,
        channel,
        useTLS
      });
      console.log('Connected successfully to IRC server.');

      // Hide the connection screen and show the chat window
      document.querySelector('.connection-screen').classList.add('hidden');
      document.querySelector('.chat-window').classList.remove('hidden');

      // Start listening for messages
      startListeningForMessages();
    } catch (e) {
      console.error('Failed to connect:', e);
    }
  } else {
    alert('Please fill in all fields!');
  }
});

// Send Message Button
document.getElementById('send-message').addEventListener('click', async function () {
  const message = document.getElementById('message').value;

  if (message) {
    try {
      await invoke('send_irc_message', { message });
      // Clear the input field
      document.getElementById('message').value = '';
    } catch (e) {
      console.error('Failed to send message:', e);
    }
  } else {
    alert('Please type a message!');
  }
});

// Function to Listen for Incoming Messages
async function startListeningForMessages() {
  await listen('new_irc_message', event => {
    const message = event.payload.message;
    const messageArea = document.getElementById('message-area');
    const messageElement = document.createElement('div');
    messageElement.textContent = message;
    messageArea.appendChild(messageElement);
  });
}

// Minimize Button (Make sure this element exists in your HTML)
const minimizeButton = document.getElementById('minimize');
if (minimizeButton) {
  minimizeButton.addEventListener('click', async function () {
    try {
      await appWindow.minimize();
    } catch (e) {
      console.error('Failed to minimize the window:', e);
    }
  });
}
