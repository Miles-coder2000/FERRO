# FERRO Canonical Workspace & Path Resolution Rules

## Mandatory Project Root Constants
Whenever a tool call, MCP server argument, terminal command, or file operation requires the project path, use these exact directories—never guess relative paths, user home directories (`C:/Users/...`), or placeholder strings:

- **Primary Project Root:** `D:/FERRO/` (Windows native: `D:\FERRO\`)
- **Rust & Tauri Backend Root:** `D:/FERRO/src-tauri` (Windows native: `D:\FERRO\src-tauri\`)
- **Svelte 5 Frontend Root:** `D:/FERRO/src`
- **Frontend Compiled Bundle:** `D:/FERRO/dist`
- **Landing Page Directory:** `D:/FERRO/landing`
- **Agent Configuration Directory:** `D:/FERRO/.agent`
- **Dev SQLite Index Database:** `D:/FERRO/src-tauri/dev-data/ferro_dev.db`
- **Compiled Debug Executable:** `D:/FERRO/src-tauri/target/debug/ferro-launcher.exe`
- **Compiled Release Executable:** `D:/FERRO/src-tauri/target/release/ferro-launcher.exe`

---

## Strict Path & Working Directory Rules for Cline

1. **Zero Placeholder or Guessed Paths:**
   - Never pass `/ABSOLUTE/PATH/TO/FERRO`, `./`, `../`, or `C:\...` paths to MCP tools or shell commands.
   - Always resolve project files strictly under `D:/FERRO/` and Rust crate files strictly under `D:/FERRO/src-tauri`.

2. **Context-Specific Working Directory Routing:**
   - **Frontend, Tauri CLI & Git Commands (Must run in `D:/FERRO/`):**
     - `npm install`, `npm run build`, `npx vite build`, `npx svelte-check`
     - `npx tauri dev`, `npx tauri build`, `npx tauri build --no-bundle`
     - `git status`, `git add`, `git commit`, `git pull`, `git push`
   - **Rust & Cargo Commands (Must run in `D:/FERRO/src-tauri`):**
     - `cargo check`, `cargo clippy`, `cargo test`, `cargo build`, `cargo run`
     - *(Alternative from `D:/FERRO/`: pass `--manifest-path D:/FERRO/src-tauri/Cargo.toml` explicitly.)*

3. **Persistent Shell `cwd` Guardrail:**
   - Because Cline's terminal session retains its previous working directory between commands, **always verify or explicitly reset the directory** before running a command:
     - Before root/frontend commands: `Set-Location "D:/FERRO"` (PowerShell) or `cd /d D:\FERRO` (CMD)
     - Before Rust/Cargo commands: `Set-Location "D:/FERRO/src-tauri"` (PowerShell) or `cd /d D:\FERRO\src-tauri` (CMD)

4. **Forward-Slash Normalization for MCP & JSON Payloads:**
   - Always format paths with forward slashes (`D:/FERRO/...` and `D:/FERRO/src-tauri/...`) when calling `filesystem`, `cargo`, `semgrep`, `sqlite`, or `git` MCP tools. Forward slashes are natively supported by Windows APIs, Rust, Node.js, and JSON-RPC without backslash escaping errors (`\t`, `\s`, `\f`).

5. **Compile-Time Workspace Paths vs. Runtime `.exe` Paths:**
   - Use `D:/FERRO/` and `D:/FERRO/src-tauri` **only** for development, building, testing, and MCP tool calls.
   - **NEVER** hardcode `D:/FERRO/` inside production Rust (`src-tauri/src/*.rs`) or Svelte (`src/**`) source code. Inside the compiled `ferro-launcher.exe`, always resolve user data, icons, and SQLite databases dynamically via Tauri's `app.path().app_data_dir()`, `app.path().app_cache_dir()`, and `app.path().resource_dir()`.