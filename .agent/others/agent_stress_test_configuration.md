# FERRO Automated Stress-Test & Chaos Engineering Protocol

## Objective
Execute a full-stack stress test of the **FERRO Launcher** (Rust + Tauri v2 + Svelte 5 + SQLite FTS5) inside an isolated synthetic sandbox. Validate memory ceilings, IPC concurrency, filesystem watcher resilience, UI frame stability, and security boundaries without touching real user directories.

---

## Strict Safety Rules During Testing
1. **Sandbox Isolation Only:** All generated test files, folders, symlinks, and dummy binaries MUST reside strictly inside `./.stress_sandbox/` (ensure `.stress_sandbox/` is listed in `.gitignore`). Never point stress generators at `~/Desktop`, `~/Documents`, `C:\`, or `/` root paths.
2. **Non-Destructive Execution:** Mock or intercept actual `Command::spawn` calls with a dry-run flag (`FERRO_DRY_RUN=1`) during fuzzing so adversarial launch tests never execute unintended system binaries.
3. **Mandatory Cleanup:** Delete `./.stress_sandbox/` and vacuum/reset `src-tauri/dev-data/ferro_dev.db` immediately after generating the final report.

---

## Phase 1: Synthetic Chaos Fixture Generation
Use the `filesystem` MCP or a Rust test harness to populate `./.stress_sandbox/` with the following edge-case dataset:
- **Scale & Depth:** 100,000 dummy files across 10,000 nested directories, including a deep branch 25 folders down.
- **Adversarial Filenames:**
  - Shell injection strings: `game; rm -rf .exe`, `app$(whoami).lnk`, `test & calc.exe`, `'quoted "path" name'.bat`
  - Unicode, RTL, and emoji names: `🚀_Cyber_Forge_日本語_файл.exe`, zero-width spaces, and 240-character near-MAX_PATH filenames.
- **Broken & Hostile Filesystem Entries:**
  - Circular directory symlinks (`loop_a -> loop_b -> loop_a`) to verify symlink loop detection.
  - 0-byte corrupted `.exe` / `.lnk` / `.desktop` files and unreadable/permission-denied subdirectories.
  - 500 high-resolution dummy cover images to stress thumbnail caching and `asset:` protocol loading.

---

## Phase 2: Rust Backend & SQLite Concurrency Stress (`cargo` + `sqlite` MCP)
Run release-mode stress benchmarks (`cargo test --release --test stress_suite -- --nocapture`) and inspect `ferro_dev.db` via `sqlite` MCP:
1. **Cold & Hot Indexer Flood:**
   - Trigger a full sector scan of `./.stress_sandbox/` (100,000 items).
   - **Pass Criteria:** Completes in under 3.0 seconds, ignores circular symlinks without stack overflow or infinite loops, and logs permission errors gracefully without panicking.
2. **Live Watcher Churn (`notify` crate):**
   - While FERRO is idle, programmatically create, rename, and delete 1,000 files within 2 seconds inside `./.stress_sandbox/`.
   - **Pass Criteria:** Event debouncing prevents UI or SQLite write storms; batch commits complete cleanly with zero `SQLITE_BUSY` lock errors.
3. **IPC & FTS5 Search Concurrency:**
   - Fire 500 concurrent `invoke("search_items", ...)` and `invoke("launch_item", ...)` requests across multiple async tasks.
   - **Pass Criteria:** p99 FTS5 query latency stays under 15ms; 100% of path-traversal (`../`) and shell-metacharacter payloads are rejected by canonicalization guards.

---

## Phase 3: RAM, CPU & Leak Profiling
Monitor the running `ferro` process RSS memory and CPU utilization across four stages:
1. **Stage A (Cold Idle):** App open on **The Forge** with 100k indexed items, no user input for 30 seconds.
   - **Target:** RAM under 50MB, CPU under 1.0%.
2. **Stage B (Active Re-Index + Search Burst):** Simultaneous background re-scan + rapid query typing.
   - **Target:** Peak RAM under 120MB, zero thread starvation.
3. **Stage C (Post-Load Cooldown):** 60 seconds after burst completes.
   - **Target:** Memory drops back within 10% of Stage A baseline (verifies no unbounded image/icon cache leak).

---

## Phase 4: Svelte 5 UI & WebView Torture Test (`tauri` MCP)
Launch the dev build (`cargo tauri dev`) with `FERRO_DRY_RUN=1` and use `@hypothesi/tauri-mcp-server` to automate the live WebView:
1. **Rapid Sector Navigation:** Switch between **The Forge**, **Armory**, **Stack**, and **Folders** 50 times in 10 seconds.
   - **Verify:** Zero Svelte 5 `$effect` cycle errors, zero uncaught promise rejections in the WebView console.
2. **Search & CLI Input Storm:** Inject rapid keystrokes (20 chars/sec), instant clear/backspace, and a 5,000-character paste into the top search bar.
   - **Verify:** Virtualized list/grid renders smoothly without main-thread frame drops or input lag.
3. **Visual Layout & Glassmorphism Audit:** Capture screenshots via `tauri` MCP at `1920x1080`, `1280x720`, and narrow `900x600` viewports while displaying 240-character filenames and missing icons.
   - **Verify:** Obsidian Industrial panel borders, text truncation (`text-overflow: ellipsis`), and copper-amber focus badges remain aligned with zero horizontal overflow.

---

## Phase 5: Security Audit, Cleanup & Report Output
1. Run `semgrep` (`security_check`) and `cargo clippy --all-targets -- -D warnings` to ensure test harnesses introduced zero warnings or unsafe patterns.
2. Delete `./.stress_sandbox/` and restore `ferro_dev.db` to its clean seed state.
3. Output a **FERRO Stress-Test Telemetry Report** containing:
   - Summary table of **Pass / Fail** status per phase.
   - Exact measured metrics (Cold RAM, Peak RAM, Index Time for 100k files, p50/p99 Search Latency).
   - Ranked list of any bottlenecks, memory spikes, or layout clipping found, with concrete code fixes ready to apply.