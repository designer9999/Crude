# Croc Transfer

Encrypted peer-to-peer file & text transfer dashboard. Built with Svelte 5, pywebview, Tailwind v4, and M3 Expressive dark theme.

Save contacts with pre-agreed code phrases for one-click transfers — like AnyDesk, but for croc.

## Features

- Contact system with saved code phrases
- File & folder transfer with drag-and-drop
- Inline text messaging between peers
- Auto-receive mode (always listening)
- LAN relay support for fast local transfers
- Activity log with clickable received items
- Self-update from GitHub releases
- Single-exe distribution via PyInstaller

## Quick Start

```
Croc-Transfer.bat          # Production (uses dist/)
dev.bat                    # Development (Vite hot reload)
```

## Project Structure

```
Crude/
  main.py                  # pywebview bootstrap
  backend/
    croc_api.py            # Python API exposed via window.pywebview.api
    updater.py             # GitHub release checker + downloader
    version.py             # APP_VERSION + GITHUB_REPO
  frontend/
    src/
      App.svelte           # Root layout (contact bar, FAB, status bar)
      app.css              # Tailwind v4 + M3 tokens + motion springs
      lib/
        api/bridge.ts      # Typed pywebview.api wrappers
        state/app-state.svelte.ts  # Global state + contacts + persistence
        theme/             # M3 Expressive color system
        ui/                # M3 components (Button, Card, Dialog, Switch, etc.)
      features/
        contacts/          # ContactBar, ContactChip, ContactDialog, ContactAvatar
        transfer/          # TransferPage, UnifiedSendArea, QuickReceive, ActivityLog
        send/              # DropZone, FileList, CodeDisplay (reused by transfer)
        settings/          # SettingsPage (global defaults + about)
        LogPanel.svelte    # Raw croc output log
```

## Requirements

- Node.js 20+ (`npm install`)
- Python 3.13 + `pywebview` (`pip install pywebview`)
- croc (`winget install schollz.croc`)

## Development

```bash
npm install               # Install frontend dependencies
dev.bat                   # Starts Vite + pywebview in dev mode
```

## Build

```bash
npx vite build            # Builds to dist/
Croc-Transfer.bat         # Runs from dist/
```

## Build Exe

```bash
npx vite build
python -m PyInstaller --noconfirm --onefile --windowed --name CrocTransfer --icon=imageres.dll_14_147.ico --add-data dist;dist --add-data backend;backend main.py
```

Output: `dist/CrocTransfer.exe`
