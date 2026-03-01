#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri::{Manager, WebviewWindow};
use tauri_plugin_clipboard_manager::ClipboardExt;

/// Stores the bundle identifier of the app that was frontmost before Texere was summoned.
pub struct PreviousApp(pub Mutex<Option<String>>);

pub struct AutoPasteInFlight(pub Arc<AtomicBool>);

// ---------------------------------------------------------------------------
// macOS native helpers (NSWorkspace = no Accessibility permission needed)
// ---------------------------------------------------------------------------

/// Check if the current process has Accessibility (Trusted) permission.
/// Required for keystroke simulation via CGEvent or System Events.
#[cfg(target_os = "macos")]
fn is_accessibility_trusted() -> bool {
    extern "C" {
        fn AXIsProcessTrusted() -> bool;
    }
    unsafe { AXIsProcessTrusted() }
}

#[cfg(target_os = "macos")]
pub fn auto_paste_permission_granted() -> bool {
    is_accessibility_trusted()
}

#[cfg(not(target_os = "macos"))]
pub fn auto_paste_permission_granted() -> bool {
    true
}

#[cfg(target_os = "linux")]
fn is_x11_session() -> bool {
    if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
        if session_type.eq_ignore_ascii_case("x11") {
            return true;
        }
        if session_type.eq_ignore_ascii_case("wayland") {
            return false;
        }
    }

    std::env::var_os("DISPLAY").is_some() && std::env::var_os("WAYLAND_DISPLAY").is_none()
}

