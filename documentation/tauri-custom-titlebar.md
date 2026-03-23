# Tauri v2 — Custom Titlebar (Drag + Double-Click Maximize)

## Problem

With `decorations: false` in `tauri.conf.json`, the native titlebar is removed. You need to implement:
- **Drag to move** the window
- **Double-click to maximize/restore** (toggle)
- Buttons (close, minimize, maximize, settings) must NOT trigger drag/maximize

## What Does NOT Work

### `data-tauri-drag-region`
- Only handles drag, does NOT handle double-click-to-maximize
- Double-click-to-restore (from maximized state) is buggy/broken
- The attribute only applies to the exact element, not children

### Timeout-based approach
- Delaying `startDragging()` with `setTimeout` to let double-click fire first
- Makes dragging feel laggy and unresponsive — terrible UX

## Correct Solution

One `mousedown` handler that checks `e.detail`:
- `e.detail === 1` → single click → `startDragging()`
- `e.detail === 2` → double click → `toggleMaximize()`

The browser tracks click count in `e.detail` natively, so both work from the same event with zero delay.

### Implementation

**tauri.conf.json:**
```json
{
  "windows": [{
    "decorations": false,
    "transparent": true
  }]
}
```

**App.svelte:**
```typescript
import { getCurrentWindow } from "@tauri-apps/api/window";

// Bridge functions (thin wrappers around getCurrentWindow())
async function windowStartDrag() { await getCurrentWindow().startDragging(); }
async function windowToggleMaximize() { await getCurrentWindow().toggleMaximize(); }
async function windowMinimize() { await getCurrentWindow().minimize(); }
async function windowClose() { await getCurrentWindow().close(); }

function handleTitlebarMouseDown(e: MouseEvent) {
  if (e.button !== 0) return; // left-click only

  // Don't drag/maximize when clicking interactive elements
  const target = e.target as HTMLElement;
  if (target.closest("button, input, a, [role='button']")) return;

  if (e.detail === 2) {
    windowToggleMaximize(); // double-click → maximize/restore
  } else {
    windowStartDrag();      // single-click → drag window
  }
}
```

**HTML:**
```html
<div class="titlebar" onmousedown={handleTitlebarMouseDown}>
  <!-- your titlebar content -->
  <button onclick={windowMinimize}>—</button>
  <button onclick={windowToggleMaximize}>□</button>
  <button onclick={windowClose}>✕</button>
</div>
```

## Key Details

- `e.detail` is a native browser property — counts rapid clicks (1, 2, 3...)
- `startDragging()` hands control to the OS — native drag feel, no JS overhead
- `toggleMaximize()` handles both maximize AND restore automatically
- The `target.closest()` check prevents buttons from triggering drag/maximize
- No timeouts, no polling, no `data-tauri-drag-region` needed

## Required Permissions

In `src-tauri/capabilities/default.json`:
```json
{
  "permissions": [
    "core:window:allow-start-dragging",
    "core:window:allow-minimize",
    "core:window:allow-toggle-maximize",
    "core:window:allow-close"
  ]
}
```
