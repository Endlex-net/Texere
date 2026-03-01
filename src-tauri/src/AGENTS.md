# BACKEND KNOWLEDGE BASE: src-tauri/src

## OVERVIEW
Rust backend for Texere. Manages the macOS menu bar lifecycle, high-performance window pooling, and AI-driven text transformations.

## STRUCTURE
- **lib.rs**: App entry point, plugin setup (Store, Shortcut, Tray), and command registration.
- **window.rs**: Window pooling logic (<50ms summon), positioning near cursor, and hotkey registration.
- **ai.rs**: OpenAI API client, system prompts, and heuristic style detection (Formal/Informal/Technical).
- **clipboard.rs**: Clipboard monitoring and auto-paste focus restoration logic.
- **tray.rs**: Native macOS menu bar (Tray) icon and menu definition.
- **settings.rs**: Type-safe wrapper around `tauri-plugin-store`.
- **templates.rs**: CRUD operations for user-defined snippet templates.
- **types.rs**: Canonical Rust definitions for shared types. Includes roundtrip serialization tests.
- **cursor_position.rs**: macOS-specific implementation for fetching screen coordinates.
- **notifications.rs**: Helper for system-level user alerts (e.g., accessibility permissions).

## WHERE TO LOOK
| Task | File |
|------|------|
| Update AI Prompt | `ai.rs` (SYSTEM_PROMPT) |
| Change Hotkeys | `window.rs` (register_summon_hotkey) |
| Modify Window Size | `window.rs` (create_editor_window) |
| Add Menu Bar Item | `tray.rs` |
| Add New Command | `lib.rs` (invoke_handler) + relevant module |

## CONVENTIONS
- **Error Propagation**: Return `Result<T, String>` for all `#[tauri::command]` functions.
- **State Access**: Use `tauri::State` for shared resources (WindowPool, PreviousApp, Store).
- **Async Commands**: Use `async fn` for commands performing I/O (AI calls, file ops).
- **Serialization**: Always use `#[serde(rename_all = "camelCase")]` to match TypeScript conventions.
- **Testing**: Every module should have a `mod tests` block for core logic and type safety.

## ANTI-PATTERNS
- **Console Leakage**: NEVER remove `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` in `main.rs`.
- **Direct App Logic in lib.rs**: Keep `lib.rs` for wiring; move business logic to dedicated modules.
- **Blocking the Main Thread**: Use `tauri::async_runtime::spawn` for long-running background tasks.

## KEY PATTERNS
- **Window Pooling**: Maintains a `WindowPool` (Arc<Mutex<Vec<WebviewWindow>>>) to bypass window creation lag.
- **Focus Restoration**: `record_frontmost_app` stores the active app before Texere summons, allowing auto-paste to return focus accurately.
- **Dual Type Tests**: `types.rs` contains tests that ensure Rust structs serialize to the exact JSON format expected by the frontend.
