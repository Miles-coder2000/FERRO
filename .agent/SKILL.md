---
name: ferro-build-and-run
description: Build, compile, and run the FERRO Launcher desktop application (Vite + Svelte 5 frontend and Rust + Tauri 2.0 Windows .exe binary) in D:/FERRO using Windows PowerShell. Use when building, compiling, running dev mode, or verifying ferro-launcher.exe.
---

# FERRO Windows Build, Compilation & Execution Skill

## Workspace & Target Constants
- **Project Root:** `D:/FERRO/`
- **Rust Crate Root:** `D:/FERRO/src-tauri`
- **Target Triple:** `x86_64-pc-windows-msvc`
- **Compiled Frontend:** `D:/FERRO/dist`
- **Debug Binary:** `D:/FERRO/src-tauri/target/debug/ferro-launcher.exe`
- **Release Binary:** `D:/FERRO/src-tauri/target/release/ferro-launcher.exe`

## Execution Steps (Windows PowerShell Only)

### Step 1: Verify Rust MSVC Toolchain & Tauri CLI
```powershell
Set-Location "D:/FERRO"
rustc --version
rustup target add x86_64-pc-windows-msvc
npx tauri --version