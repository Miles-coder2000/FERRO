# FERRO Core Instructions (`.clinerules/00-ferro-instructions.md`)

> Registers project-wide conventions and references the centralized logging protocol.

## Reference: Centralized Logging Architecture

The FERRO logging protocol is defined in **`@.agent/others/logging.md`** (source) and mirrored at **`D:/FERRO/.agent/logging.md`**. All build, audit, stress-test, Git sync, and runtime logs must be routed into `D:/FERRO/logs/` per that document:

- `logs/runtime/` — Rust tracing, IPC errors, watcher/SQLite events
- `logs/build/` — Vite, Cargo, Clippy, Tauri build outputs
- `logs/audit/` — codebase audits, svelte-check, semgrep scans
- `logs/stress/` — benchmarks, telemetry, dry-run intercepts
- `logs/git/` — Git sync/add/commit/rebase/push logs

Each subdirectory contains a `.gitkeep` file so the empty folder hierarchy is tracked by Git while `logs/**/*.log` is ignored (see `D:/FERRO/.gitignore`).

## General Directives

- Always resolve project paths strictly under `D:/FERRO/` (never `C:/...` or placeholders).
- Rust/Cargo commands run in `D:/FERRO/src-tauri/`; frontend/Tauri/git commands run in `D:/FERRO/`.
- Before building or running, verify the target triple `x86_64-pc-windows-msvc` is installed (`rustup target add x86_64-pc-windows-msvc`).
- Exclude `D:/FERRO/logs/` from the filesystem watcher and indexer to prevent infinite notify loops.
