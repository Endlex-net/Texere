## 2026-02-27
- LSP diagnostics initially failed because rust-analyzer initialization timed out; resolved by ensuring the `rust-analyzer` rustup component is installed.
- First AX implementation attempt used pointer constants directly; compile errors indicated `accessibility-sys` expects `CFStringRef`, so conversions were added.
- Settings window was present in macOS window list but rendered black/empty because opacity was forced via Cocoa (`setOpaque_`) while transparency state was not set through Tauri window builder; this can desync NSWindow/WKWebView composition.
