# FERRO Landing Page — Vercel Production Deployment Protocol

## Objective
Configure, build, and publish the official **FERRO Launcher** product landing page and download portal (`D:/FERRO/landing/`) to **Vercel** in production mode, verify live HTTP and Open Graph (`og-preview.png`) asset delivery, and sync deployment configs back to GitHub.

---

## Workspace & Deployment Constants
- **Project Root:** `D:/FERRO/`
- **Landing Page Directory (Vercel Root Directory):** `D:/FERRO/landing`
- **Compiled Static Output:** `D:/FERRO/landing/dist`
- **Open Graph Preview Image:** `D:/FERRO/landing/public/assets/og-preview.png`
- **Build & Deploy Log Output:** `D:/FERRO/logs/build/vercel-deploy.log`
- **GitHub Repository:** `https://github.com/Miles-coder2000/FERRO.git`

---

## Strict Operational & Anti-Hang Guardrails
1. **Windows PowerShell Only:** Use `Set-Location`, `;`, `if ($?) { ... }`, and `$env:KEY="VAL"`. Never use Bash syntax (`&&`, `rm -rf`, `export`).
2. **No Interactive Auth Hangs:** Never run interactive `vercel login` inside Cline's unattended terminal. Check `npx --yes vercel whoami` first; if unauthenticated, pause and instruct the user to run `npx vercel login` in their own terminal tab or connect the repository in the Vercel Dashboard.
3. **Log All Outputs:** Route build and deployment logs into `D:/FERRO/logs/build/` so verbose CLI output never freezes Cline's terminal buffer.

---

## Phase 1: Pre-Deployment Asset, SEO & Build Verification
Before touching Vercel settings, verify that `D:/FERRO/landing/` compiles cleanly with all branding and Open Graph assets in place:

1. **Verify Required Static Assets:**
   Confirm these files exist inside `D:/FERRO/landing/public/assets/`:
   - `og-preview.png` (`1200x630` social share card)
   - `ferro-logo.png` (Iron Helix emblem)
   - `ferro-bg.png` (Obsidian Industrial background)

2. **Verify `D:/FERRO/landing/index.html` Metadata:**
   Ensure `<head>` contains:
   - `<title>FERRO Launcher — Lightweight Rust & Tauri 2.0 Desktop Organizer</title>`
   - `<meta name="description" content="Unify your games, apps, system folders, and CLI commands into one blazing-fast, sub-50MB Windows launcher built with Rust and Tauri 2.0." />`
   - `<meta property="og:title" ... />`, `<meta property="og:description" ... />`, `<meta property="og:type" content="website" />`
   - `<meta name="twitter:card" content="summary_large_image" />`

3. **Run Local Production Build Gate:**
   ```powershell
   Set-Location "D:/FERRO/landing"
   $env:CI = "true"
   npm install --no-audit --no-fund *>&1 | Out-File -FilePath "D:/FERRO/logs/build/landing-install.log" -Encoding UTF8
   if ($?) {
       npm run build *>&1 | Tee-Object -FilePath "D:/FERRO/logs/build/landing-build.log"
   }
   Test-Path "D:/FERRO/landing/dist/index.html"