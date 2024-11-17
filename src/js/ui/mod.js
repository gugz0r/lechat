// src/js/ui/mod.js
import { setupComponents } from './components.js';
import { setupEvents } from './events.js';

export function initializeUI() {
  setupComponents();
  setupEvents();
}