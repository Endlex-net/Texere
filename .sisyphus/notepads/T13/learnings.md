## 2026-02-27
- For `accessibility-sys` 0.2, AX attribute constants are `&str`; convert to `CFString` before passing to `AXUIElementCopyAttributeValue`/`AXUIElementCopyParameterizedAttributeValue`.
- Use `kAXValueTypeCGRect` with `AXValueGetValue` for caret bounds extraction.
- Keep cursor lookup fully fallible (`Option`) so denied Accessibility permissions cleanly fall back to existing center behavior.
- Settings secondary windows should set transparency explicitly (`.transparent(false)`) when the app globally enables `macOSPrivateApi` and uses transparent primary/editor windows.
- Vite multi-entry config is valid here: `dist/settings.html` is generated and references compiled `settings-*` JS/CSS bundles, so an invisible settings window can still be a native compositing issue rather than missing assets.
