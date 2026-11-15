# FERRO Centralized Logging Architecture & Log-Routing Protocol (`D:/FERRO/logs/`)

## Objective
Centralize all log files generated during development, compilation, codebase audits, stress tests, Git syncs, and runtime execution into **`D:/FERRO/logs/`**. Ensure existing scattered `.log` files are migrated cleanly, Rust and frontend tooling write directly to categorized subdirectories inside `D:/FERRO/logs/`, and log writes never trigger Git bloat or infinite filesystem-watcher loops.

---

## 1. Canonical Directory Hierarchy (`D:/FERRO/logs/`)

Organize all logs inside `D:/FERRO/logs/` using these five categorized subdirectories:

```text
D:/FERRO/logs/
├── runtime/        # Rust tracing logs, IPC errors, watcher/SQLite events (ferro-runtime.log, ferro-error.log)
├── build/          # Vite, Cargo, Clippy, and Tauri .exe build outputs (vite-build.log, cargo-build.log, tauri-bundle.log)
├── audit/          # Codebase audit reports, svelte-check, and semgrep security scans (audit-report.log, semgrep.log)
├── stress/         # 100k-file benchmarks, RAM/CPU telemetry, and FERRO_DRY_RUN=1 intercepts (stress-telemetry.log)
└── git/            # Automated Git add/commit/rebase/push sync logs (git-sync.log)
```

---

## 2. Step 1: Initialize Subdirectories & Migrate Existing Scattered Logs

Execute this Windows PowerShell routine to create the canonical folder structure and move any existing `.log` files scattered across `D:/FERRO/`, `D:/FERRO/src-tauri/`, `D:/FERRO/landing/`, or `D:/FERRO/.stress_sandbox/` into `D:/FERRO/logs/`:

```powershell
Set-Location "D:/FERRO"

$logDirs = @(
    "D:/FERRO/logs/runtime",
    "D:/FERRO/logs/build",
    "D:/FERRO/logs/audit",
    "D:/FERRO/logs/stress",
    "D:/FERRO/logs/git"
)

foreach ($dir in $logDirs) {
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $gitkeep = Join-Path $dir ".gitkeep"
    if (-not (Test-Path $gitkeep)) { New-Item -ItemType File -Path $gitkeep | Out-Null }
}

# Find all .log files outside D:/FERRO/logs/ and node_modules/target
Get-ChildItem -Path "D:/FERRO" -Filter "*.log" -Recurse -File -ErrorAction SilentlyContinue |
    Where-Object {
        $_.FullName -notmatch "\\logs\\" -and
        $_.FullName -notmatch "\\node_modules\\" -and
        $_.FullName -notmatch "\\target\\"
    } |
    ForEach-Object {
        $destFolder = switch -Regex ($_.Name) {
            "audit|semgrep|clippy|lint|check" { "D:/FERRO/logs/audit" }
            "stress|bench|telemetry|dry-run"  { "D:/FERRO/logs/stress" }
            "build|vite|cargo|tauri|npm"      { "D:/FERRO/logs/build" }
            "git|sync|push"                   { "D:/FERRO/logs/git" }
            default                           { "D:/FERRO/logs/runtime" }
        }
        $timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
        $destPath = Join-Path $destFolder "$($_.BaseName)-$timestamp.log"
        Move-Item -Path $_.FullName -Destination $destPath -Force
    }
```

---

## 3. Step 2: Rust Backend Log Routing (`tracing` + `tracing-appender`)

Configure `D:/FERRO/src-tauri/src/` so the desktop application writes structured, non-blocking logs to `D:/FERRO/logs/runtime/` during development while remaining portable when distributed as `ferro-launcher.exe`:

1. **Dependencies (`D:/FERRO/src-tauri/Cargo.toml`):**
   - Use `tracing`, `tracing-subscriber` (`features = ["env-filter", "fmt", "time"]`), and `tracing-appender`.
