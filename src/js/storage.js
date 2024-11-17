import { invoke } from './api/tauri';

export async function saveImage(imageData, filename) {
  try {
    await invoke('save_pasted_image', { imageData: Array.from(imageData), filename });
  } catch (error) {
    console.error('Failed to save image:', error);
  }
}