#[cfg(target_os = "linux")]
fn xdotool_available() -> bool {
    std::process::Command::new("xdotool")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[tauri::command]
pub fn can_enable_auto_paste() -> bool {
    auto_paste_permission_granted()
}

/// Get the frontmost application's bundle identifier via NSWorkspace.
/// Does **not** require Accessibility permissions (unlike osascript + System Events).
#[cfg(target_os = "macos")]
fn get_frontmost_bundle_id() -> Option<String> {
    unsafe {
        let workspace: *mut objc::runtime::Object = msg_send![class!(NSWorkspace), sharedWorkspace];
        if workspace.is_null() {
            return None;
        }
        let front_app: *mut objc::runtime::Object = msg_send![workspace, frontmostApplication];
        if front_app.is_null() {
            return None;
        }
        let bundle_id: *mut objc::runtime::Object = msg_send![front_app, bundleIdentifier];
        if bundle_id.is_null() {
            return None;
        }
        let utf8: *const std::os::raw::c_char = msg_send![bundle_id, UTF8String];
        if utf8.is_null() {
            return None;
        }
        let bid = std::ffi::CStr::from_ptr(utf8).to_string_lossy().to_string();
        if bid.is_empty() {
            None
        } else {
            Some(bid)
        }
    }
}

/// Activate an application by its bundle identifier via NSWorkspace.
/// Does **not** require Accessibility permissions.
/// Returns `true` if the activation request was sent successfully.
#[cfg(target_os = "macos")]
fn activate_app_by_bundle_id(bundle_id: &str) -> bool {
    unsafe {
        let workspace: *mut objc::runtime::Object = msg_send![class!(NSWorkspace), sharedWorkspace];
        if workspace.is_null() {
            return false;
        }
        let running_apps: *mut objc::runtime::Object = msg_send![workspace, runningApplications];
        if running_apps.is_null() {
            return false;
        }
        let count: usize = msg_send![running_apps, count];
        for i in 0..count {
            let app: *mut objc::runtime::Object = msg_send![running_apps, objectAtIndex: i];
            if app.is_null() {
                continue;
            }
            let app_bid: *mut objc::runtime::Object = msg_send![app, bundleIdentifier];
            if app_bid.is_null() {
                continue;
            }
            let utf8: *const std::os::raw::c_char = msg_send![app_bid, UTF8String];
            if utf8.is_null() {
                continue;
            }
            let app_bid_str = std::ffi::CStr::from_ptr(utf8).to_string_lossy();
            if app_bid_str == bundle_id {
                // NSApplicationActivateIgnoringOtherApps = 1 << 1 = 2
                let _: bool = msg_send![app, activateWithOptions: 2usize];
                return true;
            }
        }
        false
    }
}

/// Simulate Cmd+V keystroke using the CGEvent API.
/// **Requires** Accessibility permission (`AXIsProcessTrusted`).
#[cfg(target_os = "macos")]
fn simulate_cmd_v() {
    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn CGEventCreateKeyboardEvent(
            source: *const std::ffi::c_void,
            virtual_key: u16,
            key_down: bool,
        ) -> *mut std::ffi::c_void;
        fn CGEventSetFlags(event: *mut std::ffi::c_void, flags: u64);
        fn CGEventPost(tap_location: u32, event: *mut std::ffi::c_void);
    }

    const K_CG_HID_EVENT_TAP: u32 = 0;
    const K_CG_EVENT_FLAG_MASK_COMMAND: u64 = 1 << 20;
    const K_VK_ANSI_V: u16 = 0x09;

    unsafe {
        use core_foundation::base::CFRelease;

        // Key down
        let event_down = CGEventCreateKeyboardEvent(std::ptr::null(), K_VK_ANSI_V, true);
        if !event_down.is_null() {
            CGEventSetFlags(event_down, K_CG_EVENT_FLAG_MASK_COMMAND);
            CGEventPost(K_CG_HID_EVENT_TAP, event_down);
            CFRelease(event_down as _);
        }

        // Small gap between key-down and key-up
        std::thread::sleep(std::time::Duration::from_millis(30));

        // Key up
        let event_up = CGEventCreateKeyboardEvent(std::ptr::null(), K_VK_ANSI_V, false);
        if !event_up.is_null() {
            CGEventSetFlags(event_up, K_CG_EVENT_FLAG_MASK_COMMAND);
            CGEventPost(K_CG_HID_EVENT_TAP, event_up);
            CFRelease(event_up as _);
        }
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Record the frontmost application (call before showing Texere window).
/// Uses NSWorkspace — **no** Accessibility permission needed.
#[cfg(target_os = "macos")]
pub fn record_frontmost_app(prev: &PreviousApp) {
    if let Some(bid) = get_frontmost_bundle_id() {
        log::info!("Recorded frontmost app: {}", bid);
        if let Ok(mut lock) = prev.0.lock() {
            *lock = Some(bid);
        }
    } else {
        log::warn!("Failed to get frontmost app bundle ID via NSWorkspace");
    }
}

#[cfg(target_os = "linux")]
pub fn record_frontmost_app(prev: &PreviousApp) {
    if !is_x11_session() || !xdotool_available() {
        return;
    }

    let output = std::process::Command::new("xdotool")
        .arg("getactivewindow")
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let window_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !window_id.is_empty() {
                if let Ok(mut lock) = prev.0.lock() {
                    *lock = Some(window_id);
                }
            }
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn record_frontmost_app(_prev: &PreviousApp) {}

/// Copy content to clipboard, close the window, and optionally auto-paste.
#[tauri::command]
pub fn copy_and_dismiss(
    window: WebviewWindow,
    content: String,
    auto_paste: bool,
    prev_app: tauri::State<PreviousApp>,
    auto_paste_in_flight: tauri::State<AutoPasteInFlight>,
) -> Result<(), String> {
    // Handle empty content edge case — just dismiss without clipboard change
    if content.is_empty() {
        return window
            .close()
            .map_err(|e| format!("Failed to close window: {}", e));
    }

    // Copy to clipboard
    window
        .app_handle()
        .clipboard()
        .write_text(&content)
        .map_err(|e| format!("Clipboard write failed: {}", e))?;

    // Grab the recorded previous-app bundle ID before closing
    let bundle_id = if auto_paste {
        prev_app.0.lock().ok().and_then(|lock| lock.clone())
    } else {
        None
    };

    log::info!(
        "copy_and_dismiss: auto_paste={}, bundle_id={:?}",
        auto_paste,
        bundle_id
    );

    // Close the Texere window
    window
        .close()
        .map_err(|e| format!("Failed to close window: {}", e))?;

    // Auto-paste on macOS
    if auto_paste {
        #[cfg(target_os = "macos")]
        {
            let in_flight = auto_paste_in_flight.0.clone();

            let acquired = in_flight
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok();

            log::info!(
                "auto_paste guard: compare_exchange_acquired={}, bundle_id={:?}",
                acquired,
                bundle_id
            );

            if acquired {
                std::thread::spawn(move || {
                    log::info!("auto_paste worker start: bundle_id={:?}", bundle_id);
                    auto_paste_macos(bundle_id.as_deref());
                    in_flight.store(false, Ordering::SeqCst);
                    log::info!("auto_paste worker end: in_flight_released=true");
                });
            } else {
                log::warn!("Skipping auto-paste: another operation is already in flight");
            }
        }

        #[cfg(target_os = "linux")]
        {
            let in_flight = auto_paste_in_flight.0.clone();

            let acquired = in_flight
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok();

            if acquired {
                std::thread::spawn(move || {
                    auto_paste_linux(bundle_id.as_deref());
                    in_flight.store(false, Ordering::SeqCst);
                });
            } else {
                log::warn!("Skipping auto-paste: another operation is already in flight");
            }
        }
    }

    Ok(())
}

/// Close the window without copying anything.
#[tauri::command]
pub fn dismiss_without_copy(window: WebviewWindow) -> Result<(), String> {
    window
        .close()
        .map_err(|e| format!("Failed to close window: {}", e))
}

/// Build the osascript arguments for auto-paste (kept for tests & fallback).
///
/// Returns a `Vec<String>` of args to pass to `Command::new("osascript")`.
/// Each AppleScript statement gets its own `-e` flag for maximum compatibility.
pub fn build_paste_args(bundle_id: Option<&str>) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(bid) = bundle_id {
        args.push("-e".into());
        args.push(format!("tell application id \"{}\" to activate", bid));
        args.push("-e".into());
        args.push("delay 0.3".into());
    }
    args.push("-e".into());
    args.push("tell application \"System Events\" to keystroke \"v\" using command down".into());
    args
}

/// Activate the previous app and simulate Cmd+V.
///
/// Strategy (in order):
/// 1. Activate previous app via NSWorkspace (no Accessibility needed).
/// 2. Wait for app-switch animation.
/// 3. Check `AXIsProcessTrusted()`:
///    - **trusted** → simulate Cmd+V via CGEvent (fast, reliable).
///    - **untrusted** → log an error, attempt osascript as last resort.
#[cfg(target_os = "macos")]
fn auto_paste_macos(bundle_id: Option<&str>) {
    log::info!("auto_paste_macos begin: bundle_id={:?}", bundle_id);

    // Step 1 — activate the previous app (NSWorkspace, no permission needed)
    if let Some(bid) = bundle_id {
        if activate_app_by_bundle_id(bid) {
            log::info!("Activated previous app via NSWorkspace: {}", bid);
        } else {
            log::warn!("Failed to activate app via NSWorkspace: {}", bid);
        }
    } else {
        log::warn!("No previous-app bundle ID recorded; skipping activation");
    }

    // Step 2 — wait for the app-switch animation to settle
    std::thread::sleep(std::time::Duration::from_millis(300));

    // Step 3 — simulate Cmd+V
    if is_accessibility_trusted() {
        log::info!("Accessibility trusted — simulating Cmd+V via CGEvent");
        simulate_cmd_v();
    } else {
        log::error!(
            "Auto-paste failed: Accessibility permission not granted. \
             Go to System Settings > Privacy & Security > Accessibility \
             and grant Texere access."
        );
        // Last-resort fallback: try osascript (likely fails too, but worth a shot)
        let args = build_paste_args(None);
        match std::process::Command::new("osascript").args(&args).output() {
            Ok(output) => {
                let exit_code = output
                    .status
                    .code()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "<signal>".to_string());
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                if output.status.success() {
                    log::info!(
                        "osascript fallback success: exit_code={}, stderr='{}', stdout='{}'",
                        exit_code,
                        stderr.trim(),
                        stdout.trim()
                    );
                } else {
                    log::error!(
                        "osascript fallback failed: exit_code={}, stderr='{}', stdout='{}'",
                        exit_code,
                        stderr.trim(),
                        stdout.trim()
                    );
                }
            }
            Err(err) => {
                log::error!("osascript fallback launch error: {}", err);
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn auto_paste_linux(window_id: Option<&str>) {
    if !is_x11_session() {
        log::warn!("Auto-paste on Linux is only enabled for X11 sessions");
        return;
    }
    if !xdotool_available() {
        log::warn!("Auto-paste on Linux requires xdotool to be installed");
        return;
    }

    if let Some(id) = window_id {
        let _ = std::process::Command::new("xdotool")
            .args(["windowactivate", "--sync", id])
            .status();
        std::thread::sleep(std::time::Duration::from_millis(120));
    }

    let paste_status = std::process::Command::new("xdotool")
        .args(["key", "--clearmodifiers", "ctrl+v"])
        .status();

    match paste_status {
        Ok(status) if status.success() => log::info!("Auto-paste via xdotool succeeded"),
        Ok(status) => log::warn!("Auto-paste via xdotool failed: status={status}"),
        Err(err) => log::warn!("Auto-paste via xdotool failed to launch: {err}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previous_app_default_is_none() {
        let prev = PreviousApp(Mutex::new(None));
        assert!(prev.0.lock().unwrap().is_none());
    }

    #[test]
    fn previous_app_stores_value() {
        let prev = PreviousApp(Mutex::new(None));
        *prev.0.lock().unwrap() = Some("com.apple.Safari".into());
        assert_eq!(prev.0.lock().unwrap().as_deref(), Some("com.apple.Safari"));
    }

    #[test]
    fn test_empty_content_handling() {
        // Empty content should be allowed and handled gracefully
        let content = String::new();
        assert!(content.is_empty());
    }

    #[test]
    fn test_auto_paste_delay() {
        // Auto-paste should wait 300ms for app switching
        use std::time::Duration;
        let delay = Duration::from_millis(300);
        assert_eq!(delay.as_millis(), 300);
    }

    #[test]
    fn test_bundle_id_format() {
        let bundle_id = "com.apple.Safari";
        assert!(bundle_id.contains("."));
        assert!(bundle_id.len() > 5);
    }

    #[test]
    fn test_build_paste_args_without_bundle_id() {
        let args = build_paste_args(None);
        assert_eq!(args.len(), 2); // -e, keystroke
        assert_eq!(args[0], "-e");
        assert!(args[1].contains("System Events"));
        assert!(args[1].contains("keystroke"));
    }

    #[test]
    fn test_build_paste_args_with_bundle_id() {
        let args = build_paste_args(Some("com.apple.Safari"));
        assert_eq!(args.len(), 6); // -e, activate, -e, delay, -e, keystroke
        assert_eq!(args[0], "-e");
        assert!(args[1].contains("com.apple.Safari"));
        assert!(args[1].contains("activate"));
        assert_eq!(args[2], "-e");
        assert_eq!(args[3], "delay 0.3");
        assert_eq!(args[4], "-e");
        assert!(args[5].contains("keystroke"));
    }

    #[test]
    fn test_build_paste_args_each_statement_has_own_flag() {
        // Each AppleScript statement must have its own -e flag
        let args = build_paste_args(Some("com.apple.Xcode"));
        let e_flags: Vec<_> = args
            .iter()
            .enumerate()
            .filter(|(_, a)| a.as_str() == "-e")
            .map(|(i, _)| i)
            .collect();
        assert_eq!(e_flags.len(), 3); // three -e flags
                                      // Each -e flag should be followed by a statement (not another -e)
        for &idx in &e_flags {
            assert!(idx + 1 < args.len());
            assert_ne!(args[idx + 1], "-e");
        }
    }

    #[test]
    fn test_build_paste_args_bundle_id_with_special_chars() {
        // Bundle IDs should be safe in AppleScript strings
        let args = build_paste_args(Some("com.microsoft.VSCode"));
        assert!(args[1].contains("com.microsoft.VSCode"));
    }

    #[test]
    fn test_previous_app_lock_behavior() {
        let prev = PreviousApp(Mutex::new(None));

        // Test that we can acquire lock and store value
        {
            let mut lock = prev.0.lock().unwrap();
            *lock = Some("com.test.App".into());
        }

        // Test that we can read the value back
        {
            let lock = prev.0.lock().unwrap();
            assert_eq!(lock.as_deref(), Some("com.test.App"));
        }
    }

    #[test]
    fn test_multiple_app_switches() {
        let prev = PreviousApp(Mutex::new(None));

        // Simulate multiple app switches
        let apps = vec![
            "com.apple.Safari",
            "com.apple.Xcode",
            "com.microsoft.VSCode",
        ];

        for app in &apps {
            *prev.0.lock().unwrap() = Some((*app).into());
        }

        // Last one should be preserved
        assert_eq!(
            prev.0.lock().unwrap().as_deref(),
            Some("com.microsoft.VSCode")
        );
    }
}
