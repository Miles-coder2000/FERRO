# FERRO Windows Build, Compilation & Execution Protocol

## Workspace & Target Constants
- **Project Root:** `D:\FERRO`
- **Rust Crate Root:** `D:\FERRO\src-tauri`
- **Target Triple:** `x86_64-pc-windows-msvc`
- **Frontend Output:** `D:\FERRO\dist`
- **Debug Binary:** `D:\FERRO\src-tauri\target\debug\ferro-launcher.exe`
- **Release Binary:** `D:\FERRO\src-tauri\target\release\ferro-launcher.exe`

---

## Step 1: Verify & Install Prerequisites
Before building, verify that the Windows Rust MSVC toolchain, WebView2 runtime, and Tauri CLI are installed and available on `PATH`:

1. **Rust Toolchain (`1.70+` with MSVC ABI):**
   - Check current version via `rustc --version` and `rustup toolchain list`.
   - If Rust is missing or outdated, install/update the stable MSVC toolchain and register the Windows target:
     ```powershell
     rustup toolchain install stable-x86_64-pc-windows-msvc
     rustup default stable-x86_64-pc-windows-msvc
     rustup target add x86_64-pc-windows-msvc
     ```
   - *Note:* Do not pass literal `1.70+` to `rustup install`; always install `stable` (or a specific semver like `1.85.0`) and verify `rustc --version` satisfies `>= 1.70.0`.
2. **Windows Native Build Dependencies:**
   - Confirm Microsoft C++ Build Tools (`link.exe` / Windows SDK) and Microsoft Edge WebView2 Runtime are present.
3. **Tauri CLI:**
   - Verify `npx tauri --version` works locally, or install globally if missing:
     ```powershell
     npm install -g @tauri-apps/cli
     ```

---

## Step 2: Install Frontend Dependencies
Navigate to the project root and install Node packages:

```powershell
cd D:\FERRO
npm install