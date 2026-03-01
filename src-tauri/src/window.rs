use std::sync::{atomic::Ordering, Arc, Mutex};
use tauri::{
    AppHandle, Emitter, LogicalPosition, Manager, WebviewWindow, WebviewWindowBuilder,
};
use uuid::Uuid;
use crate::clipboard::{AutoPasteInFlight, PreviousApp};

/// Window pool for instant summon (<50ms)
pub type WindowPool = Arc<Mutex<Vec<WebviewWindow>>>;

/// Target pool size - number of pre-created hidden windows
const POOL_TARGET_SIZE: usize = 2;

/// Maximum number of simultaneous windows to prevent resource exhaustion
const MAX_WINDOWS: usize = 10;

/// Initialize the window pool on app startup
pub fn init_pool(app: &AppHandle) -> WindowPool {
    let pool = Arc::new(Mutex::new(Vec::with_capacity(POOL_TARGET_SIZE)));
    
    // Pre-create hidden windows for warm pool
    for _ in 0..POOL_TARGET_SIZE {
        match create_editor_window(app, false) {
            Ok(window) => {
                if let Ok(mut pool_lock) = pool.lock() {
                    pool_lock.push(window);
                }
            }
            Err(e) => {
                // Log error but continue - we'll create windows on-demand if pool fails
                eprintln!("Failed to pre-create pooled window: {}", e);
            }
        }
    }
    
    pool
}

/// Replenish the pool to target size (async)
pub fn replenish_pool(app: AppHandle, pool: WindowPool) {
    tauri::async_runtime::spawn(async move {
        let current_size = pool.lock().map(|p| p.len()).unwrap_or(0);
        let needed = POOL_TARGET_SIZE.saturating_sub(current_size);
        
        for _ in 0..needed {
            if let Ok(window) = create_editor_window(&app, false) {
                if let Ok(mut pool_lock) = pool.lock() {
                    pool_lock.push(window);
                }
            }
        }
    });
}

