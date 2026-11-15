# FERRO Agent Rules

## 1. Core Directives & Philosophy
- **Performance is paramount:** FERRO must remain a lightweight, blazing-fast launcher. Always optimize for < 150ms startup time and < 50MB idle RAM.
- **Native over Web:** Whenever a task can be done via Rust OS-level APIs instead of JavaScript, do it in Rust.
- **Zero Electron:** Never suggest Electron, Node.js backend dependencies, or Chromium-heavy solutions.

## 2. Tech Stack Boundaries
- **Backend:** Strictly Rust with Tauri 2.0. Do NOT use Tauri 1.x legacy APIs.
- **Frontend:** Strictly Svelte 5 (using Runes: `$state`, `$derived`, `$effect`) with TypeScript. Do NOT use Svelte 4 legacy syntax (e.g., `export let`, `$:` reactive statements).
- **Styling:** Tailwind CSS v4. Do not use external component libraries (like Material UI) that conflict with the custom Obsidian Industrial theme.

## 3. Rust Backend Rules (`src-tauri`)
- **No panics:** Never use `.unwrap()` or `.expect()` in production commands. Always return a `Result<T, String>` or custom error type mapped to a string for the frontend.
- **Async First:** Use `tokio` for any I/O, file system operations, or process spawning to prevent blocking the main Tauri thread.
- **IPC Safety:** All `#[tauri::command]` inputs must be strictly typed. Validate file paths before execution.

## 4. Frontend & UI Rules (`src`)
- **Obsidian Industrial Theme:** Strictly adhere to the color palette:
  - Base: `#0B0D10`
  - Cards: `rgba(18, 22, 28, 0.82)` with backdrop blur.
  - Accents: `#E86A24` (Copper) and `#38BDF8` (Cyan).
- **Frameless Window:** Assume the window has no native OS title bar. All window controls (minimize, close, drag) must be custom Svelte components utilizing Tauri's window APIs (`appWindow.minimize()`, `data-tauri-drag-region`).
- **Keyboard First:** All lists, grids, and search bars must be fully navigable via keyboard (Arrow keys, Enter, Escape).

## 5. Code Quality & Commits
- Write modular, single-responsibility functions.
- Add minimal, high-value comments explaining *why* (especially for OS-specific Rust implementations), not *what*.
- Ensure all Svelte components are strongly typed with TypeScript interfaces for their props.