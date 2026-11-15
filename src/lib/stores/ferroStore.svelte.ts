import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type {
  AppEntry,
  ContextProfile,
  FolderCard,
  FolderUpdatePayload,
  GameEntry,
  SearchMode,
  SearchResultItem,
  SectorTab,
  TelemetrySnapshot,
} from '$lib/types/ferro';
import { isTauriRuntime } from '$lib/utils';

class FerroStore {
  activeSector = $state<SectorTab>('HOME');
  searchQuery = $state<string>('');
  selectedIndex = $state<number>(0);
  isPinnedOnTop = $state<boolean>(false);
  isScanning = $state<boolean>(false);

  apps = $state<AppEntry[]>([]);
  games = $state<GameEntry[]>([]);
  folders = $state<FolderCard[]>([]);
  contexts = $state<ContextProfile[]>([]);

  telemetry = $state<TelemetrySnapshot>({
    cpu_usage_percent: 0,
    ram_usage_mb: 0,
    indexed_items_count: 0,
    last_launch_latency_ms: 0,
  });

  cliOutput = $state<string | null>(null);
  statusToast = $state<string | null>(null);

  private unlistenFolder: UnlistenFn | null = null;
  private telemetryTimer: ReturnType<typeof setInterval> | null = null;
  private toastTimer: ReturnType<typeof setTimeout> | null = null;

  searchMode = $derived.by<SearchMode>(() => {
    const q = this.searchQuery;
    if (q.startsWith('>')) return 'cli';
    if (q.startsWith('/')) return 'folder';
    if (q.startsWith('@')) return 'game';
    return 'all';
  });

  cleanedQuery = $derived.by<string>(() => {
    const mode = this.searchMode;
    const raw = mode === 'all' ? this.searchQuery : this.searchQuery.slice(1);
    return raw.trim().toLowerCase();
  });

  isSearchOpen = $derived<boolean>(this.searchQuery.trim().length > 0);

  searchResults = $derived.by<SearchResultItem[]>(() => {
    if (!this.isSearchOpen || this.searchMode === 'cli') return [];
    const q = this.cleanedQuery;
    const mode = this.searchMode;
    const results: SearchResultItem[] = [];

    if (mode === 'all' || mode === 'game') {
      for (const g of this.games) {
        if (!q || g.title.toLowerCase().includes(q) || g.platform.toLowerCase().includes(q)) {
          results.push({
            id: g.id,
            kind: 'game',
            title: g.title,
            subtitle: `${g.platform} • Armory`,
            path: g.executable_path,
            iconPath: g.cover_image ?? g.icon_path,
            args: g.launch_args,
          });
        }
      }
    }

    if (mode === 'all' || mode === 'folder') {
      for (const f of this.folders) {
        if (!q || f.name.toLowerCase().includes(q) || f.path.toLowerCase().includes(q)) {
          results.push({
            id: f.id,
            kind: 'folder',
            title: f.name,
            subtitle: `${f.file_count} files • ${f.path}`,
            path: f.path,
          });
        }
      }
    }

    if (mode === 'all') {
      for (const a of this.apps) {
        if (!q || a.name.toLowerCase().includes(q) || a.category.toLowerCase().includes(q)) {
          results.push({
            id: a.id,
            kind: 'app',
            title: a.name,
            subtitle: `${a.category} • Stack`,
            path: a.path,
            iconPath: a.icon_path,
          });
        }
      }
    }

    return results.slice(0, 24);
  });

  showToast(msg: string) {
    this.statusToast = msg;
    if (this.toastTimer) clearTimeout(this.toastTimer);
    this.toastTimer = setTimeout(() => {
      this.statusToast = null;
    }, 3200);
  }
  async init() {
    if (!isTauriRuntime()) return;
    this.isScanning = true;
    try {
      const [apps, games, folders, contexts] = await Promise.all([
        invoke<AppEntry[]>('scan_installed_apps'),
        invoke<GameEntry[]>('scan_game_libraries'),
        invoke<FolderCard[]>('get_pinned_folders'),
        invoke<ContextProfile[]>('get_contexts'),
      ]);
      this.apps = apps;
      this.games = games;
      this.folders = folders;
      this.contexts = contexts;

      await this.syncFolderWatcher();
      await this.refreshTelemetry();
    } catch (err) {
      this.showToast(`Init error: ${String(err)}`);
    } finally {
      this.isScanning = false;
    }

    this.unlistenFolder = await listen<FolderUpdatePayload>('folder-updated', (event) => {
      const payload = event.payload;
      this.folders = this.folders.map((f) =>
        f.path === payload.path
          ? { ...f, file_count: payload.file_count, total_size_bytes: payload.total_size_bytes }
          : f
      );
    });

    this.telemetryTimer = setInterval(() => {
      this.refreshTelemetry();
    }, 2000);
  }

  destroy() {
    if (this.unlistenFolder) {
      this.unlistenFolder();
      this.unlistenFolder = null;
    }
    if (this.telemetryTimer) {
      clearInterval(this.telemetryTimer);
      this.telemetryTimer = null;
    }
  }

  async refreshTelemetry() {
    if (!isTauriRuntime()) return;
    try {
      this.telemetry = await invoke<TelemetrySnapshot>('get_system_telemetry');
    } catch {
      // Ignore transient telemetry read errors
    }
  }

