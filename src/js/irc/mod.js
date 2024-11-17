// src/js/irc/mod.js
import { setupConnection } from './connection.js';
import { setupMessageHandling } from './message.js';

export function initializeIRC() {
  setupConnection();
  setupMessageHandling();
}
