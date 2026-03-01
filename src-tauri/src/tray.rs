use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    App, WebviewUrl, WebviewWindowBuilder,
};

/// Menu item IDs for the system tray context menu.
pub mod menu_ids {
    pub const NEW_EDITOR: &str = "new_editor";
    pub const SETTINGS: &str = "settings";
    pub const QUIT: &str = "quit";
}

pub const TRAY_TOOLTIP: &str = "Texere";

/// Returns all actionable menu item IDs in display order (excludes separator).
pub fn menu_item_ids() -> Vec<&'static str> {
    vec![menu_ids::NEW_EDITOR, menu_ids::SETTINGS, menu_ids::QUIT]
}

/// Returns (id, label) pairs for all menu items in display order.
pub fn menu_item_labels() -> Vec<(&'static str, &'static str)> {
    vec![
        (menu_ids::NEW_EDITOR, "New Editor"),
        (menu_ids::SETTINGS, "Settings..."),
        (menu_ids::QUIT, "Quit Texere"),
    ]
}

/// Set up the system tray icon with menu and event handlers.
///
/// Menu structure:
///   - New Editor  → opens a new editor window
///   - Settings... → placeholder (handled by settings module)
///   - ─────────── (separator)
///   - Quit Texere  → exits the application
pub fn setup_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let new_editor =
        MenuItem::with_id(app, menu_ids::NEW_EDITOR, "New Editor", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, menu_ids::SETTINGS, "Settings...", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, menu_ids::QUIT, "Quit Texere", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&new_editor, &settings, &separator, &quit])?;

    let mut builder = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .tooltip(TRAY_TOOLTIP)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "new_editor" => {
                create_editor_window(app);
            }
            "settings" => {
                crate::window::create_settings_window(app);
            }
            "quit" => {
                app.exit(0);
            }
            _ => {
                log::warn!("Unhandled tray menu event: {:?}", event.id);
            }
        });

    // Use the app's default icon for the tray
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    Ok(())
}

/// Create a new editor window matching the standard Texere configuration.
fn create_editor_window(app: &tauri::AppHandle) {
    let label = format!(
        "editor-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    );

    let result = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("index.html".into()))
        .title("Texere")
        .inner_size(480.0, 360.0)
        .always_on_top(true)
        .decorations(false)
        .transparent(true)
        .build();

    if let Err(e) = result {
        log::error!("Failed to create editor window: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tray_menu_has_three_actionable_items() {
        assert_eq!(menu_item_ids().len(), 3);
    }

    #[test]
    fn tray_menu_item_ids_are_correct() {
        assert_eq!(menu_item_ids(), vec!["new_editor", "settings", "quit"]);
    }

    #[test]
    fn tray_menu_labels_match_ids() {
        let labels = menu_item_labels();
        assert_eq!(labels[0], ("new_editor", "New Editor"));
        assert_eq!(labels[1], ("settings", "Settings..."));
        assert_eq!(labels[2], ("quit", "Quit Texere"));
    }

    #[test]
    fn tray_menu_order_new_editor_first_quit_last() {
        let ids = menu_item_ids();
        assert_eq!(ids.first(), Some(&"new_editor"));
        assert_eq!(ids.last(), Some(&"quit"));
    }

    #[test]
    fn tray_tooltip_is_texere() {
        assert_eq!(TRAY_TOOLTIP, "Texere");
    }

    #[test]
    fn menu_id_constants_are_stable() {
        // Guard against accidental renames that would break event matching
        assert_eq!(menu_ids::NEW_EDITOR, "new_editor");
        assert_eq!(menu_ids::SETTINGS, "settings");
        assert_eq!(menu_ids::QUIT, "quit");
    }
}
