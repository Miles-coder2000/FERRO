# FERRO Launcher — Official Landing Page & Download Portal Specification

## Objective
Build a fast, responsive, production-ready product landing page and download portal for **FERRO Launcher** inside `D:\FERRO\landing\` (deployable as a static site to Vercel, Netlify, or GitHub Pages). The site must showcase FERRO's Rust + Tauri 2.0 performance, let visitors test an interactive browser simulation of **The Forge** UI, and provide direct one-click downloads for the Windows installer and standalone `.exe` via the GitHub Releases API.

---

## Tech Stack & Project Structure
- **Framework:** Vite + **Svelte 5 (Runes: `$state`, `$derived`, `$effect`)** + TypeScript (matches the desktop app's frontend DNA for shared styling and maintainability).
- **Styling:** Tailwind CSS v4 (or v3) + custom CSS tokens for the **Obsidian Industrial / Copper Cyber** theme.
- **Icons:** `lucide-svelte` + custom inline SVG for the **FERRO Iron Helix** emblem.
- **Target Directory:** `D:\FERRO\landing\`
  - `landing/public/assets/` — Copy the standalone FERRO logo (`ferro-logo.png`) and Obsidian Industrial background (`ferro-bg.png`) here, plus generate `favicon.ico` and `og-preview.png`.
  - `landing/src/lib/github.ts` — GitHub Releases API client (`https://api.github.com/repos/Miles-coder2000/FERRO/releases/latest`) with `localStorage` caching (15-minute TTL) and graceful fallback links to `https://github.com/Miles-coder2000/FERRO/releases/latest`.
  - `landing/src/lib/components/` — Modular Svelte 5 sections (`Navbar.svelte`, `HeroForgeDemo.svelte`, `Benchmarks.svelte`, `SectorsGrid.svelte`, `ArchitectureSecurity.svelte`, `DownloadHub.svelte`, `Footer.svelte`).

---

## Visual Identity: "Obsidian Industrial / Copper Cyber"
1. **Color Tokens:**
   - **Obsidian Void (Page Background):** `#090A0C`
   - **Tungsten Surface (Cards & Panels):** `#121418` with `rgba(18, 20, 24, 0.78)` + `backdrop-filter: blur(14px)`
   - **Machined Border:** `1px solid rgba(255, 255, 255, 0.08)` (hover: `rgba(232, 93, 4, 0.55)`)
   - **Primary Copper-Rust Accent:** `#E85D04`
   - **Molten Amber Highlight / Glow:** `#F48C06`
   - **Primary Text:** `#F1F3F5` | **Secondary Text:** `#9CA3AF` | **Telemetry Monospace:** `#D97706`
2. **Typography (Google Fonts):**
   - **Display Headings:** `Chakra Petch` or `Rajdhani` (weights 600, 700, uppercase tracking for industrial section badges).
   - **Body Copy:** `Inter` (weights 400, 500, 600).
   - **Telemetry, Checksums & CLI Snippets:** `JetBrains Mono` (weights 400, 500).
3. **Atmospheric Details:**
   - Subtle CSS machined-steel grid background overlay + soft radial copper forge glow behind the hero launcher preview.
   - Chamfered/sharp industrial card corners (`rounded-md` / `6px`) with subtle top-edge specular highlights.
   - Respect `@media (prefers-reduced-motion: reduce)` by disabling ambient ember/glow pulse animations.

---

## Page Sections & Functional Requirements

### 1. Sticky Glassmorphic Navbar (`Navbar.svelte`)
- **Left:** FERRO circular logo badge + `FERRO` wordmark + dynamic release version pill (`v0.1.0` fetched from GitHub Releases, fallback `WINDOWS x64`).
- **Center (Desktop):** Anchor links to `#sectors`, `#benchmarks`, `#architecture`, and `#download`.
- **Right:**
  - Secondary ghost button: **GitHub** (links to `https://github.com/Miles-coder2000/FERRO` with star/repo icon).
  - Primary glowing copper CTA button: **Download `.exe`** (smooth-scrolls to `#download` or triggers direct download).

### 2. Hero Section & Interactive Web Preview (`HeroForgeDemo.svelte`)
- **Eyebrow Telemetry Pill:** `ENGINE: RUST + TAURI 2.0 • < 50 MB IDLE RAM • ZERO ELECTRON BLOAT`
- **Headline:** *"All Your Games, Apps & System Folders—Forged Into One Launcher."*
- **Subheadline:** *"Stop juggling five bloated launchers and cluttered desktop shortcuts. FERRO unifies your PC games, developer tools, deep directories, and custom CLI commands into an ultra-lightweight native Windows command center."*
- **Primary Download CTA Row:**
  - **Primary Button:** `Download Windows Installer (.exe)` — automatically resolves the `.exe` NSIS setup asset from GitHub Releases, displaying dynamic file size (e.g., `4.8 MB • Windows 10/11 64-bit`).
  - **Secondary Button:** `Portable Standalone (.exe)` — resolves `ferro-launcher.exe` or links to the latest GitHub Release assets.
  - **Sub-CTA Metadata Line:** Displays latest tag name, release publish date, `Free & Open Source`, and a clickable **"Verify SHA-256 / Release Notes"** modal/drawer.
