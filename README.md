# Croc Transfer

Encrypted peer-to-peer file transfer. Svelte 5 + pywebview + M3 Expressive dark theme.

## Quick Start

```
Croc-Transfer.bat          # Production (uses dist/)
dev.bat                    # Development (Vite hot reload)
```

## Project Structure

```
Crude/
  main.py                  # pywebview bootstrap (--dev flag for dev mode)
  backend/
    __init__.py
    croc_api.py            # Python API exposed to JS via window.pywebview.api
  frontend/
    index.html             # Entry point (zero-flicker theme restore)
    src/
      main.ts              # Svelte mount + pywebview bridge wait
      app.css              # Tailwind v4 + M3 tokens + motion springs
      App.svelte           # Root component (top bar, tabs, status bar)
      lib/
        api/
          bridge.ts        # Typed pywebview.api wrappers + dev mock fallbacks
        theme/
          m3-color.ts      # HCT seed color → 26+ M3 color roles
          m3-tokens.ts     # Color tokens → CSS custom properties
          apply-theme.ts   # Inject tokens to DOM + localStorage cache
          theme-store.svelte.ts  # Reactive theme state ($derived tokens)
        state/
          app-state.svelte.ts    # Global app state (transfer, logs, files)
        ui/
          Button.svelte    # M3 Button (filled, tonal, outlined, error)
          Card.svelte      # M3 Card (elevated, filled, outlined)
          Icon.svelte      # Material Symbols Rounded wrapper
          ProgressBar.svelte  # M3 Linear indeterminate
          Snackbar.svelte  # M3 Snackbar (inverseSurface, auto-dismiss)
      features/
        send/
          SendPage.svelte  # Send tab layout
          DropZone.svelte  # Drag & drop + file picker
          CodeDisplay.svelte  # Transfer code with copy button
        receive/
          ReceivePage.svelte  # Receive tab layout
        guide/
          GuidePage.svelte # How-to guide + colleague setup instructions
        LogPanel.svelte    # Transfer log output
  dist/                    # Built output (vite build)
  package.json             # Svelte 5, Tailwind v4, M3 color utils
  vite.config.ts           # Vite + Svelte + Tailwind + $lib alias
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

## Colleague Setup

They need croc: `winget install schollz.croc` then restart terminal.
See the **Guide** tab in the app for full setup instructions.
