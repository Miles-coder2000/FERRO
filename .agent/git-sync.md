# FERRO Continuous Git Sync & Release Hygiene Protocol

## Repository Target
- **Remote Name:** `origin`
- **Repository URL:** `https://github.com/Miles-coder2000/FERRO.git`
- **Primary Branch:** `main`
- **MCP Integration:** Use the `git` MCP (`@modelcontextprotocol/server-git`) for status, diff, staging, branching, and commits, and run `git fetch` / `git pull --rebase` / `git push` via terminal execution when syncing with GitHub.

---

## Phase 1: Repository Initialization & Remote Verification
Before staging or pushing for the first time in a session, verify local Git state:
1. Check whether `.git/` exists in the project root:
   - If missing, run `git init -b main`.
2. Verify the remote URL via `git remote -v`:
   - If `origin` does not exist, run:
     `git remote add origin https://github.com/Miles-coder2000/FERRO.git`
   - If `origin` points to a different URL, update it:
     `git remote set-url origin https://github.com/Miles-coder2000/FERRO.git`
3. Ensure the current branch is `main` (`git branch -M main`).

---

## Phase 2: Pre-Flight `.gitignore` & Artifact Guardrails
Before running `git add`, inspect `.gitignore` and untracked files (`git status --porcelain`) to prevent bloating the repository with build artifacts or leaking local data:
1. Confirm `.gitignore` explicitly blocks:
   - `node_modules/`
   - `dist/`
   - `src-tauri/target/`
   - `src-tauri/WixTools/`
   - `src-tauri/dev-data/*.db*`
   - `.stress_sandbox/`
   - `.env` and `.env.*`
   - `*.exe`, `*.msi`, `*.nsis`, `*.log`
2. If any of these patterns are missing from `.gitignore`, append them **before** staging files.
3. Never stage large binary installers or local SQLite database files; distribute compiled `.exe` binaries via GitHub Releases rather than raw Git commits.

---

## Phase 3: Pre-Commit Build & Sanity Verification
Whenever changes touch source code (`src-tauri/`, `src/`, `Cargo.toml`, `package.json`, `tauri.conf.json`), verify the build passes before committing:
1. Run `cargo clippy --all-targets -- -D warnings` (via `cargo` MCP) if any `.rs` or `Cargo.toml` file changed.
2. Run `npm run build` (or `npx svelte-check`) if any `.svelte`, `.ts`, `.css`, or `package.json` file changed.
3. If compilation or type-checking fails, fix the errors first—**never push a broken build to `main`**.

---

## Phase 4: Atomic Add → Commit → Sync → Push Pipeline
Execute these five steps in strict order:
1. **Inspect Changes:** Run `git status --short` and `git diff --stat` to review all modified, added, and deleted files.
   - If the working tree is clean, skip to Step 4 to ensure local `main` is synced with `origin/main`.
2. **Stage Changes:** Stage all verified project files (`git add -A`). Re-verify `git status --short` to confirm zero ignored build folders or secrets were picked up.
3. **Commit with Conventional Commits:** Create a concise, descriptive commit summarizing *what* changed and *why*:
   - Format: `<type>(<scope>): <imperative summary>`
   - Allowed types: `feat`, `fix`, `refactor`, `perf`, `sec`, `docs`, `chore`, `build`, `test`
   - Examples:
     - `feat(armory): implement async game scanner with icon caching`
     - `fix(ipc): strip Windows extended path prefix before process spawn`
     - `chore(agent): add codebase audit and continuous git sync rules`
4. **Sync with Remote (`pull --rebase`):** Fetch upstream changes and rebase local commits cleanly on top of `origin/main` to maintain a linear history without unnecessary merge commits:
   - Run `git fetch origin`
   - If `origin/main` exists, run `git pull --rebase origin main`
   - *Conflict Protocol:* If a rebase conflict occurs, pause, inspect the conflicting hunks, resolve them cleanly without dropping local features, re-run Phase 3 build verification, and execute `git rebase --continue`.
5. **Push to GitHub:**
   - Run `git push -u origin main`
   - Verify synchronization with `git status -sb` (must report `## main...origin/main` with zero ahead/behind commits).

---

## Phase 5: Continuous Post-Build Loop ("And So Forth")
After **every** subsequent action in the session—including `npm run tauri build`, audit patches, stress-test fixes, dependency updates, or `.agent/` rule edits—automatically run this post-build sync loop:
1. **Check Build Delta:** Run `git status --porcelain`.
2. **Evaluate Changes:**
   - **If output is empty:** Report `"Working tree clean — FERRO is fully synced with origin/main."` and exit the loop.
   - **If new or modified files exist** (e.g., updated `Cargo.lock`, `package-lock.json`, auto-formatted code, or build fixes):
     - Re-verify `.gitignore` compliance (Phase 2).
     - Re-run build check if source files changed (Phase 3).
     - Execute a new **Add → Commit → Rebase-Sync → Push** cycle (Phase 4) with an incremental commit message (e.g., `build(release): update lockfile and tauri bundle config`).
3. **Repeat:** Re-check `git status --porcelain` after pushing and repeat Step 2 until the working tree is completely clean and `origin/main` reflects the latest verified build.

---

## Safety Guardrails
- **No Force Push:** Never run `git push --force` on `main`.
- **Credential Protection:** Never echo, log, or commit GitHub Personal Access Tokens (PATs) or SSH private keys; rely on the OS Git Credential Manager (`gh auth` / Windows Credential Manager).