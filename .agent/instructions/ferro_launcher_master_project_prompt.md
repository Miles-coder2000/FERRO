# Master Project Prompt: FERRO Launcher

> **Usage Note:** Copy and paste this entire document into an AI coding assistant (such as Cursor, Windsurf, or Claude) or use it as your primary engineering specification to build **FERRO** from scratch.

---

## 1. Project Identity & Vision

* **Application Name:** **FERRO** (derived from the Latin/Italian word for *Iron*, symbolizing the bare-metal speed, memory safety, and durability of Rust).
* **Tagline:** *An All-in-One System Organizer & High-Speed Launcher.*
* **Core Purpose:** Build an ultra-lightweight, keyboard-first, unified desktop launcher and workspace organizer that consolidates local directories, desktop applications, PC games (Steam, Epic, GOG, standalone `.exe`), and CLI shortcuts into a single, blazingly fast interface.
* **Performance Targets:**
  * Cold startup time: `< 150ms`
  * Idle RAM footprint: `< 45MB`
  * Search/Filter latency: `< 5ms` across 10,000+ indexed items.

---

## 2. Recommended Tech Stack

### Core Framework
* **Desktop Runtime:** **Tauri v2.0 (Stable)**
  * *Why:* Uses the OS's native WebView (`WebView2` on Windows, `WebKit` on macOS/Linux) instead of bundling Chromium, resulting in a `< 8MB` binary size and minimal memory overhead while giving full access to Rust's native multi-threaded system APIs.

### Backend (Rust `src-tauri`)
* **Async Runtime:** `tokio` (for non-blocking file indexing, icon extraction, and process spawning).
* **File System & Indexing:** `walkdir` + `ignore` (for fast recursive directory traversal) and `notify` (for real-time filesystem event watching so folder counts update live).
* **Local Persistence & Caching:** `rusqlite` (embedded SQLite with `FTS5` full-text search enabled for instant fuzzy matching) or `redb` (pure Rust embedded key-value store).
* **System Telemetry & Icons:**
  * `sysinfo` (for real-time CPU, RAM, and launcher latency metrics displayed in the top bar).
  * `windows` / `ico` / `image` crates (to extract high-resolution `.exe` and `.lnk` icons and cache them as `.webp` or `.png` thumbnails).
* **Global Shortcuts & Window Management:** `tauri-plugin-global-shortcut` (to toggle the launcher anywhere via `Alt + Space` or `Ctrl + Space`).

### Frontend (`src`)
* **Framework:** **Svelte 5 (Runes)** with **TypeScript** (compiled via **Vite**).
  * *Why:* Svelte compiles away the virtual DOM, ensuring 60/120fps micro-animations and zero framework runtime overhead.
* **Styling:** **Tailwind CSS v4** with custom CSS variables for the industrial theme.
* **Icons:** `lucide-svelte` paired with custom SVG industrial frames.

---

## 3. Visual Theme: "Obsidian Industrial / Copper Cyber"

Reject standard flat dark modes and generic neon cyber themes. **FERRO** uses a tactile, hardware-meets-software aesthetic inspired by forged tungsten, brushed dark steel, and glowing molten copper.

### Color Palette
* **Primary Canvas (Obsidian Metal):** `#0B0D10`
* **Elevated Card Surface (Brushed Tungsten):** `rgba(18, 22, 28, 0.82)` with `backdrop-filter: blur(16px)`
* **Card Border (Machined Steel):** `#262C36` (1px solid) with chamfered/beveled inner highlight `rgba(255, 255, 255, 0.05)`
* **Primary Accent (Molten Copper / Rust):** `#E86A24`
* **Secondary Accent (Forge Amber Glow):** `#FF9E44` (used for active states, hover borders, and glowing telemetry)
* **Telemetry Accent (Cool Cyan):** `#38BDF8` (used sparingly for active system metrics and file waveforms)
* **Text Primary:** `#F1F5F9`
* **Text Muted:** `#8A94A6`

### Typography
* **Display & Headers:** `'Chakra Petch'` or `'Rajdhani'` (uppercase, wide tracking `0.08em`, bold geometric industrial feel).
* **Body & UI:** `'Inter'` or `'Plus Jakarta Sans'`.
* **Telemetry, Paths & CLI:** `'JetBrains Mono'` (for file counts, latency readouts, and search commands).

---

## 4. UI Layout & Component Architecture

Design the main window as a frameless, rounded-corner desktop HUD (`1280x760` default, resizable) layered over the custom Obsidian Industrial background texture.

### A. Top Telemetry & Command Header
1. **Top Status Strip:** Displays the mini **FERRO** emblem on the left, and live Rust-polled system metrics on the right (`CPU: 0.30% | RAM: 15.6 MB | Latency: 0.35 ms`), plus custom window controls (Minimize, Pin-to-Top, Close).
2. **Global Search & CLI Bar:** Centered command palette input (`"Search anything or launch with Rust CLI..."`).
   * Supports instant fuzzy search across apps, games, and folders.
   * Supports command prefixes:
     * `> ` executes a background shell/terminal command.
     * `/ ` jumps directly into folder navigation mode.
     * `@ ` filters strictly by installed games.