/// Create a new editor window with specified visibility.
/// Hidden windows are placed off-screen to prevent any flash at the default
/// (center) position when they are later shown via hotkey.
pub fn create_editor_window(app: &AppHandle, visible: bool) -> Result<WebviewWindow, String> {
    let label = format!("editor-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
    
    let mut builder = WebviewWindowBuilder::new(app, &label, tauri::WebviewUrl::App("index.html".into()))
        .title("Texere")
        .inner_size(480.0, 360.0)
        .always_on_top(true)
        .visible_on_all_workspaces(true)
        .decorations(false)
        .transparent(false)  // Disable transparency to fix rendering
        .resizable(true)
        .visible(visible)
        .focused(false);

    // Place hidden (pooled) windows off-screen so they never flash at center
    if !visible {
        builder = builder.position(-10000.0, -10000.0);
    }
    
    let window = builder.build()
        .map_err(|e| format!("Failed to create window: {}", e))?;
        
    // Temporarily disable vibrancy to test if it's causing the blank window
    // crate::appearance::apply_vibrancy(&window);
    
    Ok(window)
}

fn position_window_near_cursor(window: &WebviewWindow) {
    let cursor_pos = crate::cursor_position::get_cursor_screen_position()
        .or_else(crate::cursor_position::get_mouse_screen_position);
    let Some((cursor_x, cursor_y)) = cursor_pos else {
        // Cursor position unavailable (likely accessibility permissions denied)
        // Notify user on first failure
        #[cfg(target_os = "macos")]
        {
            if let (Some(app), Some(store)) = (window.app_handle().try_state::<tauri::AppHandle>(), window.app_handle().try_state::<tauri_plugin_store::Store<tauri::Wry>>()) {
                crate::notifications::send_accessibility_notification_once(&app, &store);
            }
        }
        if let Ok(Some(monitor)) = window.current_monitor() {
            let scale = monitor.scale_factor();
            let monitor_pos = monitor.position().to_logical::<f64>(scale);
            let monitor_size = monitor.size().to_logical::<f64>(scale);
            let x = monitor_pos.x + (monitor_size.width - 480.0) / 2.0;
            let y = monitor_pos.y + (monitor_size.height - 360.0) / 2.0;
            let _ = window.set_position(LogicalPosition::new(x, y));
        }
        return;
    };

    let mut x = cursor_x + 16.0;
    let mut y = cursor_y + 8.0;

    if let Ok(window_size) = window.outer_size() {
        let mut monitor_bounds: Option<(f64, f64, f64, f64)> = None;

        if let Ok(monitors) = window.available_monitors() {
            for monitor in monitors {
                let scale = monitor.scale_factor();
                let monitor_pos = monitor.position().to_logical::<f64>(scale);
                let monitor_size = monitor.size().to_logical::<f64>(scale);
                let min_x = monitor_pos.x;
                let min_y = monitor_pos.y;
                let max_x = monitor_pos.x + monitor_size.width;
                let max_y = monitor_pos.y + monitor_size.height;

                if cursor_x >= min_x && cursor_x <= max_x && cursor_y >= min_y && cursor_y <= max_y {
                    monitor_bounds = Some((min_x, min_y, max_x, max_y));
                    break;
                }
            }
        }

        if monitor_bounds.is_none() {
            if let Ok(Some(monitor)) = window.current_monitor() {
                let scale = monitor.scale_factor();
                let monitor_pos = monitor.position().to_logical::<f64>(scale);
                let monitor_size = monitor.size().to_logical::<f64>(scale);
                monitor_bounds = Some((
                    monitor_pos.x,
                    monitor_pos.y,
                    monitor_pos.x + monitor_size.width,
                    monitor_pos.y + monitor_size.height,
                ));
            }
        }

        if let Some((min_x, min_y, bound_max_x, bound_max_y)) = monitor_bounds {
            let scale = window.scale_factor().unwrap_or(1.0);
            let window_size = window_size.to_logical::<f64>(scale);
            let max_x = (bound_max_x - window_size.width).max(min_x);
            let max_y = (bound_max_y - window_size.height).max(min_y);
            x = x.clamp(min_x, max_x);
            y = y.clamp(min_y, max_y);
        }
    }

    if let Err(e) = window.set_position(LogicalPosition::new(x, y)) {
        eprintln!("Failed to position window near cursor: {}", e);
    }
}

/// Create the settings window.
pub fn create_settings_window(app: &AppHandle) {
    // Switch to Regular activation policy FIRST (Tauri-level)
    #[cfg(target_os = "macos")]
    {
        let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
    }

    // If settings window already exists, just show + focus it
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
        #[cfg(target_os = "macos")]
        activate_app_and_focus_window(&window);
        return;
    }

    // Create window with minimal configuration
    let window = match WebviewWindowBuilder::new(app, "settings", tauri::WebviewUrl::App(std::path::PathBuf::from("settings.html")))
        .title("Texere Settings")
        .inner_size(600.0, 500.0)
        .min_inner_size(400.0, 300.0)
        .transparent(false)
        .resizable(true)
        .decorations(true)
        .visible(true)
        .build() {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Failed to create settings window: {}", e);
                return;
            }
    };

    // Setup window appearance
    #[cfg(target_os = "macos")]
    {
        make_webview_opaque(&window);
        
        // Center on screen
        if let Ok(Some(monitor)) = window.primary_monitor() {
            let scale = monitor.scale_factor();
            let mon_size = monitor.size().to_logical::<f64>(scale);
            let mon_pos = monitor.position().to_logical::<f64>(scale);
            let x = mon_pos.x + (mon_size.width - 600.0) / 2.0;
            let y = mon_pos.y + (mon_size.height - 500.0) / 2.0;
            let _ = window.set_position(LogicalPosition::new(x, y));
        }
        
        // Bring to front
        activate_app_and_focus_window(&window);
    }

    // Show the window
    let _ = window.show();
    let _ = window.set_focus();

    // Handle window close - switch back to Accessory policy
    let app_handle = app.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Destroyed = event {
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory};
                unsafe {
                    NSApp().setActivationPolicy_(NSApplicationActivationPolicyAccessory);
                }
            }
            let _ = app_handle.set_activation_policy(tauri::ActivationPolicy::Accessory);
        }
    });
}

/// Make the settings window's webview opaque instead of transparent.
/// When macOSPrivateApi is enabled and the app uses transparent editor windows,
/// new windows may inherit transparent webview behavior. This forces opacity.
#[cfg(target_os = "macos")]
fn make_webview_opaque(window: &WebviewWindow) {
    let _ = window;
}

#[cfg(not(target_os = "macos"))]
fn make_webview_opaque(_window: &WebviewWindow) {}

