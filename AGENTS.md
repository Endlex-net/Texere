# PROJECT KNOWLEDGE BASE: Texere

**Generated:** 2026-02-27
**Stack:** Tauri v2 + Svelte 5 + TypeScript + Rust

## OVERVIEW

Texere is a **menu bar quick-notes app** for macOS. A transparent, always-on-top text editor with Vim keybindings, AI text formatting, and template management.

## STRUCTURE

```
texere/
├── src/                    # Svelte 5 frontend
│   ├── lib/
│   │   ├── components/     # Svelte UI components
│   │   ├── editor/         # CodeMirror 6 setup
│   │   ├── types.ts        # Shared TypeScript types
│   │   ├── settings.ts     # Settings management
│   │   ├── templates.ts    # Template CRUD
│   │   ├── ai.ts           # AI formatting (OpenAI)
│   │   └── clipboard.ts    # Clipboard operations
│   ├── App.svelte          # Main app component
│   └── main.ts             # Frontend entry
├── src-tauri/              # Rust backend
│   └── src/
│       ├── lib.rs          # Tauri app setup
│       ├── ai.rs           # OpenAI API integration
│       ├── window.rs       # Window management
│       ├── tray.rs         # System tray/menu bar
│       ├── clipboard.rs    # Clipboard + auto-paste
│       ├── settings.rs     # Settings persistence
│       ├── templates.rs    # Template storage
│       ├── cursor_position.rs # Cursor tracking
│       └── types.rs        # Rust types + tests
└── package.json            # Frontend deps + scripts
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add AI feature | `src-tauri/src/ai.rs` | OpenAI API calls |
| Modify editor | `src/lib/editor/createEditor.ts` | CodeMirror config |
| Change UI | `src/lib/components/*.svelte` | Svelte components |
| Add hotkey | `src-tauri/src/window.rs` | Global shortcuts |
| Update types | `src/lib/types.ts` + `src-tauri/src/types.rs` | Keep in sync |
| Settings UI | `src/lib/components/` + `src/lib/settings.ts` | |
| System tray | `src-tauri/src/tray.rs` | Menu bar integration |

## CODE MAP

### Frontend → Backend Communication

**Pattern:** `invoke()` from frontend, `#[tauri::command]` in Rust

```typescript
// Frontend: src/lib/ai.ts
import { invoke } from '@tauri-apps/api/core';
const result = await invoke('format_text', { apiKey, content, model });
```

```rust
// Backend: src-tauri/src/ai.rs
#[tauri::command]
pub async fn format_text(api_key: String, content: String, model: String) -> Result<...> { }
```

**Registered in:** `src-tauri/src/lib.rs` → `invoke_handler![]`

### Key Commands (Rust)

| Command | Module | Purpose |
|---------|--------|---------|
| `format_text` | ai.rs | AI text formatting via OpenAI |
| `copy_and_dismiss` | clipboard.rs | Copy text, paste to prev app, hide window |
| `dismiss_without_copy` | clipboard.rs | Hide window without copying |
| `show_pooled_window` | window.rs | Show/hide window with hotkey |
| `get_settings` / `set_settings` | settings.rs | Settings CRUD |
| `get_templates` / `save_template` / `delete_template` | templates.rs | Template CRUD |

## CONVENTIONS

### File Naming
- **Rust:** `snake_case.rs` (ai.rs, window.rs)
- **TypeScript:** `camelCase.ts` (ai.ts, settings.ts)
- **Svelte:** `PascalCase.svelte` (Editor.svelte, StatusBar.svelte)

### Type Safety
- **Dual types:** Types defined in BOTH `src/lib/types.ts` AND `src-tauri/src/types.rs`
- **Serialization:** Rust uses `#[serde(rename_all = "camelCase")]` to match TS
- **Tests:** Rust types have unit tests in `types.rs`

### Frontend Patterns
- **Props:** Explicitly typed with `export let`
- **Events:** Use `createEventDispatcher` for custom events
- **Component refs:** `bind:this` + exported methods (getValue, setValue)
- **Slots:** Used for composable layouts (WindowContainer slots)

### Rust Patterns
- **Error handling:** Return `Result<T, String>` for invoke commands
- **State management:** Use `app.manage()` for shared state (store, clipboard)
- **Async:** Commands are async when doing I/O (API calls, file ops)

## ANTI-PATTERNS (THIS PROJECT)

### NEVER
- **Remove the Windows console prevention code** in `src-tauri/src/main.rs`:
  ```rust
  // DO NOT REMOVE!!
  #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
  ```

### AVOID
- **Skipping type tests:** Add roundtrip tests for new types in `types.rs`
- **Breaking type parity:** Keep TS and Rust types synchronized

## UNIQUE STYLES

### Vim Mode Toggle
CodeMirror vim extension uses a Compartment for dynamic enable/disable:
```typescript
const vimCompartment = new Compartment();
// In createEditor: vimCompartment.of(vimEnabled ? vim() : [])
// To toggle: view.dispatch({ effects: vimCompartment.reconfigure(...) })
```

### Window Pooling
Windows are pooled/reused rather than recreated for performance:
```rust
// window.rs - shows existing window or creates new one
pub fn show_pooled_window(app: AppHandle) -> Result<(), String>
```

### Clipboard Auto-Paste
After copying, app can automatically paste to previous active window:
```rust
// Stores previous app handle, restores focus, simulates paste
clipboard::copy_and_dismiss(..., auto_paste: bool)
```

## COMMANDS

```bash
# Development
cd texere && bun run dev          # Vite dev server + Tauri dev

# Building
cd texere && bun run build        # Production build

# Type checking
cd texere && bun run check        # svelte-check + tsc

# Tauri CLI
cd texere/src-tauri && cargo build
cd texere/src-tauri && cargo test
```

## NOTES

- **macOS-only features:** Uses `macos-private-api` for window management
- **Menu bar app:** Hides from Dock (`ActivationPolicy::Accessory`)
- **Transparent window:** Custom chrome (no OS title bar)
- **Global shortcuts:** Registered at app startup via `tauri-plugin-global-shortcut`
- **Settings storage:** Uses `tauri-plugin-store` (JSON file)
- **Template storage:** Simple JSON file in app data directory

## EXTERNAL DOCS

- **Tauri v2:** https://v2.tauri.app/
- **Svelte 5:** https://svelte.dev/docs/svelte/overview
- **CodeMirror 6:** https://codemirror.net/docs/
- **Vim mode:** `@replit/codemirror-vim`
