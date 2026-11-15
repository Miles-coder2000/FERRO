# FERRO Launcher — Master Architecture & Build Prompt

## Project Overview
Build **FERRO Launcher**, an ultra-lightweight, high-performance Windows desktop launcher and workspace organizer built with **Rust** and **Tauri 2.0**. FERRO unifies PC games, desktop applications, deep system directories, and custom CLI workflows into a single native-speed command center—eliminating the RAM bloat and sluggish boot times of Electron launchers.

- **Target Platform:** Windows 10 / Windows 11 (`x86_64-pc-windows-msvc`)
- **Performance Targets:**
  - Installer size: `< 6 MB` (using native Windows WebView2)
  - Idle RAM footprint: `< 50 MB`
  - Cold boot to interactive UI: `< 200 ms`
  - Search latency across 100,000+ indexed items: `< 15 ms` via SQLite FTS5

---

## Tech Stack
1. **Core Backend & OS Integration:**
   - **Rust (Stable 2021+ Edition)** + **Tauri 2.0 (`tauri = "2"`)**
   - **Async Runtime:** `tokio` (multi-threaded runtime + `spawn_blocking` for filesystem/SQLite I/O)
   - **Database & Full-Text Search:** `rusqlite` (bundled SQLite with `FTS5` and `WAL` mode enabled)
   - **Filesystem Indexing & Watching:** `ignore` / `walkdir` (parallel directory traversal) + `notify` (debounced live filesystem event watcher)
   - **System Telemetry:** `sysinfo` (real-time CPU %, RAM usage, and process monitoring)
   - **Error Handling & Serialization:** `serde`, `serde_json`, `thiserror`
2. **Frontend UI Layer:**
   - **Svelte 5 (Runes mode: `$state`, `$derived`, `$effect`, `$props`)** + **TypeScript (strict)** + **Vite**
   - **Styling:** Tailwind CSS + custom CSS variables for the **Obsidian Industrial** theme
   - **Icons:** `lucide-svelte` + custom SVG iconography + native Windows `.exe`/`.lnk` extracted icon caching via Tauri's `asset:` protocol

---

## Visual Identity: "Obsidian Industrial / Copper Cyber"
Reject flat generic dark modes and neon purple/cyan clichés. Implement a tactile, hardware-meets-software aesthetic:
- **Color Palette:**
  - `--bg-obsidian`: `#090A0C` (deep matte void base)
  - `--surface-tungsten`: `rgba(18, 20, 24, 0.78)` with `backdrop-filter: blur(14px)` (asymmetric glassmorphic panels)
  - `--surface-elevated`: `#181B20`
  - `--border-machined`: `rgba(255, 255, 255, 0.08)`
  - `--accent-copper`: `#E85D04` (primary interactive highlight, active sector badge)
  - `--accent-amber`: `#F48C06` (hover glow, telemetry sparklines, focus rings)
  - `--text-primary`: `#F1F3F5`
  - `--text-secondary`: `#9CA3AF`
  - `--text-telemetry`: `#D97706`
- **Typography:**
  - Display / Sector Headers: `Chakra Petch` (bold uppercase, wide letter-spacing)
  - Body / Item Labels: `Inter`
  - CLI Bar / Telemetry / Paths: `JetBrains Mono`

---

## Application Layout & Core Sectors
1. **Top Telemetry & Command Bar:**
   - **Brand Header:** Circular **FERRO Iron Helix** emblem + wordmark.
   - **Universal Command & Search Palette:** Instant fuzzy/FTS5 search across all games, apps, and folders, plus direct CLI command execution (`> cmd` prefix) with keyboard shortcuts (`Ctrl+K` / `Alt+Space`).
   - **Live Telemetry Strip:** Real-time `CPU % | RAM MB | IPC Latency ms` readout powered by Rust `sysinfo`.
2. **Left Navigation Rail ("SECTORS"):**
   - **HOME (`THE FORGE`):** Unified command dashboard showing Quick Armory (recently played games), Active System Folders with file-count badges, and pinned Utilities & Apps.
   - **FOLDERS:** Deep directory bookmarking and instant Explorer/terminal opening with live file counts.
   - **ARMORY (Games):** Auto-detected and manually added PC games (Steam, Epic, standalone `.exe`) with custom cover art cards, launch arguments, and play-state tracking.
   - **STACK (Apps):** Categorized desktop utilities, IDEs, terminals, and creative suites.
   - **CONTEXTS (Settings):** Sector directory manager, re-index controls, startup behavior, and theme/opacity tuning.
3. **Bottom Engine Status Bar:**
   - Displays `ENGINE: RUST • TAURI 2.0`, active background indexing progress indicator, and quick-scan trigger.

---

## Core Rust Tauri Commands (`#[tauri::command]`)
- `get_dashboard_summary() -> Result<DashboardData, AppError>`
- `scan_sector(sector_id: String, root_path: String, on_progress: Channel<ScanProgress>) -> Result<ScanStats, AppError>`
- `search_items(query: String, sector_filter: Option<String>, limit: usize) -> Result<Vec<LauncherItem>, AppError>`
- `launch_item(item_id: String, extra_args: Vec<String>) -> Result<LaunchReceipt, AppError>`
- `add_custom_item(payload: NewItemPayload) -> Result<LauncherItem, AppError>`
- `get_system_telemetry() -> Result<SystemTelemetry, AppError>`