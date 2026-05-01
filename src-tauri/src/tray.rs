use crate::types::Note;
use tauri::{
    menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    App, AppHandle, Manager,
};

/// Menu item IDs for the system tray context menu.
pub mod menu_ids {
    pub const NEW_EDITOR: &str = "new_editor";
    pub const SETTINGS: &str = "settings";
    pub const QUIT: &str = "quit";
}

pub const TRAY_TOOLTIP: &str = "Texere";

/// Returns all static actionable menu item IDs (excludes note items and separators).
pub fn menu_item_ids() -> Vec<&'static str> {
    vec![menu_ids::NEW_EDITOR, menu_ids::SETTINGS, menu_ids::QUIT]
}

/// Returns (id, label) pairs for static menu items in display order.
pub fn menu_item_labels() -> Vec<(&'static str, &'static str)> {
    vec![
        (menu_ids::NEW_EDITOR, "New Note"),
        (menu_ids::SETTINGS, "Settings..."),
        (menu_ids::QUIT, "Quit Texere"),
    ]
}

/// Newtype wrapper so we can call `app.manage(ManagedTray(...))`.
pub struct ManagedTray(pub tauri::tray::TrayIcon<tauri::Wry>);

/// Build the tray `Menu` from the current notes list.
///
/// Structure with notes:   New Note → ─── → 📌 Note … → ─── → Settings… → ─── → Quit
/// Structure without notes: New Note → ─── → Settings… → ─── → Quit
fn build_tray_menu(
    manager: &impl Manager<tauri::Wry>,
    notes: &[Note],
) -> tauri::Result<Menu<tauri::Wry>> {
    let new_note =
        MenuItem::with_id(manager, menu_ids::NEW_EDITOR, "New Note", true, None::<&str>)?;
    let sep_notes = PredefinedMenuItem::separator(manager)?;
    let sep_settings = PredefinedMenuItem::separator(manager)?;
    let sep_quit = PredefinedMenuItem::separator(manager)?;
    let settings =
        MenuItem::with_id(manager, menu_ids::SETTINGS, "Settings...", true, None::<&str>)?;
    let quit =
        MenuItem::with_id(manager, menu_ids::QUIT, "Quit Texere", true, None::<&str>)?;

    // Pre-build note menu items (empty when there are no notes)
    let note_items: Vec<MenuItem<tauri::Wry>> = notes
        .iter()
        .map(|note| {
            let id = format!("note:{}", note.id);
            let label = format!("📌 {}", note.name);
            MenuItem::with_id(manager, id, label, true, None::<&str>)
                .expect("failed to create note menu item")
        })
        .collect();

    let mut items: Vec<&dyn IsMenuItem<tauri::Wry>> = vec![&new_note];

    if !notes.is_empty() {
        items.push(&sep_notes);
        for item in &note_items {
            items.push(item);
        }
    }

    items.push(&sep_settings);
    items.push(&settings);
    items.push(&sep_quit);
    items.push(&quit);

    Menu::with_items(manager, &items)
}

/// Rebuild the Tray menu in-place after a notes change.
/// `notes` is the current full list (already saved to disk by the caller).
pub fn rebuild_tray(app: &AppHandle, notes: &[Note]) -> Result<(), String> {
    let tray_state = app.state::<ManagedTray>();
    let menu = build_tray_menu(app, notes).map_err(|e| e.to_string())?;
    tray_state
        .0
        .set_menu(Some(menu))
        .map_err(|e| e.to_string())
}

/// Set up the system tray icon with menu and event handlers.
///
/// Menu structure (no notes):
///   New Note → ─── → Settings… → ─── → Quit Texere
///
/// Menu structure (with notes):
///   New Note → ─── → 📌 Note1 → 📌 Note2 → ─── → Settings… → ─── → Quit Texere
pub fn setup_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    // Load notes to initialise the menu correctly on startup
    let initial_notes = crate::notes::load_notes(&app.handle());
    let menu = build_tray_menu(app, &initial_notes)?;

    let mut builder = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .tooltip(TRAY_TOOLTIP)
        .on_menu_event(|app, event| {
            let id = event.id.as_ref();

            // Handle note menu items (id = "note:{uuid}")
            if id.starts_with("note:") {
                let note_id = id.trim_start_matches("note:").to_string();
                let notes = crate::notes::load_notes(app);
                if let Some(note) = notes.iter().find(|n| n.id == note_id) {
                    let _ = crate::window::open_note_window_internal(
                        app,
                        note_id,
                        note.name.clone(),
                        note.content.clone(),
                    );
                }
                return;
            }

            match id {
                "new_editor" => {
                    // Open a fresh editor window from the pool
                    let _ = crate::window::show_pooled_window(
                        app.clone(),
                        app.state::<crate::window::WindowPool>(),
                        app.state::<crate::clipboard::PreviousApp>(),
                        app.state::<crate::clipboard::AutoPasteInFlight>(),
                    );
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
            }
        });

    // Use the app's default icon for the tray
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    let tray = builder.build(app)?;
    // Persist the TrayIcon handle so rebuild_tray can call set_menu later
    app.manage(ManagedTray(tray));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tray_menu_has_three_static_items() {
        assert_eq!(menu_item_ids().len(), 3);
    }

    #[test]
    fn tray_menu_item_ids_are_correct() {
        assert_eq!(menu_item_ids(), vec!["new_editor", "settings", "quit"]);
    }

    #[test]
    fn tray_menu_labels_match_ids() {
        let labels = menu_item_labels();
        assert_eq!(labels[0], ("new_editor", "New Note"));
        assert_eq!(labels[1], ("settings", "Settings..."));
        assert_eq!(labels[2], ("quit", "Quit Texere"));
    }

    #[test]
    fn tray_menu_order_new_note_first_quit_last() {
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
        assert_eq!(menu_ids::NEW_EDITOR, "new_editor");
        assert_eq!(menu_ids::SETTINGS, "settings");
        assert_eq!(menu_ids::QUIT, "quit");
    }

    #[test]
    fn note_menu_id_prefix_convention() {
        let note_id = "abc-123";
        let menu_id = format!("note:{}", note_id);
        assert!(menu_id.starts_with("note:"));
        assert_eq!(menu_id.trim_start_matches("note:"), note_id);
    }
}
