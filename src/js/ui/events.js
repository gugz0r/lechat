// src/js/ui/events.js

import { saveImage } from '../storage.js';

export function setupEvents() {
    const pasteArea = document.getElementById('paste-area');
  
    pasteArea.addEventListener('paste', async (event) => {
      const items = event.clipboardData.items;
      for (let item of items) {
        if (item.type.startsWith('image/')) {
          const blob = item.getAsFile();
          const arrayBuffer = await blob.arrayBuffer();
          const filename = `pasted_image_${Date.now()}.png`;
          await saveImage(new Uint8Array(arrayBuffer), filename);
          // Update UI to display the pasted image
        }
      }
    });
  }
  