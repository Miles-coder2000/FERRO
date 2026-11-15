<<<<<<< HEAD
export function resolveAssetUrl(path: string): string {
  return new URL(path, import.meta.url).href;
}
=======
import { convertFileSrc } from '@tauri-apps/api/core';

export function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

export function resolveAssetUrl(filePath?: string): string | undefined {
  if (!filePath) return undefined;
  if (filePath.startsWith('http://') || filePath.startsWith('https://') || filePath.startsWith('data:')) {
    return filePath;
  }
  return isTauriRuntime() ? convertFileSrc(filePath) : undefined;
}

export function formatBytes(bytes: number): string {
  if (bytes <= 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const idx = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  const value = bytes / Math.pow(1024, idx);
  return `${value >= 10 || idx === 0 ? value.toFixed(0) : value.toFixed(1)} ${units[idx]}`;
}
>>>>>>> 743870a (feat: initial FERRO launcher project structure)
