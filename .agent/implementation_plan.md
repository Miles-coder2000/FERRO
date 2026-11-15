# Implementation Plan

## Overview

Build FERRO, an ultra-lightweight, high-speed, keyboard-first desktop launcher and workspace organizer using Tauri v2, Rust, Svelte 5 (Runes), TypeScript, and Tailwind CSS v4, styled in an Obsidian Industrial / Copper Cyber visual theme.

FERRO unifies desktop applications, PC game libraries (Steam, Epic, local executables), system folder watching with real-time file counts, and direct CLI shortcuts into a single frameless HUD window (`1280x760`). Rust backend capabilities include multi-threaded recursive directory indexing, SQLite + FTS5 metadata persistence, native Windows icon extraction (`.exe`/`.lnk`), live `notify` directory watching, process launch latency measurement (`launch_target`), system hardware telemetry via `sysinfo`, and global hotkey summoning.

## Types

### Rust Data Structures (`src-tauri/src/models.rs`)

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub icon_path: Option<String>,
    pub category: String,
    pub launch_count: u32,
    pub last_launched: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameEntry {
    pub id: String,
    pub title: String,
    pub platform: String, // "Steam", "Epic", "GOG", "Local"
    pub executable_path: String,
    pub launch_args: Vec<String>,
    pub cover_image: Option<String>,
    pub icon_path: Option<String>,
    pub launch_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderCard {
    pub id: String,
    pub name: String,
    pub path: String,
    pub file_count: usize,
    pub total_size_bytes: u64,
    pub is_pinned: bool,
    pub icon_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContextProfile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub app_ids: Vec<String>,
    pub folder_paths: Vec<String>,
    pub shortcut_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelemetrySnapshot {
    pub cpu_usage_percent: f32,
    pub ram_usage_mb: f32,
    pub indexed_items_count: usize,
    pub last_launch_latency_ms: f64,
}
```

### TypeScript Data Structures (`src/lib/types/ferro.ts`)

```typescript
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

export type SectorTab = 'HOME' | 'FOLDERS' | 'ARMORY' | 'STACK' | 'CONTEXTS';
```

## Files

### New Files to Create

#### Root & Frontend Configuration

- `package.json`: NPM package configuration specifying dependencies (Svelte 5, Vite, `@tauri-apps/api`, `tailwindcss`, `@tailwindcss/vite`, `lucide-svelte`, `clsx`, `tailwind-merge`).
- `vite.config.ts`: Vite setup with `@sveltejs/vite-plugin-svelte` and `@tailwindcss/vite`.
- `tsconfig.json`: TypeScript configuration for Svelte 5 project.
- `svelte.config.js`: Svelte 5 preprocessor configuration.
- `index.html`: Main HTML template with Google Fonts preloading (`Chakra Petch`, `Inter`, `JetBrains Mono`).

#### Rust Backend (`src-tauri/`)

- `src-tauri/Cargo.toml`: Rust crate dependencies (`tauri`, `serde`, `serde_json`, `tokio`, `rusqlite`, `walkdir`, `notify`, `sysinfo`, `image`, `ico`, `windows-sys`, `tauri-plugin-global-shortcut`, `tauri-plugin-opener`, `open`, `dirs`).
- `src-tauri/tauri.conf.json`: Tauri v2 configuration file specifying frameless window, transparent/dark background, `1280x760` window size, permissions, and app identifier (`com.ferro.launcher`).
- `src-tauri/build.rs`: Standard Tauri build script.
- `src-tauri/src/main.rs`: Entry point invoking `ferro_lib::run()`.
- `src-tauri/src/lib.rs`: Tauri builder, registering IPC command handlers, state management, and plugins.
- `src-tauri/src/models.rs`: Struct definitions (`AppEntry`, `GameEntry`, `FolderCard`, `ContextProfile`, `TelemetrySnapshot`).
- `src-tauri/src/db.rs`: SQLite persistence layer with FTS5 table initialization and CRUD methods.
- `src-tauri/src/indexer.rs`: Application scanner (`scan_installed_apps`), icon extractor (using Windows shell APIs / `ico` crate to extract icons into `%APPDATA%/ferro/icons`), and Steam/Epic game library parsers (`scan_game_libraries`).
- `src-tauri/src/watcher.rs`: Live filesystem monitoring using `notify::RecommendedWatcher` emitting `folder-updated` Tauri events.
- `src-tauri/src/launcher.rs`: Process execution engine with microsecond latency recording (`launch_target`) and shell command execution (`>` CLI prefix handling).
- `src-tauri/src/telemetry.rs`: Hardware monitoring service (`get_system_telemetry`) polling CPU % and RAM footprint via `sysinfo`.

#### Frontend (`src/`)

- `src/app.css`: Tailwind v4 theme configuration with `@theme` block establishing Obsidian Industrial color tokens (`obsidian`, `tungsten`, `steel`, `molten-copper`, `forge-amber`, `cool-cyan`).
- `src/main.ts`: Svelte application entry point mounting `App.svelte`.
- `src/App.svelte`: HUD Layout root container with top bar, left rail, main viewport switcher, global keyboard event listeners (arrow keys, Enter, Esc), and IPC state initialization.
- `src/lib/types/ferro.ts`: Shared TypeScript interfaces.
- `src/lib/stores/ferroStore.svelte.ts`: Svelte 5 Rune-based reactive state store for active tab, query, search results, telemetry, pinned folders, app entries, and games.
- `src/lib/components/TopBar.svelte`: Frameless top bar with custom window controls (minimize, pin, close), telemetry readout ticker, and central command palette input (`>`, `/`, `@` prefix handling).
- `src/lib/components/Sidebar.svelte`: Left rail navigation ("SECTORS": HOME, FOLDERS, ARMORY, STACK, CONTEXTS) with illuminated copper/amber indicators.
- `src/lib/components/ArmoryGrid.svelte`: PC Game cards display grid with 16:9 cover art, platform indicators, hover glow, and instant launch trigger.
- `src/lib/components/FolderMatrix.svelte`: Active folder grid displaying live file counts, size summaries, and native folder opening actions.
- `src/lib/components/AppDock.svelte`: Application matrix of beveled metallic icons with right-click context menu options (*Run as Admin*, *Open File Location*, *Pin*).
- `src/lib/components/ContextsPanel.svelte`: Workspace modes panel allowing one-click launching of batched application groups and pinned folder sets.
- `src/lib/components/SearchResults.svelte`: Instant search result overlay displaying grouped matches when user types in search bar.
- `src/assets/logo.png`: Copied/converted from `FERRO_LOGO.jfif`.
- `src/assets/bg-canvas.jpg`: Copied/converted from `FERRO_BACKGROUND.jfif`.

## Functions

### Rust IPC Commands (`src-tauri/src/`)

- `scan_installed_apps() -> Result<Vec<AppEntry>, String>` (`indexer.rs`): Discovers `.lnk` and `.exe` files in Start Menu and Program Files directories, extracts icons to disk, caches results in SQLite, and returns items.
- `scan_game_libraries() -> Result<Vec<GameEntry>, String>` (`indexer.rs`): Parses Steam `vdf`/`acf` files and Epic Games manifests to detect installed games with cover art / executable paths.
- `watch_pinned_folders(paths: Vec<String>, state: State<AppState>) -> Result<(), String>` (`watcher.rs`): Initializes `notify::RecommendedWatcher` on supplied directory paths and emits `folder-updated` payload events to frontend on changes.
- `launch_target(target_path: String, args: Vec<String>, as_admin: bool) -> Result<u64, String>` (`launcher.rs`): Spawns process using `std::process::Command`, measures execution dispatch latency in microseconds, updates DB launch frequency metrics, and returns latency.
- `get_system_telemetry(state: State<AppState>) -> TelemetrySnapshot` (`telemetry.rs`): Queries `sysinfo::System` for CPU percentage and process resident memory (RAM) in MB.
- `execute_shell_command(command_str: String) -> Result<String, String>` (`launcher.rs`): Executes shell/terminal commands (triggered by `>` search prefix).
- `get_folder_details(path: String) -> Result<FolderCard, String>` (`indexer.rs`): Reads directory contents using `walkdir` to count files and calculate total byte size.

### Svelte Frontend State & Helper Methods (`src/lib/`)

- `loadInitialData()`: Invokes backend scanner APIs (`scan_installed_apps`, `scan_game_libraries`) and initializes telemetry interval polling.
- `handleSearchInput(query: string)`: Parses search string for special mode prefixes (`>`, `/`, `@`) and filters active item list in real-time.
- `handleKeyDown(e: KeyboardEvent)`: Manages keyboard navigation (ArrowUp/ArrowDown, Enter to launch top match, Esc to clear/hide).
- `launchItem(item: AppEntry | GameEntry)`: Invokes `launch_target` command and updates UI latency ticker.
- `openFolder(path: string)`: Triggers native file explorer dispatch via `open` or Tauri opener plugin.

## Classes

*N/A - Structural design relies on idiomatic Rust modules/structs and Svelte 5 Runes/Components.*

### Svelte Store Class / Module (`src/lib/stores/ferroStore.svelte.ts`)

- `FerroState`: Encapsulates Svelte 5 `$state()` variables (`activeSector`, `searchQuery`, `telemetry`, `apps`, `games`, `folders`, `contexts`, `selectedIndex`) with methods for state updates and command invocations.

## Dependencies

### Frontend Dependencies (`package.json`)

- `svelte`: `^5.0.0`
- `@tauri-apps/api`: `^2.0.0`
- `@tauri-apps/plugin-global-shortcut`: `^2.0.0`
- `@tauri-apps/plugin-opener`: `^2.0.0`
- `lucide-svelte`: `^0.460.0`
- `tailwindcss`: `^4.0.0`
- `@tailwindcss/vite`: `^4.0.0`
- `vite`: `^6.0.0`
- `typescript`: `^5.6.0`

### Backend Dependencies (`src-tauri/Cargo.toml`)

- `tauri`: `2.2.0`
- `serde`: `1.0` (with `derive`)
- `serde_json`: `1.0`
- `tokio`: `{ version = "1.0", features = ["full"] }`
- `rusqlite`: `{ version = "0.31", features = ["bundled"] }`
- `walkdir`: `2.5`
- `notify`: `6.1`
- `sysinfo`: `0.30`
- `image`: `0.25`
- `ico`: `0.3`
- `dirs`: `5.0`
- `open`: `5.3`
- `windows-sys`: `{ version = "0.59", features = ["Win32_UI_Shell", "Win32_Graphics_Gdi"] }`
- `tauri-plugin-global-shortcut`: `2.0.0`
- `tauri-plugin-opener`: `2.0.0`

## Testing

- **Backend Rust Tests (`src-tauri/src/`):**
  - Test SQLite table creation and FTS indexing in `db.rs`.
  - Test application link resolver and game manifest parser logic in `indexer.rs`.
  - Test shell process launcher and execution dispatch in `launcher.rs`.
  - Run via `cargo test --manifest-path src-tauri/Cargo.toml`.
- **Frontend Build & Lint Validation:**
  - Test TypeScript types and Svelte component compilation via `npm run check` / `npx ssc` or `vite build`.
  - Validate frameless UI render, keyboard navigation responsiveness, and styling accuracy.

## Implementation Order

1. **Project Infrastructure Setup:**
   - Create root configuration files (`package.json`, `vite.config.ts`, `tsconfig.json`, `svelte.config.js`, `index.html`).
   - Create Tauri v2 configuration (`src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/build.rs`).
   - Convert/move asset images (`FERRO_LOGO.jfif`, `FERRO_BACKGROUND.jfif`) into `src/assets/`.

2. **Backend Engine Implementation (`src-tauri/src/`):**
   - Write `models.rs` for data structures.
   - Write `db.rs` for SQLite database initialization and caching.
   - Write `indexer.rs` for Start Menu application indexing and Steam/Epic game detection.
   - Write `watcher.rs` for real-time filesystem directory monitoring using `notify`.
   - Write `launcher.rs` for target process execution and shell command execution with latency profiling.
   - Write `telemetry.rs` for sysinfo CPU and RAM tracking.
   - Wire commands into `lib.rs` and `main.rs`.

3. **Frontend Architecture & Styling Setup:**
   - Configure Tailwind v4 with Obsidian Industrial design tokens in `src/app.css`.
   - Create TypeScript definitions in `src/lib/types/ferro.ts`.
   - Create Svelte 5 reactive store in `src/lib/stores/ferroStore.svelte.ts`.

4. **Frontend HUD Components Construction:**
   - Implement `TopBar.svelte` with frameless window management, telemetry strip, and multi-mode search/CLI input.
   - Implement `Sidebar.svelte` with sector navigation.
   - Implement `ArmoryGrid.svelte` for cover-art game showcase.
   - Implement `FolderMatrix.svelte` for workspace directory cards.
   - Implement `AppDock.svelte` for application icons matrix.
   - Implement `ContextsPanel.svelte` for preset launcher modes.
   - Implement `SearchResults.svelte` overlay component.
   - Assemble top-level HUD in `App.svelte` with full keyboard bindings (`ArrowUp`, `ArrowDown`, `Enter`, `Escape`).

5. **Integration & Verification:**
   - Verify Rust compilation (`cargo check`, `cargo test`).
   - Run Vite build to verify frontend compilation.
   - Validate UI layout, theme styling, IPC command wiring, and keyboard shortcuts.
