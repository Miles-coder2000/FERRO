# FERRO Agent Operational Guidelines & Methodologies

> **Purpose:** This document defines the standard operating procedures (SOPs), architectural patterns, and quality gates the AI agent must follow when implementing features, writing Rust/Svelte code, or debugging **FERRO**.

---

## 1. End-to-End Feature Implementation Workflow

Whenever asked to build a new feature (e.g., adding a new sector, indexer, or launcher action), execute strictly in this 5-phase sequence:

### Phase 1: Contract & Data Modeling (Rust ↔ TypeScript)
1. Define the Rust data structures first in `src-tauri/src/models.rs` with `#[derive(Debug, Clone, Serialize, Deserialize)]` and `#[serde(rename_all = "camelCase")]`.
2. Immediately mirror those exact types in `src/lib/types/models.ts` so the IPC boundary is 100% type-safe.

### Phase 2: Asynchronous Rust Backend (`src-tauri`)
1. Implement core business logic in dedicated domain modules (`indexer.rs`, `watcher.rs`, `launcher.rs`, `db.rs`), keeping `lib.rs` / `main.rs` clean and limited to wiring state and commands.
2. Expose thin `#[tauri::command]` wrappers that validate inputs, delegate work to `tokio::spawn` or `tokio::task::spawn_blocking` (for CPU-heavy icon extraction/SQLite queries), and map all errors to `Result<T, String>`.
3. Register the command in `tauri::generate_handler![...]`.

### Phase 3: Tauri v2 Capabilities & Permissions
1. Before testing any new OS API (filesystem paths, shell execution, global shortcuts, asset protocol), explicitly update `src-tauri/capabilities/default.json` with the required Tauri v2 permission scopes.
2. Never assume Tauri v1 permissive defaults; scope file access strictly to required directories (`$APPCACHE`, user-pinned folders, and program directories).

### Phase 4: Reactive Svelte 5 Frontend (`src`)
1. Encapsulate Tauri `invoke` and `listen` calls inside `src/lib/services/ipc.ts`—never scatter raw `invoke()` calls across UI components.
2. Manage shared state using Svelte 5 `.svelte.ts` rune modules (e.g., `launcherState.svelte.ts`) utilizing `$state` and `$derived`.
3. Build UI components using the **Obsidian Industrial** design tokens, ensuring every interactive element supports keyboard navigation.

### Phase 5: Verification & Static Analysis Gate
Before concluding any task, run and verify:
- Backend check: `cargo check` and `cargo clippy -- -D warnings` inside `src-tauri/`.
- Frontend check: `npm run check` (`svelte-check`) to verify zero TypeScript or Svelte 5 rune compiler warnings.

---

## 2. High-Performance Systems Methodologies

### A. Streaming Data via Tauri v2 `Channel` (Zero UI Blocking)
- **Rule:** Never block a single `invoke` call while scanning thousands of files or games.
- **Method:** Use `tauri::ipc::Channel<T>` to stream discovered `AppEntry` or `GameEntry` items incrementally from Rust to Svelte as they are found, updating the UI progressively in real time.

### B. Icon Extraction & Asset Protocol Caching
- **Rule:** Never send raw Base64 image strings over Tauri IPC—this bloats memory and slows down JSON serialization.
- **Method:**
  1. In Rust, extract `.exe`, `.lnk`, or `.icns` icons once during indexing.
  2. Resize to `128x128` and save as `.webp` or `.png` inside the app's local cache directory (`app.path().app_cache_dir()`), hashed by the executable path.
  3. Store only the cached file path in SQLite, and render it in Svelte using `@tauri-apps/api/core`'s `convertFileSrc(cachedPath)`.

### C. SQLite FTS5 Search Methodology
- **Method:**
  1. Store all indexed apps, games, and pinned folders in a local SQLite database (`rusqlite`) backed by an `FTS5` virtual table.
  2. Wrap bulk index updates inside a single SQLite `Transaction` for sub-10ms writes.
  3. Query FTS5 using prefix matching (`query*`) combined with a `launch_count` + `last_launched` ranking score so frequently used items surface first.

### D. Detached Process Execution
- **Method:** When launching an app or game via `launch_target`, spawn the child process completely detached from FERRO's process tree (e.g., using `CREATE_NEW_PROCESS_GROUP` / `DETACHED_PROCESS` flags on Windows or `setsid` on Unix) so closing or restarting FERRO never terminates running games or apps.

---

## 3. UI & "Obsidian Industrial" Construction Guidelines

### A. Visual Layering Hierarchy
Construct all views using this exact 4-layer stack:
1. **Layer 0 (Canvas):** Full-bleed Obsidian Industrial background (`#0B0D10` + subtle dark metallic texture).
2. **Layer 1 (Glassmorphic Containers):** Elevated panels using `rgba(18, 22, 28, 0.82)`, `backdrop-blur-md`, and `1px solid #262C36` borders.
3. **Layer 2 (Tactile Hardware Bevels):** Inner subtle top/left highlight (`inset 0 1px 0 rgba(255, 255, 255, 0.06)`) to simulate machined steel edges.
4. **Layer 3 (Molten Copper / Cyan Telemetry):** Reserve `#E86A24` / `#FF9E44` strictly for active focus rings, hover borders, and primary actions; reserve `#38BDF8` for live system telemetry and file activity waveforms.

### B. Frameless Window & Drag Regions
- Always apply `data-tauri-drag-region` to the top telemetry strip and empty sidebar headers.
- Explicitly prevent interactive children (buttons, search inputs, window controls) from inheriting drag behavior so clicks register reliably.

### C. Keyboard-First Navigation Pattern
- Maintain a centralized `focusedIndex` (`$state<number>(0)`) in the active view.
- `ArrowDown` / `ArrowUp` / `ArrowLeft` / `ArrowRight` must move focus cleanly across cards and lists.
- `Enter` launches the currently focused item; `Escape` clears the active search query first, and hides the launcher window on a second press.
- Auto-focus the global search input (`Cmd/Ctrl + K` or on window wake) without requiring a mouse click.

---

## 4. Debugging & Troubleshooting Protocol

1. **IPC Serialization Errors:** Check for `snake_case` vs `camelCase` mismatches between Rust structs and TypeScript interfaces.
2. **Missing Icons / Broken Images:** Verify that `assetProtocol` is enabled in `tauri.conf.json` and that the cache directory is permitted in `capabilities/default.json`.
3. **Memory Spikes:** Profile with the top bar's live RAM readout (`sysinfo`). Ensure file watchers (`notify`) debounce rapid filesystem bursts (e.g., during `npm install` or game updates) over a `250ms` window before emitting `folder-updated` events to Svelte.