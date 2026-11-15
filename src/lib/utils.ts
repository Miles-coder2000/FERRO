import { convertFileSrc } from '@tauri-apps/api/core';

export function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

export function resolveAssetUrl(filePath?: string): string | undefined {
  if (!filePath) return undefined;
  if (filePath.startsWith('http://') || filePath.startsWith('https://') || filePath.startsWith('data:')) {
    return filePath;
  }
  if (isTauriRuntime()) {
    return convertFileSrc(filePath);
  }
  // Vite dev-mode fallback: resolve assets via import.meta.url
  try {
    return new URL(filePath, import.meta.url).href;
  } catch {
    return undefined;
  }
}

export function formatBytes(bytes: number): string {
  if (bytes <= 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const idx = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  const value = bytes / Math.pow(1024, idx);
  return `${value >= 10 || idx === 0 ? value.toFixed(0) : value.toFixed(1)} ${units[idx]}`;
}