### B. Left Navigation Rail ("SECTORS")
A vertical sidebar (`220px` width) with tactile metallic pill buttons that glow with an amber inner border when active:
1. **HOME (The Forge):** Unified dashboard overview (default view).
2. **FOLDERS:** Pinned workspaces, quick-access directories, and storage analytics.
3. **ARMORY (Games):** Full cover-art grid of auto-detected and manually added games.
4. **STACK (Apps):** Categorized desktop software, developer tools, and utilities.
5. **CONTEXTS (Settings):** Custom workspaces (e.g., *Work Mode*, *Gaming Mode*, *3D Dev Mode* that launch multiple apps/folders with one click) and indexer settings.

### C. Main Dashboard Grid ("THE FORGE")
1. **Quick Armory (Games Carousel):**
   * Wide 16:9 game cards with rich cover art, game title, platform badge (Steam, Epic, Local), and launch status (`32ms launch | Tauri Active`).
   * Hovering triggers a subtle copper border glow and a quick-launch play overlay.
2. **Active System Folders (2x3 Grid):**
   * Cards for pinned directories (e.g., `/Projects`, `/Creative Assets`, `/Saves`, `/Systems`).
   * Displays real-time file count (`67 files`) powered by the Rust `notify` watcher and a stylized cyan/copper activity sparkline.
   * Clicking opens the folder in the native file explorer (or an integrated quick-preview drawer).
3. **Utilities & Apps (Icon Matrix):**
   * High-contrast app icons housed inside custom octagonal/beveled metallic frames (e.g., VS Code, Blender, Terminal, Discord, Browser).
   * Supports drag-and-drop reordering and right-click context menus (*Run as Admin*, *Open File Location*, *Pin to Top*).
4. **Bottom Engine Badge:**
   * Subtle status pill in the bottom-right corner: `ENGINE: RUST • TAURI 2.0`.

---

## 5. Core Rust Backend Specifications (`src-tauri`)

Implement the following Tauri `#[tauri::command]` endpoints:

1. `scan_installed_apps() -> Result<Vec<AppEntry>, String>`
   * Scans OS application directories (e.g., `C:\ProgramData\Microsoft\Windows\Start Menu\Programs`, `%AppData%\Microsoft\Windows\Start Menu\Programs`, `/Applications`, or `/usr/share/applications`), resolves shortcuts, extracts icons, and caches metadata in SQLite.
2. `scan_game_libraries() -> Result<Vec<GameEntry>, String>`
   * Parses Steam `libraryfolders.vdf` and `.acf` manifests, Epic Games `.item` manifests, and user-defined game folders to populate the **Armory** automatically.
3. `watch_pinned_folders(paths: Vec<String>) -> Result<(), String>`
   * Spawns a background thread using `notify::RecommendedWatcher` that emits a Tauri event (`folder-updated`) whenever files are added, modified, or deleted in pinned directories.
4. `launch_target(target_path: String, args: Vec<String>, as_admin: bool) -> Result<u64, String>`
   * Measures execution dispatch time in microseconds, spawns the detached child process safely, logs launch frequency (for smart sorting), and returns the dispatch latency.
5. `get_system_telemetry() -> TelemetrySnapshot`
   * Returns current process memory usage, system CPU load, and active indexed item count.

---

## 6. Visual Asset Prompts (Logo & Canvas)

Use these exact prompts if regenerating or refining the visual assets for the application:

* **App Icon / Logo ("The Iron Helix") — 1:1 Aspect Ratio:**
  > *"3D industrial emblem logo on a dark obsidian background, circular badge crafted from weathered dark tungsten and brushed iron with glowing molten copper and amber accents. In the center, an intricate mechanical gear is interwoven with a sharp geometric letter 'F' and a metallic infinity helix loop. Realistic metallic micro-scratches, subtle warm forge sparks, high-contrast studio lighting, centered symmetrical composition, clean icon design."*

* **Launcher Wallpaper Canvas ("Obsidian Industrial") — 16:9 Aspect Ratio:**
  > *"Dark obsidian industrial sci-fi background texture, heavy matte black and gunmetal steel armor plates with beveled edges, subtle brushed metal scratches, thin glowing molten copper circuit lines embedded in seams, dark atmospheric smoke and faint warm embers in the lower left corner, clean uncluttered central area suitable for UI overlay, 8k resolution, unreal engine 5 aesthetic."*

---

## 7. Step-by-Step Build Instructions for AI Assistant

When building this project, execute in the following strict order:
1. **Scaffold Project:** Initialize a Tauri v2 + SvelteKit + TypeScript + Tailwind CSS project structure.
2. **Setup Theme & Assets:** Configure Google Fonts (`Chakra Petch`, `Inter`, `JetBrains Mono`), create the CSS design tokens for the *Obsidian Industrial* palette, and place `logo.png` and `bg-canvas.jpg` in `src/assets/`.
3. **Build Rust Core (`src-tauri/src/`):**
   * Create `models.rs` (`AppEntry`, `GameEntry`, `FolderCard`, `ContextProfile`).
   * Create `indexer.rs` (parallel scanning + icon caching).
   * Create `watcher.rs` (real-time folder monitoring).
   * Create `launcher.rs` (cross-platform process spawning + latency timer).
4. **Build Frontend HUD (`src/lib/components/`):**
   * `TopBar.svelte` (Telemetry + Search/CLI bar).
   * `Sidebar.svelte` (Sectors navigation).
   * `ArmoryGrid.svelte` (Game cards).
   * `FolderMatrix.svelte` (Active system folders with live counts).
   * `AppDock.svelte` (Beveled metallic utility icons).
5. **Wire IPC & Keyboard Navigation:** Connect Svelte stores to Tauri `invoke` commands, add full arrow-key + `Enter` navigation in the search bar, and register the global shortcut to summon/hide the launcher.