2. **Environment-Aware Log Directory Resolution (`src-tauri/src/utils/logging.rs`):**
   - Check `$env:FERRO_LOG_DIR` first; if unset and `D:/FERRO/logs/runtime` exists on the developer machine, resolve to `D:/FERRO/logs/runtime`.
   - When running as a standalone `.exe` on an end-user machine where `D:/FERRO/` does not exist, fall back automatically to `app.path().app_log_dir()` (`%LOCALAPPDATA%\com.ferro.launcher\logs`) so the binary never fails or creates a stray `D:\FERRO` folder on user systems.
3. **Dual-File Rolling Writer:**
   - Write all `INFO`+ events to `ferro-runtime.log` (daily rotation, retaining the latest 7 files).
   - Write `WARN` and `ERROR` events to a dedicated `ferro-error.log` for rapid failure triage.
   - Keep the `tracing_appender::non_blocking::WorkerGuard` alive inside Tauri's managed app state (`app.manage(LogGuard(guard))`) for the entire application lifetime so buffered logs flush cleanly on exit.

---

## 4. Step 3: Route Agent & CLI Tool Outputs into `D:/FERRO/logs/`

Whenever Cline executes build, audit, stress-test, or Git sync workflows, tee console output into timestamped log files inside `D:/FERRO/logs/`:

- **Frontend & Rust Build Logs (`D:/FERRO/logs/build/`):**
  ```powershell
  Set-Location "D:/FERRO"
  npx vite build *>&1 | Tee-Object -FilePath "D:/FERRO/logs/build/vite-build.log"
  if ($?) {
      Set-Location "D:/FERRO/src-tauri"
      cargo build --release *>&1 | Tee-Object -FilePath "D:/FERRO/logs/build/cargo-build.log"
  }
  ```
- **Codebase Audit & Security Logs (`D:/FERRO/logs/audit/`):**
  ```powershell
  Set-Location "D:/FERRO/src-tauri"
  cargo clippy --all-targets -- -D warnings *>&1 | Tee-Object -FilePath "D:/FERRO/logs/audit/clippy-lint.log"
  Set-Location "D:/FERRO"
  npx svelte-check --tsconfig ./tsconfig.json *>&1 | Tee-Object -FilePath "D:/FERRO/logs/audit/svelte-check.log"
  ```
- **Stress-Test & Dry-Run Logs (`D:/FERRO/logs/stress/`):**
  ```powershell
  Set-Location "D:/FERRO/src-tauri"
  $env:FERRO_DRY_RUN="1"
  $env:FERRO_LOG_DIR="D:/FERRO/logs/stress"
  cargo test --release --test stress_suite -- --nocapture *>&1 | Tee-Object -FilePath "D:/FERRO/logs/stress/stress-telemetry.log"
  ```
- **Runtime `.exe` Launch with Explicit Log Routing (`D:/FERRO/logs/runtime/`):**
  ```powershell
  Set-Location "D:/FERRO/src-tauri"
  $env:RUST_LOG="ferro_launcher=debug,info"
  $env:FERRO_LOG_DIR="D:/FERRO/logs/runtime"
  cargo run --release
  ```

---

## 5. Critical Guardrails (Anti-Loop, Security & Git Hygiene)

1. **Exclude `D:/FERRO/logs/` from the Filesystem Watcher & Indexer:**
   - Hardcode `logs` (and any resolved `app_log_dir`) into the `ignore` / `notify` exclusion list in `src-tauri/src/core/watcher.rs` and `indexer.rs`. If FERRO indexes its own log folder, every log write will trigger a `notify` event, causing an infinite CPU/disk write loop.

2. **`.gitignore` Preservation of Folder Structure:**
   - Add these exact rules to `D:/FERRO/.gitignore` so Git tracks the empty folder hierarchy via `.gitkeep` without committing multi-megabyte `.log` dumps:
     ```gitignore
     logs/**/*.log
     !logs/**/.gitkeep
     ```

3. **Log Redaction & Retention Policy:**
   - Never log raw environment secrets, tokens, or full user home directory listings at `INFO` level.
   - Prune log files older than 14 days during maintenance:
     ```powershell
     Get-ChildItem -Path "D:/FERRO/logs" -Filter "*.log" -Recurse -File |
         Where-Object { $_.LastWriteTime -lt (Get-Date).AddDays(-14) } |
         Remove-Item -Force
     ```
