## 2026-02-27
- Implemented cursor spawn via macOS Accessibility AX chain (`SystemWide -> FocusedApplication -> FocusedUIElement -> SelectedTextRange -> BoundsForRange`) with `Option` return to guarantee graceful degradation.
- Applied spawn offsets exactly as specified (`x + 16`, `y + 8`) and clamped against current monitor logical bounds to prevent off-screen placement.
- Kept fallback behavior implicit by only setting position when cursor coordinates are available; existing default placement remains unchanged.
- Switched settings window to explicit non-transparent mode at creation (`WebviewWindowBuilder::transparent(false)`) instead of post-hoc Cocoa opacity mutation; keeps Tauri/Wry/WebKit compositing path consistent for secondary windows.
