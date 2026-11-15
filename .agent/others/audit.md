# FERRO Full-Stack Codebase Audit & Cleanup Protocol

## Objective
Perform a read-first, end-to-end diagnostic audit of the **FERRO Launcher** (`src-tauri/` Rust backend, `src/` Svelte 5 frontend, SQLite schema, and Tauri v2 configuration) to identify:
1. Compiler errors, type mismatches, and silent runtime bugs.
2. UI/UX glitches, layout overflows, and broken asset paths.
3. Redundant code, unused crates/npm packages, duplicate IPC handlers, and dead files.
4. Concurrency hazards, memory leaks, and Tauri v2 capability/CSP misconfigurations.

## Non-Destructive Audit Rules
- **Audit First, Patch Second:** Do NOT delete files, strip dependencies, or refactor logic during Phases 1–5. Collect all findings with exact `file:line` references first.
- **No Blind Auto-Removal:** Never remove a crate, Svelte component, or CSS class solely on heuristic grep—verify cross-language references (e.g., Rust `#[tauri::command]` names invoked dynamically via `invoke("...")` in TypeScript, or `tauri.conf.json` plugin hooks).

---

## Phase 1: Compiler, Type-Checker & Lint Verification
Run strict static checks across both layers using the `cargo` MCP and project scripts:
1. **Rust Backend (`src-tauri/`):**
   - Execute `cargo clippy --all-targets --all-features -- -D warnings`
   - Flag any `.unwrap()`, `.expect()`, `panic!()`, or `todo!()` inside production `src-tauri/src/` paths (outside `#[cfg(test)]`).
2. **Svelte 5 & TypeScript Frontend (`src/`):**
   - Execute `npx svelte-check --tsconfig ./tsconfig.json` and `npm run build`
   - Flag any legacy Svelte 4 syntax (`export let`, `$:`, `createEventDispatcher`) mixed into Svelte 5 Runes (`$state`, `$derived`, `$effect`, `$props`) components.

---

## Phase 2: Redundancy, Dead Code & Dependency Bloat Sweep
Inspect the dependency graph and module tree for unused or duplicated artifacts:
1. **Unused Rust Crates & Symbols:**
   - Run `cargo machete` (or inspect `Cargo.toml` vs `use` imports) to detect unused dependencies inflating compile time and binary size.
   - Check for `#[allow(dead_code)]` suppressions hiding abandoned structs, traits, or helper functions.
2. **Unused Frontend Packages, Exports & Assets:**
   - Run `npx knip` (or trace entry points from `src/main.ts` / `App.svelte`) to list unused npm packages, unreferenced `.svelte` components, dead TypeScript exports, and orphaned CSS rules.
   - Check `public/` and `src/assets/` for unreferenced images or duplicate icons.
3. **Cross-Layer Redundancy:**
   - Cross-reference every `#[tauri::command]` registered in `tauri::generate_handler![...]` against all `invoke(...)` calls in `src/` to find orphaned backend commands or mismatched payload parameter names.
   - Inspect SQLite queries for duplicate `SELECT` statements, missing indexes on `WHERE`/`ORDER BY` columns, or redundant database connection initializations.

---

## Phase 3: Runtime Bugs, Concurrency & Portable `.exe` Hazards
Audit high-risk failure modes specific to desktop launchers:
1. **Async Runtime Blocking:** Verify that heavy filesystem walks, icon extractions, and SQLite writes run via `tokio::task::spawn_blocking` or dedicated worker threads—never blocking Tauri's main UI/IPC thread.
2. **Filesystem & Watcher Edge Cases:**
   - Check `notify` watcher handlers for missing event debouncing, unhandled `PermissionDenied` errors, or infinite recursion on circular symlinks.
   - Verify canonical path handling on Windows (`\\?\` extended-length path prefixes returned by `std::fs::canonicalize` that can break third-party game executables if not stripped before `Command::new`).
3. **Standalone `.exe` Path Resolution:**
   - Audit all file reads for hardcoded relative paths (`../`, `dev-data/`, `src/`) that fail when `ferro.exe` runs outside the repo; confirm all runtime state resolves via Tauri's `app.path().app_data_dir()` and `app.path().resource_dir()`.
4. **Svelte 5 Reactivity & Listener Leaks:**
   - Inspect every `$effect` block for circular state mutations (infinite re-render loops).
   - Verify every Tauri `listen(...)` event listener registered in a component has a matching `unlisten()` cleanup on destroy.

---

## Phase 4: Live UI/UX & Visual Glitch Audit (`tauri` MCP)
Start the live app and inspect the rendered WebView via `@hypothesi/tauri-mcp-server`:
1. **Console & IPC Error Capture:** Navigate through **The Forge**, **Armory**, **Stack**, **Folders**, and **Settings** while recording all WebView console warnings, failed `asset:` protocol image loads, and rejected IPC promises.
2. **Obsidian Industrial Visual & Layout Checks:**
   - Capture screenshots at `1920x1080`, `1280x720`, and `900x600`.
   - Check for clipped glassmorphic borders, overlapping `z-index` modals/tooltips, missing fallback icons on `.exe`/folder cards, broken keyboard focus rings, and horizontal scrollbars caused by long paths.

---

## Phase 5: Tauri v2 Security & Capability Audit (`semgrep` MCP)
1. Run `semgrep` (`security_check`) across `src-tauri/` and `src/`.
2. Audit `src-tauri/tauri.conf.json` and `src-tauri/capabilities/*.json`:
   - Confirm `app.security.csp` is non-null and restricts `script-src 'self'`.
   - Confirm `withGlobalTauri` is disabled (`false`) and `freezePrototype` is enabled (`true`).
   - Verify no wildcard filesystem or shell permissions exist and all custom `launch_item` commands enforce path canonicalization without shell interpolation.

---

## Phase 6: Diagnostic Report & Interactive Fix Plan
Output a structured **FERRO Codebase Audit Report** organized by severity:
- **P0 — Critical Bugs & Security Flaws:** Crashes, panics, command injection, or broken `.exe` runtime paths.
- **P1 — Runtime Glitches & Performance Bottlenecks:** UI freezes, watcher storms, memory/listener leaks, or layout breakage.
- **P2 — Redundancy & Dead Code:** Unused crates/npm packages, orphaned IPC commands, duplicate queries, and dead components.
- **P3 — Code Hygiene & Warnings:** Clippy/Svelte-check warnings and stylistic inconsistencies.

For every item, provide:
- **Location:** `path/to/file:line`
- **Root Cause:** Concise technical explanation
- **Proposed Fix:** Exact diff or command to resolve it

End by asking which severity tiers (`P0`–`P3`) I approve you to patch automatically.