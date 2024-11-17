// src/js/main.js
import { initializeUI } from './ui/mod.js';
import { initializeIRC } from './irc/mod.js';

document.addEventListener('DOMContentLoaded', () => {
  initializeUI();
  initializeIRC();
});
