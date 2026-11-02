# Tod — Windows Beta

Private beta build guide for the first Windows tester.

---

## GitHub repository configuration

**No secrets required.** The workflow uses only `GITHUB_TOKEN` (automatically provided by Actions) to upload build artifacts. No code-signing, no auto-update key, no external services.

The repository must have Actions enabled (the default for public and private repos).

---

## Triggering the Windows build

Two ways:

**Automatic** — any push to `main` that touches `src/`, `src-tauri/`, `package.json`, or `pnpm-lock.yaml` triggers the workflow.

**Manual** — go to the repository on GitHub → Actions → "Windows Build" → "Run workflow" → select `main` → "Run workflow". Use this to build without making a code change.

---

## Finding the installer

1. Go to the repository on GitHub → Actions.
2. Click the "Windows Build" run you want.
3. Scroll to the bottom of the run page — the **Artifacts** section lists `tod-windows-installer-<sha>`.
4. Download and unzip it. Inside is `Tod_0.1.0_x64-setup.exe` (the NSIS installer).

Artifacts are kept for 30 days.

---

## Installing and testing on Windows

### Installation

1. Download and unzip the artifact.
2. Double-click `Tod_0.1.0_x64-setup.exe`.
3. If Windows Defender SmartScreen shows "Windows protected your PC", click **More info → Run anyway**. This is expected for unsigned apps.
4. The installer runs without requiring administrator rights (installs per-user into `%LOCALAPPDATA%\Programs\Tod\`).
5. A Start Menu shortcut is created automatically.

### First launch and database creation

1. Launch Tod from the Start Menu or the installation folder.
2. On first launch, Tauri creates the app data directory and SQLite database at:
   ```
   C:\Users\<you>\AppData\Roaming\com.lukoOG.tod\planner.db
   ```
3. The database schema is applied automatically (migrations run in the Rust setup function before the window opens).
4. Verify the app loads today's view and that activities from the seed routines appear.

### Testing core functionality

- Navigate between days with the arrow buttons and arrow keys.
- Add a task, tick it off, add a note.
- Write a reflection and wait 1–2 seconds — "Saved" should appear without any manual action.
- Close and reopen the app. Confirm the task, note, and reflection are still there.

### Verifying application restart persistence

After adding data:
1. Quit Tod (close the window).
2. Reopen it from the Start Menu.
3. Navigate to the same day — data must be intact.

This confirms SQLite is writing to the persistent `AppData\Roaming` path rather than a temporary location.

---

## WebView2 runtime

Tod's UI runs inside Microsoft Edge WebView2. Windows 11 ships with WebView2 pre-installed. On Windows 10 it is usually already present (it ships with recent Windows updates and Office 365), but if it is missing, the NSIS installer automatically downloads and silently installs the official Microsoft bootstrapper at install time (`downloadBootstrapper` mode). The user sees no extra prompt.

---

## Known compatibility notes

| Area | Status | Notes |
|---|---|---|
| Database path | ✅ | Uses Tauri `data_dir()` — fully cross-platform |
| SQLite | ✅ | Bundled statically; no system SQLite needed |
| WebView2 CSS (`color-mix`, `backdrop-filter`, `-webkit-app-region`) | ✅ | All supported in Chromium-based WebView2 |
| Window controls spacer | ℹ️ | A 4.25 rem left spacer in the toolbar was added for macOS traffic lights. With native window decorations on Windows the spacer is invisible dead space inside the left grid column — it does not affect the centered date navigation. Can be made platform-conditional in a future pass. |
| Font stack | ℹ️ | `-apple-system` / `SF Pro Text` fall back to `system-ui` (Segoe UI) on Windows. Looks clean; not identical to macOS. |
| App identifier change | ⚠️ | Renamed from `com.mac.tod` to `com.lukoOG.tod`. On macOS, existing dev data is at `~/Library/Application Support/com.mac.tod/` — copy `planner.db` to `~/Library/Application Support/com.lukoOG.tod/` to keep it. |
| Code signing | ℹ️ | Not configured. SmartScreen will warn on first run. Acceptable for a private beta. |

---

## Preserving the macOS dev workflow

Nothing in these changes affects local development on macOS:

```sh
pnpm tauri dev   # still works, unchanged
cargo test       # still passes
pnpm check       # still 0 errors / 0 warnings
```

The `--bundles nsis` flag in the workflow is CI-only; local `pnpm tauri build` still produces the full macOS bundle.
