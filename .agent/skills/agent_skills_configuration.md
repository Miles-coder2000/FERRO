# FERRO Agent Skills & Domain Knowledge

## 1. Tauri 2.0 Integration
- **IPC Communication:** You know how to bridge Svelte 5 and Rust using `@tauri-apps/api/core` (`invoke`, `listen`, `emit`).
- **System Tray & Window Management:** You can configure the Tauri v2 tray icon, background execution, and handle frameless window attributes (transparent, decorations: false).
- **Global Shortcuts:** You understand how to register and handle OS-level global shortcuts (e.g., `Alt+Space`) via the `tauri-plugin-global-shortcut` plugin to toggle the app visibility.

## 2. High-Performance Rust Systems
- **File System Indexing:** You are skilled in using `walkdir` and `ignore` crates for fast, multi-threaded directory traversal.
- **Live Watching:** You know how to implement `notify` for cross-platform, real-time filesystem event monitoring.
- **Process Management:** You can utilize `std::process::Command` or `tokio::process` to launch external `.exe`, `.app`, or shell scripts completely detached from the launcher process.
- **Embedded Databases:** You are proficient with `rusqlite` and FTS5 (Full-Text Search) for rapid, fuzzy-matching local queries.

## 3. Modern Svelte 5 (Runes) Frontend
- **State Management:** You are an expert in Svelte 5 Runes. You use `$state()` for reactive variables, `$derived()` for computed values, and `$effect()` for side effects.
- **Snippet Architecture:** You understand how to use Svelte 5 snippets (`{#snippet ...}`) for reusable component logic instead of legacy slots.
- **Animation:** You utilize Svelte's built-in transition engine (`svelte/transition` and `svelte/animate`) to create fluid 60fps micro-animations for the UI cards and lists.

## 4. Desktop OS Nuances
- **Windows:** You understand parsing `.lnk` files, extracting `.exe` icons via the `windows` crate, and querying the registry for installed software.
- **macOS:** You understand `/Applications` `.app` bundle structures and extracting `icns` data.
- **Game Platforms:** You know how to parse Steam's `libraryfolders.vdf` and `.acf` files, and Epic Games' `.item` JSON manifests to auto-populate the user's game library.