  async rescanAll() {
    if (!isTauriRuntime() || this.isScanning) return;
    this.isScanning = true;
    try {
      const [apps, games] = await Promise.all([
        invoke<AppEntry[]>('scan_installed_apps'),
        invoke<GameEntry[]>('scan_game_libraries'),
      ]);
      this.apps = apps;
      this.games = games;
      await this.refreshTelemetry();
      this.showToast(`Indexed ${apps.length} apps & ${games.length} games`);
    } catch (err) {
      this.showToast(`Scan error: ${String(err)}`);
    } finally {
      this.isScanning = false;
    }
  }

  async syncFolderWatcher() {
    if (!isTauriRuntime()) return;
    const paths = this.folders.map((f) => f.path);
    await invoke('watch_pinned_folders', { paths });
  }

  async launchTarget(targetPath: string, args: string[] = [], asAdmin = false) {
    if (!isTauriRuntime()) return;
    try {
      const latencyUs = await invoke<number>('launch_target', {
        targetPath,
        args,
        asAdmin,
      });
      const ms = latencyUs / 1000;
      this.telemetry = { ...this.telemetry, last_launch_latency_ms: ms };
      this.showToast(`Dispatched in ${ms.toFixed(2)} ms`);
    } catch (err) {
      this.showToast(`Launch failed: ${String(err)}`);
    }
  }

  async openFolder(path: string) {
    if (!isTauriRuntime()) return;
    try {
      await invoke('open_folder_in_explorer', { path });
    } catch (err) {
      this.showToast(`Open folder failed: ${String(err)}`);
    }
  }
  async revealLocation(path: string) {
    if (!isTauriRuntime()) return;
    try {
      await invoke('reveal_file_location', { path });
    } catch (err) {
      this.showToast(`Reveal failed: ${String(err)}`);
    }
  }

  async pinFolder(path: string, name?: string) {
    if (!isTauriRuntime()) return;
    try {
      this.folders = await invoke<FolderCard[]>('pin_folder', { path, name: name ?? null });
      await this.syncFolderWatcher();
      this.showToast(`Pinned folder: ${path}`);
    } catch (err) {
      this.showToast(`Pin failed: ${String(err)}`);
    }
  }

  async unpinFolder(id: string) {
    if (!isTauriRuntime()) return;
    try {
      this.folders = await invoke<FolderCard[]>('unpin_folder', { id });
      await this.syncFolderWatcher();
    } catch (err) {
      this.showToast(`Unpin failed: ${String(err)}`);
    }
  }

  async addCustomGame(title: string, executablePath: string, platform: GameEntry['platform']) {
    if (!isTauriRuntime()) return;
    try {
      this.games = await invoke<GameEntry[]>('add_custom_game', {
        title,
        executablePath,
        platform,
      });
      this.showToast(`Added ${title} to Armory`);
    } catch (err) {
      this.showToast(`Failed to add game: ${String(err)}`);
    }
  }

  async saveContext(profile: ContextProfile) {
    if (!isTauriRuntime()) return;
    try {
      this.contexts = await invoke<ContextProfile[]>('save_context', { profile });
      this.showToast(`Saved context "${profile.name}"`);
    } catch (err) {
      this.showToast(`Save context failed: ${String(err)}`);
    }
  }

  async deleteContext(id: string) {
    if (!isTauriRuntime()) return;
    try {
      this.contexts = await invoke<ContextProfile[]>('delete_context', { id });
    } catch (err) {
      this.showToast(`Delete context failed: ${String(err)}`);
    }
  }

  async launchContext(profile: ContextProfile) {
    for (const appId of profile.app_ids) {
      const app = this.apps.find((a) => a.id === appId);
      if (app) {
        await this.launchTarget(app.path);
      }
    }
    for (const folderPath of profile.folder_paths) {
      await this.openFolder(folderPath);
    }
    this.showToast(`Activated context: ${profile.name}`);
  }

  async runCliCommand(cmdText: string) {
    if (!isTauriRuntime()) return;
    try {
      const output = await invoke<string>('execute_shell_command', { commandStr: cmdText });
      this.cliOutput = output;
    } catch (err) {
      this.cliOutput = `ERROR: ${String(err)}`;
    }
  }

  async executeSearchSelection() {
    if (this.searchMode === 'cli') {
      const cmd = this.searchQuery.slice(1).trim();
      if (cmd) await this.runCliCommand(cmd);
      return;
    }
    const item = this.searchResults[this.selectedIndex];
    if (!item) return;
    if (item.kind === 'folder') {
      await this.openFolder(item.path);
    } else {
      await this.launchTarget(item.path, item.args ?? []);
    }
    this.resetSearch();
  }

  moveSelection(delta: number) {
    const len = this.searchResults.length;
    if (len === 0) return;
    this.selectedIndex = (this.selectedIndex + delta + len) % len;
  }

  resetSearch() {
    this.searchQuery = '';
    this.selectedIndex = 0;
    this.cliOutput = null;
  }

  async togglePinOnTop() {
    if (!isTauriRuntime()) return;
    this.isPinnedOnTop = !this.isPinnedOnTop;
    await getCurrentWindow().setAlwaysOnTop(this.isPinnedOnTop);
  }

  async minimizeWindow() {
    if (!isTauriRuntime()) return;
    await getCurrentWindow().minimize();
  }

  async closeWindow() {
    if (!isTauriRuntime()) return;
    await getCurrentWindow().hide();
  }


}

export const ferroStore = new FerroStore();
