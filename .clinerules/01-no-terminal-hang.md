# FERRO Terminal Anti-Hang & Non-Blocking Execution Rules

## Objective
Prevent terminal deadlocks, infinite waits, interactive prompt freezes, and orphaned background processes when Cline executes commands in Windows PowerShell inside `D:/FERRO/` and `D:/FERRO/src-tauri`.

---

## Rule 1: Never Run Blocking Foreground Servers or Watchers
Cline's command runner waits for the foreground process group and stdout/stderr pipes to close. Running a dev server or GUI app directly in the foreground will hang Cline indefinitely.
- **FORBIDDEN Commands in Cline Terminal:**
  - `npx tauri dev`
  - `npx vite` / `npm run dev`
  - `cargo watch`
  - `cargo run` / `cargo run --release` (unless wrapped in a strict timeout or dry-run exit flag)
  - Direct foreground execution of `ferro-launcher.exe` without detachment
- **REQUIRED Alternatives:**
  - **For compilation & verification:** Always use finite build commands that exit automatically:
    - `Set-Location "D:/FERRO"; npx vite build`
    - `Set-Location "D:/FERRO/src-tauri"; cargo check`
    - `Set-Location "D:/FERRO"; npx tauri build --no-bundle`
  - **When a live dev server is needed:** Ask the user to run `npx tauri dev` in their own dedicated terminal tab instead of launching it inside Cline's command tool.
  - **When launching `ferro-launcher.exe` for quick smoke tests:** Detach it cleanly with redirected streams so it never holds Cline's PTY/pipe open:
    ```powershell
    Start-Process -FilePath "D:/FERRO/src-tauri/target/release/ferro-launcher.exe" -WorkingDirectory "D:/FERRO/src-tauri/target/release" -WindowStyle Normal
    ```

---

## Rule 2: Enforce Non-Interactive Flags on Every CLI Tool
A single `[Y/n]` or `Ok to proceed?` prompt will stall Cline forever because stdin is unattended.
- **Mandatory Environment Variables (Set before package/git commands):**
  ```powershell
  $env:CI = "true"
  $env:npm_config_yes = "true"
  $env:GIT_TERMINAL_PROMPT = "0"
  $env:GCM_INTERACTIVE = "never"
  ```
- **Mandatory CLI Flags:**
  - **`npx`:** Always pass `--yes` (`npx --yes vite build`, `npx --yes svelte-check`).
  - **`npm`:** Pass `--no-audit --no-fund --yes` when installing or scaffolding.
  - **`git`:** Never run bare `git commit` or `git rebase -i`. Always pass `-m "<message>"` on commits, `--no-edit` on merges/cherry-picks, and `--no-pager` on inspection commands (`git --no-pager status`, `git --no-pager diff`, `git --no-pager log -n 10`).
  - **`winget` / installers:** Always append `--accept-package-agreements --accept-source-agreements --silent`.

---

## Rule 3: Prevent Pager & Console Buffer Freezes
- **Disable Pagers Globally in Session:**
  ```powershell
  $env:PAGER = "cat"
  $env:GIT_PAGER = "cat"
  ```
- **Cap Verbose Build Output:** Never dump tens of thousands of raw compiler lines directly into Cline's terminal capture. Route verbose output to `D:/FERRO/logs/build/` and print only the summary or last 40 lines:
  ```powershell
  Set-Location "D:/FERRO/src-tauri"
  cargo build --release *>&1 | Out-File -FilePath "D:/FERRO/logs/build/cargo-build.log" -Encoding UTF8
  Get-Content "D:/FERRO/logs/build/cargo-build.log" -Tail 40
  ```

---

## Rule 4: Wrap Risky Tests & Binaries in a Hard PowerShell Timeout
Whenever running integration tests (`cargo test`), stress suites, or custom scripts that could deadlock on a mutex or filesystem lock, wrap execution in a PowerShell background job with a strict timeout (default `45` seconds) and forced process cleanup:

```powershell
Set-Location "D:/FERRO/src-tauri"
$job = Start-Job -ScriptBlock {
    Set-Location "D:/FERRO/src-tauri"
    $env:FERRO_DRY_RUN = "1"
    cargo test --release --test stress_suite -- --nocapture 2>&1
}
if (Wait-Job $job -Timeout 45) {
    Receive-Job $job
    Remove-Job $job -Force
} else {
    Stop-Job $job
    Remove-Job $job -Force
    Get-Process -Name "ferro-launcher", "stress_suite*" -ErrorAction SilentlyContinue | Stop-Process -Force
    Write-Error "Command timed out after 45s and was terminated to prevent terminal hang."
}
```

---

## Rule 5: Pre-Command Orphan Process & Lock Cleanup
Before running `cargo build`, `npx tauri build`, or `npm install`, check for and stop lingering background processes that hold file locks on `ferro-launcher.exe` or `node_modules` (which otherwise cause Cargo/npm to hang retrying file locks):
```powershell
Get-Process -Name "ferro-launcher" -ErrorAction SilentlyContinue | Stop-Process -Force
```

---

## Rule 6: Prefer MCP Servers Over Shell Spawning
Whenever a task can be performed via an installed MCP server in `cline_mcp_settings.json`, **use the MCP tool instead of `execute_command`**:
- File reading, writing, searching, and directory listing → `filesystem` MCP
- `cargo check`, `cargo clippy`, `cargo test` → `cargo` MCP
- SQLite schema and query inspection → `sqlite` MCP
- Git status, diff, staging, and local commits → `git` MCP
- Static security checks → `semgrep` MCP