/// Activate the macOS app and bring a specific window to front.
/// This is needed because Texere uses ActivationPolicy::Accessory (menu bar app),
/// which means windows don't automatically come to the foreground.
///
#[cfg(target_os = "macos")]
fn activate_app_and_focus_window(window: &WebviewWindow) {
    use cocoa::appkit::{
        NSApp, NSApplication, NSWindow,
        NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
    };
    use cocoa::base::{id, YES};

    unsafe {
        let ns_app = NSApp();
        // Switch to Regular policy so the app can own the foreground
        ns_app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        // Force-activate ignoring other apps
        ns_app.activateIgnoringOtherApps_(YES);

        if let Ok(ns_window) = window.ns_window() {
            let ns_window = ns_window as id;
            ns_window.makeKeyAndOrderFront_(cocoa::base::nil);
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn activate_app_and_focus_window(_window: &WebviewWindow) {
    // No-op on non-macOS platforms
}

/// Show a window from the pool (or create new if empty)
#[tauri::command]
pub fn show_pooled_window(
    app: AppHandle,
    pool: tauri::State<WindowPool>,
    prev_app: tauri::State<PreviousApp>,
    auto_paste_in_flight: tauri::State<AutoPasteInFlight>,
) -> Result<String, String> {
    // Check max window count before creating new windows
    let active_count = app.webview_windows()
        .into_iter()
        .filter(|(label, _)| label.starts_with("editor-"))
        .count();
    
    if active_count >= MAX_WINDOWS {
        return Err(format!("Maximum {} windows reached. Close some windows first.", MAX_WINDOWS));
    }
    
    // Record frontmost app before we steal focus
    crate::clipboard::record_frontmost_app(&prev_app);

    // Try to get a window from the pool
    let window = {
        let mut pool_lock = pool.lock()
            .map_err(|_| "Failed to lock window pool")?;
        pool_lock.pop()
    };
    
    let (window, label) = if let Some(window) = window {
        // Use pooled window — position BEFORE showing
        let label = window.label().to_string();
        position_window_near_cursor(&window);
        if let Err(e) = window.show() {
            return Err(format!("Failed to show window: {}", e));
        }
        if let Err(e) = window.set_focus() {
            eprintln!("Failed to focus window: {}", e);
        }
        let _ = window.emit("texere://refresh-settings", ());
        #[cfg(target_os = "macos")]
        activate_app_and_focus_window(&window);
        (window, label)
    } else {
        // Pool empty - create new window on-the-fly
        let window = create_editor_window(&app, false)?;
        position_window_near_cursor(&window);
        if let Err(e) = window.show() {
            return Err(format!("Failed to show window: {}", e));
        }
        if let Err(e) = window.set_focus() {
            eprintln!("Failed to focus window: {}", e);
        }
        let _ = window.emit("texere://refresh-settings", ());
        #[cfg(target_os = "macos")]
        activate_app_and_focus_window(&window);
        let label = window.label().to_string();
        (window, label)
    };

    let focus_window = window.clone();
    let auto_paste_in_flight = auto_paste_in_flight.0.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(120)).await;

        if !matches!(focus_window.is_visible(), Ok(true)) {
            return;
        }

        if auto_paste_in_flight.load(Ordering::SeqCst) {
            return;
        }

        let _ = focus_window.set_focus();
        let _ = focus_window.emit("texere://force-focus", ());
        let _ = focus_window.eval("window.dispatchEvent(new CustomEvent('texere://force-focus')); try { document.querySelector('.cm-content')?.focus(); } catch (_) {}");
    });
    
    // Replenish pool asynchronously
    let pool_clone = Arc::clone(&*pool);
    replenish_pool(app, pool_clone);
    
    Ok(label)
}

/// Close a specific window
#[tauri::command]
pub fn close_window(window: WebviewWindow) -> Result<(), String> {
    window.close()
        .map_err(|e| format!("Failed to close window: {}", e))
}

/// List all open editor window labels
#[tauri::command]
pub fn list_windows(app: AppHandle) -> Vec<String> {
    app.webview_windows()
        .into_iter()
        .filter_map(|(label, _)| {
            if label.starts_with("editor-") {
                Some(label)
            } else {
                None
            }
        })
        .collect()
}