- **Interactive Browser Mockup ("Try The Forge Live"):**
  - Embed an interactive, self-contained HTML/Svelte replica of the FERRO desktop window right below the hero CTAs:
    - Users can click sidebar tabs (**Home / The Forge**, **Armory (Games)**, **Stack (Apps)**, **Folders**) to switch views live.
    - Users can type into the simulated `Search anything or launch with Rust CLI...` bar to filter mock games, apps, and folders in real time with simulated `< 12 ms` FTS5 latency readouts.
    - Live simulated top-right telemetry ticker (`CPU: 0.3% | RAM: 38.4 MB | Latency: 0.35 ms`).

### 3. Resource & Speed Benchmarks (`Benchmarks.svelte`)
- Visual side-by-side animated progress bars comparing **FERRO (Rust + Tauri 2.0)** against **Standard Electron Launchers**:
  1. **Idle Memory Footprint:** `38 MB` (FERRO) vs `320 MB` (Electron Launchers) — *~88% less RAM*
  2. **Installer Package Size:** `~4.8 MB` (FERRO) vs `115 MB` (Electron Launchers) — *20x smaller download*
  3. **100,000-File Search Latency:** `< 15 ms` via SQLite FTS5 vs `140+ ms`
  4. **Cold Boot to Interactive:** `< 180 ms` vs `2.1 seconds`

### 4. Unified Sectors Bento Grid (`SectorsGrid.svelte`)
Four high-contrast Obsidian Industrial cards detailing core capabilities:
- **01 // THE FORGE (Command Dashboard):** Pin frequent projects, monitor live hardware telemetry, and jump straight back into recent sessions.
- **02 // ARMORY (Unified Game Library):** Automatically indexes local titles across drives with custom cover art, launch arguments, and instant native process execution.
- **03 // STACK (Apps & Developer Tools):** Group IDEs, terminals, creative software, and custom scripts into zero-clutter workspaces.
- **04 // ACTIVE SYSTEM FOLDERS:** Asynchronous Rust directory scanning with live filesystem watchers (`notify`) and instant file-count badges.

### 5. Rust Architecture & Security Guarantees (`ArchitectureSecurity.svelte`)
- Two-column technical showcase aimed at power users and developers:
  - **Native Rust + WebView2 Engine:** Uses Windows' built-in Edge WebView2 instead of shipping a redundant 120 MB Chromium runtime.
  - **Hardened Local Execution:** Zero shell-string interpolation (eliminating command injection), strict path canonicalization guards, and OS native credential storage (`keyring`).

### 6. Dedicated Download & Quick-Start Hub (`DownloadHub.svelte`)
- **Card A — Windows Installer (`FERRO_x64-setup.exe`) [RECOMMENDED]:**
  - Guided NSIS installer, automatic WebView2 verification, Start Menu integration, and clean Windows uninstaller.
  - Direct download button + release version badge.
- **Card B — Portable Executable (`ferro-launcher.exe`):**
  - Single zero-install binary; drop onto any folder or USB drive and run immediately.
  - Direct download button + copyable PowerShell SHA-256 verification command (`Get-FileHash .\ferro-launcher.exe -Algorithm SHA256`).
- **Build From Source Accordion (For Developers):**
  - Copyable terminal block showing `git clone https://github.com/Miles-coder2000/FERRO.git`, `npm install`, and `npx tauri build --no-bundle`.
- **System Requirements Strip:** Windows 10 (version 1809+) or Windows 11 (x64), Microsoft Edge WebView2 Runtime, 50 MB disk space.

### 7. Footer (`Footer.svelte`)
- FERRO emblem + short tagline.
- Links: **GitHub Repository**, **Latest Releases**, **Report a Bug (Issues)**, and **Source License**.

---

## SEO, Accessibility & Verification Rules
1. **Meta & Open Graph Tags (`landing/index.html`):**
   - Title: `FERRO Launcher — Lightweight Rust & Tauri 2.0 Desktop Organizer`
   - Description: `Unify your games, apps, system folders, and CLI commands into one blazing-fast, sub-50MB Windows launcher built with Rust and Tauri 2.0.`
   - Configure `og:title`, `og:description`, `og:image`, and `twitter:card` (`summary_large_image`).
2. **Accessibility & Responsiveness:**
   - Test across `1440px` desktop, `768px` tablet, and `375px` mobile viewports with zero horizontal overflow.
   - Ensure all interactive buttons, tabs, and download links have visible copper focus rings (`focus-visible:outline-2 focus-visible:outline-[#F48C06]`) and `aria-label` attributes.
3. **Build Verification:**
   - Run `npm install` and `npm run build` inside `D:\FERRO\landing\` and verify zero TypeScript/Svelte errors and a clean `landing/dist/` production output.