export interface AppEntry {
  id: string;
  name: string;
  path: string;
  icon_path?: string;
  category: string;
  launch_count: number;
  last_launched?: number;
}

export interface GameEntry {
  id: string;
  title: string;
  platform: 'Steam' | 'Epic' | 'GOG' | 'Local';
  executable_path: string;
  launch_args: string[];
  cover_image?: string;
  icon_path?: string;
  launch_count: number;
}

export interface FolderCard {
  id: string;
  name: string;
  path: string;
  file_count: number;
  total_size_bytes: number;
  is_pinned: boolean;
  icon_name?: string;
}

export interface ContextProfile {
  id: string;
  name: string;
  description: string;
  app_ids: string[];
  folder_paths: string[];
  shortcut_key?: string;
}

export interface TelemetrySnapshot {
  cpu_usage_percent: number;
  ram_usage_mb: number;
  indexed_items_count: number;
  last_launch_latency_ms: number;
}

export interface FolderUpdatePayload {
  path: string;
  file_count: number;
  total_size_bytes: number;
}

export type SectorTab = 'HOME' | 'FOLDERS' | 'ARMORY' | 'STACK' | 'CONTEXTS';

export type SearchMode = 'all' | 'cli' | 'folder' | 'game';

export interface SearchResultItem {
  id: string;
  kind: 'app' | 'game' | 'folder';
  title: string;
  subtitle: string;
  path: string;
  iconPath?: string;
  args?: string[];
}