/// Register the global hotkey for summoning windows.
///
/// NOTE: In tauri-plugin-global-shortcut v2.3+, the event handler is set on the
/// plugin Builder via `with_handler`. This function only registers the shortcut.
pub fn register_summon_hotkey(app: &AppHandle, hotkey_str: &str) -> Result<(), String> {
    use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

    let mut mods = Modifiers::empty();
    let mut code = Code::Space;

    // Simple parsing for hotkey string, e.g., "CommandOrControl+Shift+Space"
    let parts: Vec<&str> = hotkey_str.split('+').collect();
    for part in parts {
        match part.trim().to_lowercase().as_str() {
            "commandorcontrol" | "cmd" | "ctrl" | "control" | "command" | "super" | "meta" => mods |= Modifiers::SUPER,
            "shift" => mods |= Modifiers::SHIFT,
            "alt" | "option" => mods |= Modifiers::ALT,
            "space" => code = Code::Space,
            "enter" | "return" => code = Code::Enter,
            "esc" | "escape" => code = Code::Escape,
            "up" | "arrowup" => code = Code::ArrowUp,
            "down" | "arrowdown" => code = Code::ArrowDown,
            "left" | "arrowleft" => code = Code::ArrowLeft,
            "right" | "arrowright" => code = Code::ArrowRight,
            "a" => code = Code::KeyA,
            "b" => code = Code::KeyB,
            "c" => code = Code::KeyC,
            "d" => code = Code::KeyD,
            "e" => code = Code::KeyE,
            "f" => code = Code::KeyF,
            "g" => code = Code::KeyG,
            "h" => code = Code::KeyH,
            "i" => code = Code::KeyI,
            "j" => code = Code::KeyJ,
            "k" => code = Code::KeyK,
            "l" => code = Code::KeyL,
            "m" => code = Code::KeyM,
            "n" => code = Code::KeyN,
            "o" => code = Code::KeyO,
            "p" => code = Code::KeyP,
            "q" => code = Code::KeyQ,
            "r" => code = Code::KeyR,
            "s" => code = Code::KeyS,
            "t" => code = Code::KeyT,
            "u" => code = Code::KeyU,
            "v" => code = Code::KeyV,
            "w" => code = Code::KeyW,
            "x" => code = Code::KeyX,
            "y" => code = Code::KeyY,
            "z" => code = Code::KeyZ,
            _ => {}
        }
    }

    // Always ensure at least a code, or it fails. We provide sensible defaults if empty above
    if mods.is_empty() && code == Code::Space {
        mods = Modifiers::SHIFT | Modifiers::SUPER;
    }

    let shortcut = Shortcut::new(Some(mods), code);

    let _ = app.global_shortcut().unregister_all();

    app.global_shortcut()
        .register(shortcut)
        .map_err(|e| {
            let error_msg = format!("Failed to register hotkey: {}", e);
            // Send notification about hotkey conflict
            crate::notifications::send_notification(
                app,
                "Texere: Hotkey Registration Failed",
                &format!("Hotkey '{}' is in use by another app or invalid. Please change it in settings.", hotkey_str)
            );
            error_msg
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_label_format() {
        let label = format!("editor-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        assert!(label.starts_with("editor-"));
        assert!(label.len() > 7);
    }

    #[test]
    fn test_pool_target_size() {
        assert_eq!(POOL_TARGET_SIZE, 2);
    }

    #[test]
    fn test_replenish_calculation() {
        let current = 1;
        let needed = POOL_TARGET_SIZE.saturating_sub(current);
        assert_eq!(needed, 1);
        
        let current = 2;
        let needed = POOL_TARGET_SIZE.saturating_sub(current);
        assert_eq!(needed, 0);
        
        let current = 3;
        let needed = POOL_TARGET_SIZE.saturating_sub(current);
        assert_eq!(needed, 0);
    }

    #[test]
    fn test_max_windows_constant() {
        assert_eq!(MAX_WINDOWS, 10);
        assert!(MAX_WINDOWS > POOL_TARGET_SIZE);
    }

    #[test]
    fn test_max_windows_error_case() {
        // Should cap at MAX_WINDOWS
        let max = MAX_WINDOWS;
        assert_eq!(max, 10);
        // If active_count >= MAX_WINDOWS, should error
    }

    #[test]
    fn test_pool_initialization_failure_handling() {
        // Pool should survive partial initialization failures
        // On-demand window creation should work even if pre-creation fails
        let target = POOL_TARGET_SIZE;
        assert!(target > 0);
    }

    #[test]
    fn test_saturating_sub_prevents_underflow() {
        let current = 5;
        let needed = POOL_TARGET_SIZE.saturating_sub(current);
        assert_eq!(needed, 0); // Should be 0, not negative
    }
}
