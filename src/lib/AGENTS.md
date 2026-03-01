# LIB KNOWLEDGE BASE

## OVERVIEW
Frontend core logic and UI components. Manages the CodeMirror 6 instance, AI formatting interface, and note templates.

## STRUCTURE
- `components/`: UI elements (Editor, WindowChrome, StatusBar)
- `editor/`: CodeMirror 6 configuration and extension logic
- `ai.ts`: Frontend API for AI text processing
- `settings.ts`: Local settings management and sync with Rust
- `templates.ts`: CRUD operations for editor templates
- `types.ts`: TypeScript definitions (sync with `src-tauri/src/types.rs`)

## WHERE TO LOOK
| Task | Target |
| :--- | :--- |
| Adjust Editor behavior | `editor/createEditor.ts` |
| Change layout / theme | `components/WindowContainer.svelte` |
| Modify Vim bindings | `editor/createEditor.ts` |
| Edit Status Bar info | `components/StatusBar.svelte` |
| Update Template logic | `templates.ts` |
| Handle clipboard ops | `clipboard.ts` |

## CONVENTIONS
- **Props:** Use `export let` for component inputs.
- **Events:** Use `createEventDispatcher` for bubbling actions to parents.
- **Styles:** Use `:global()` in Svelte blocks only when targeting CodeMirror classes.
- **Types:** Import shared types from `types.ts`. Avoid inline interfaces for core data.

## EDITOR PATTERNS
- **Instance Management:** `createEditor.ts` returns an `EditorView`. `Editor.svelte` manages its lifecycle via `onMount` and `onDestroy`.
- **Imperative Bridge:** `Editor.svelte` exports `getValue()` and `setValue()` methods. Parents use `bind:this` to access these.
- **Dynamic Config:** Use CM6 `Compartment` for settings that change at runtime (Vim mode, line wrapping).

## COMPONENT PATTERNS
- **Slots:** `WindowContainer` uses slots for flexible layout. Check `StatusBar` placement in the `statusbar` named slot.
- **Prop Callbacks:** Use function props (e.g., `getEditorContent`) to share state between siblings like `WindowChrome` and `Editor`.
- **Global Styles:** App-wide variables are defined in the root, but component-specific layout lives in scoped `<style>` tags.
