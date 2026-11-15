# FERRO Project Directory Architecture & Organization Protocol

## Objective
Maintain a clean, modular, enterprise-grade repository structure across `D:\FERRO` while actively developing and building the application. Whenever new features are added—or when existing folders become cluttered—follow this blueprint to organize files without breaking Rust module trees, Svelte 5 imports, Tauri v2 configuration paths, or Git history.

---

## 1. Canonical Repository Structure

Every file in `D:\FERRO` must live in its designated architectural layer. Never leave loose scripts, stray binaries, unreferenced images, or temporary test databases in the repository root.

```text
D:\FERRO\
├── .agent/                          # AI Agent Configuration, Guardrails & Playbooks
│   ├── rules                        # Core stack constraints (Tauri v2, Svelte 5 Runes, Rust)
│   ├── skills                       # Domain knowledge & OS-specific capabilities
│   ├── guidelines                   # End-to-end feature SOP & UI layering hierarchy
│   ├── security                     # AppSec rules & MCP runtime boundaries
│   └── instructions/                # Detailed operational & maintenance workflows
│       ├── project-structure.md     # This canonical architecture & refactoring guide
│       ├── build-and-run.md         # Windows MSVC & Tauri CLI build steps
│       ├── audit.md                 # Full-stack static analysis & dead-code audit
│       ├── stress-test.md           # 100k-file chaos & memory profiling suite
│       ├── git-sync.md              # Automated GitHub add/commit/rebase/push loop
│       └── landing-page.md          # Product landing page & download portal spec
│
├── docs/                            # Public Project Documentation & Design Specs
│   ├── ARCHITECTURE.md              # IPC boundary, SQLite FTS5 & process spawn diagrams
│   ├── KEYBINDINGS.md               # Global shortcuts & CLI prefix reference (`>`, `/`, `@`)
│   └── CHANGELOG.md                 # Versioned release notes
│
├── scripts/                         # Developer Automation & Packaging Helpers
│   ├── clean-artifacts.ps1          # Purges temporary test sandboxes & stale caches
│   └── verify-release.ps1           # Generates SHA-256 hashes for release `.exe` builds
│
├── landing/                         # Static Product Landing Page & Download Portal
│   ├── public/                      # Static branding assets (`ferro-logo.png`, `og-preview.png`)
│   ├── src/
│   │   ├── lib/
│   │   │   ├── components/          # Modular landing sections (`HeroForgeDemo.svelte`, etc.)
│   │   │   └── github.ts            # GitHub Releases API client with local caching
│   │   ├── App.svelte
│   │   └── main.ts
│   ├── index.html
│   └── package.json
│
├── src/                             # Svelte 5 + TypeScript Frontend (The HUD)
│   ├── assets/                      # Bundled UI textures, SVG frames & Obsidian Industrial art
│   │   ├── icons/                   # Custom beveled SVG frames & sector emblems
│   │   └── textures/                # Optimized background canvases & metallic overlays
│   ├── lib/
│   │   ├── components/
│   │   │   ├── layout/              # Frameless window chrome (`TopBar.svelte`, `Sidebar.svelte`)
│   │   │   ├── sectors/             # Sector views (`ForgeHome.svelte`, `ArmoryGrid.svelte`, `StackDock.svelte`, `FolderMatrix.svelte`, `ContextsView.svelte`)
│   │   │   └── ui/                  # Reusable Obsidian Industrial primitives (`GlassCard.svelte`, `CopperButton.svelte`, `TelemetryBadge.svelte`, `CommandPalette.svelte`)
│   │   ├── services/                # Typed IPC bridges (`ipc.ts`) & global keyboard listeners (`keyboard.ts`)
│   │   ├── state/                   # Svelte 5 `.svelte.ts` rune stores (`launcherState.svelte.ts`, `telemetryState.svelte.ts`)
│   │   ├── styles/                  # Design tokens (`tokens.css`) & Tailwind v4 entry (`app.css`)
│   │   └── types/                   # TypeScript mirrors of Rust IPC structs (`models.ts`)
│   ├── App.svelte                   # Root HUD shell & keyboard focus orchestrator
│   └── main.ts                      # Svelte mount entry point
│
├── src-tauri/                       # Rust Native Backend & Tauri v2 Runtime
│   ├── capabilities/                # Principle-of-least-privilege Tauri v2 ACL manifests
│   │   └── default.json
│   ├── icons/                       # App bundle icons (`icon.ico`, `icon.icns`, `32x32.png`, `128x128.png`)
│   ├── src/
│   │   ├── commands/                # Thin `#[tauri::command]` IPC handlers (input validation + async delegation)
│   │   │   ├── mod.rs
│   │   │   ├── indexer_cmd.rs       # App, game & folder scan triggers
│   │   │   ├── launcher_cmd.rs      # Detached process spawn & elevation commands
│   │   │   ├── search_cmd.rs        # SQLite FTS5 query & CLI prefix dispatch
│   │   │   └── telemetry_cmd.rs     # Live CPU, RAM & latency snapshot endpoints
│   │   ├── core/                    # Domain engines & OS integrations
│   │   │   ├── mod.rs
│   │   │   ├── app_scanner.rs       # Start Menu / `.lnk` / `.desktop` parser
│   │   │   ├── game_scanner.rs      # Steam `.vdf`/`.acf` & Epic `.item` manifest parser
│   │   │   ├── icon_cache.rs        # Native icon extraction & `.webp`/`.png` thumbnail writer
│   │   │   ├── process_runner.rs    # Detached OS process spawner (`CREATE_NEW_PROCESS_GROUP`)
│   │   │   ├── telemetry.rs         # `sysinfo` polling & microsecond dispatch timer
│   │   │   └── watcher.rs           # Debounced `notify` filesystem event monitor
│   │   ├── db/                      # Embedded SQLite persistence layer
│   │   │   ├── mod.rs
│   │   │   ├── migrations.rs        # Schema initialization & FTS5 virtual table setup
│   │   │   └── repository.rs        # Parameterized CRUD & ranked prefix search queries
│   │   ├── models/                  # Shared `Serialize`/`Deserialize` IPC contracts
│   │   │   ├── mod.rs
│   │   │   └── entities.rs          # `AppEntry`, `GameEntry`, `FolderCard`, `TelemetrySnapshot`
│   │   ├── utils/                   # Cross-platform helpers
│   │   │   ├── mod.rs
│   │   │   └── path_guard.rs        # Path canonicalization, symlink loop & `\\?\` prefix stripping
│   │   ├── lib.rs                   # Tauri Builder setup, plugin registration & state management
│   │   └── main.rs                  # Minimal Windows subsystem entry point (calls `ferro_lib::run()`)
│   ├── tests/                       # Integration & stress-test suites (`stress_suite.rs`)
│   ├── build.rs                     # Tauri build script
│   ├── Cargo.toml                   # Pinned Rust dependencies
│   └── tauri.conf.json              # Window geometry, CSP, bundle & assetProtocol settings
│
├── .gitignore                       # Blocks `target/`, `dist/`, `node_modules/`, `*.db*`, `.stress_sandbox/`
├── index.html                       # Frontend HTML shell with Google Fonts (`Chakra Petch`, `Inter`, `JetBrains Mono`)
├── package.json                     # Frontend scripts & dependencies
├── svelte.config.js                 # Svelte compiler configuration
├── tsconfig.json                    # Strict TypeScript config with `$lib` path aliases
└── vite.config.ts                   # Vite bundler config (port 1420, `clearScreen: false`)
```

---

## 2. File Naming & Modularity Conventions

Enforce consistent naming across every directory so any developer or agent can locate files predictably:

| Layer | File Type | Naming Convention | Example |
| :--- | :--- | :--- | :--- |
| **Rust (`src-tauri/src/`)** | Modules & Source Files | `snake_case.rs` | `game_scanner.rs`, `path_guard.rs` |
| **Svelte (`src/lib/components/`)** | UI & Sector Components | `PascalCase.svelte` | `ArmoryGrid.svelte`, `GlassCard.svelte` |
| **Svelte 5 Runes (`src/lib/state/`)** | Reactive State Modules | `camelCase.svelte.ts` | `launcherState.svelte.ts` |
| **TypeScript (`src/lib/`)** | Services, Types & Helpers | `camelCase.ts` | `ipc.ts`, `models.ts` |
| **Docs & Agent Guides** | Markdown Specifications | `kebab-case.md` | `project-structure.md`, `build-and-run.md` |

### Size & Responsibility Thresholds
- **Keep Entrypoints Thin:** `src-tauri/src/main.rs` must remain under 15 lines. `src-tauri/src/lib.rs` must only wire plugins, managed state, and `tauri::generate_handler![...]`.
- **Single-Responsibility Modules:** If any `.rs` or `.svelte` file exceeds **300 lines**, split sub-components or helper traits into their own dedicated module within the appropriate subfolder (`ui/`, `core/`, `utils/`).
- **Zero Direct `invoke()` in Views:** Components inside `src/lib/components/` must never import `@tauri-apps/api/core` directly; route all backend communication through typed functions in `src/lib/services/ipc.ts`.

---

## 3. Step-by-Step Re-Organization Protocol (While Building)

Whenever reorganizing existing files or migrating misplaced code into the canonical layout above, execute strictly in this 5-step order so the build never breaks:

### Step 1: Inventory & Classify Stray Files
1. Scan the repository tree (`src/`, `src-tauri/src/`, `.agent/`, and root `D:\FERRO\`).
2. Identify:
   - Root-level clutter (loose test scripts, misplaced assets, backup `.rs`/`.svelte` files).
   - Monolithic files combining UI rendering, IPC calls, and state management in one component.
   - Flat `src-tauri/src/` files that belong inside `commands/`, `core/`, `db/`, `models/`, or `utils/`.
   - Any operational markdown guides at `.agent/*.md` (other than core `rules`, `skills`, `guidelines`, `security`) that should be consolidated cleanly under `.agent/instructions/`.

### Step 2: Move Files Using `git mv` (Preserve History)
1. Create target subdirectories first (`mkdir -p` / `New-Item -ItemType Directory -Force`).
2. Always relocate tracked files via `git mv <old_path> <new_path>` rather than copy-deleting, preserving `git blame` and commit history.

### Step 3: Repair Module Trees & Import Aliases
1. **Rust Module Wiring (`src-tauri/src/`):**
   - Add `mod.rs` inside every new subfolder (`commands/mod.rs`, `core/mod.rs`, `db/mod.rs`, `models/mod.rs`, `utils/mod.rs`) with explicit `pub mod ...;` exports.
   - Update `use crate::...` paths and `tauri::generate_handler![commands::indexer_cmd::..., ...]` in `src-tauri/src/lib.rs`.
2. **TypeScript & Svelte Imports (`src/`):**
   - Configure `$lib` alias in `tsconfig.json` and `vite.config.ts` (`"$lib": ["./src/lib"]`) so component moves never require fragile `../../../` relative chains.
   - Update all component, store, and asset imports (`$lib/components/...`, `$lib/state/...`, `$lib/services/ipc`).
3. **Tauri Configuration (`src-tauri/tauri.conf.json`):**
   - Verify `build.frontendDist` points to `"../dist"` and `bundle.icon` points to `"icons/..."`.

### Step 4: Purge Empty Directories & Temporary Artifacts
1. Remove leftover empty directories after moving files.
2. Confirm temporary directories (`.stress_sandbox/`, `src-tauri/dev-data/*.db*`, `dist/`, `src-tauri/target/`) are excluded in `.gitignore` and never tracked by Git.

### Step 5: Mandatory Compilation & Sync Gate
Before marking any structural reorganization complete, run the full verification suite:
1. **Frontend Check:** `cd D:\FERRO && npx svelte-check --tsconfig ./tsconfig.json && npx vite build`
2. **Rust Check:** `cd D:\FERRO\src-tauri && cargo check && cargo clippy --all-targets -- -D warnings`
3. **Git Sync:** Commit the structural refactor atomically (`refactor(structure): organize codebase into canonical domain layers`) following `@.agent/instructions/git-sync.md`.