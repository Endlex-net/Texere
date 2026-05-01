pub mod tray;
pub mod appearance;
pub mod types;
pub mod settings;
pub mod window;
pub mod clipboard;
pub mod templates;
pub mod notes;
pub mod cursor_position;
pub mod ai;
pub mod notifications;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  use std::sync::{Arc, atomic::AtomicBool};

  tauri::Builder::default()
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .setup(|app| {
      use tauri_plugin_store::StoreBuilder;
      use tauri::Manager;

      // Hide from Dock — app lives in the menu bar only
      #[cfg(target_os = "macos")]
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      // System tray icon + menu
      tray::setup_tray(app)?;

      let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
      std::fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
      let store_path = app_data_dir.join("app-store.json");
      let store = StoreBuilder::new(app, store_path).build()?;
      app.manage(store.clone());
      app.manage(clipboard::PreviousApp(std::sync::Mutex::new(None)));
      app.manage(clipboard::AutoPasteInFlight(Arc::new(AtomicBool::new(false))));

if cfg!(debug_assertions) {
app.handle().plugin(
tauri_plugin_log::Builder::default()
.level(log::LevelFilter::Info)
.build(),
)?;
}

      let shortcut_handler = |app: &tauri::AppHandle, _hotkey: &tauri_plugin_global_shortcut::Shortcut, event: tauri_plugin_global_shortcut::ShortcutEvent| {
          if event.state == tauri_plugin_global_shortcut::ShortcutState::Released {
              if let Err(e) = window::show_pooled_window(
                  app.clone(),
                  app.state::<window::WindowPool>(),
                  app.state::<clipboard::PreviousApp>(),
                  app.state::<clipboard::AutoPasteInFlight>(),
              ) {
                  eprintln!("Failed to show pooled window from hotkey: {}", e);
              }
          }
      };

      app.handle().plugin(
          tauri_plugin_global_shortcut::Builder::new()
              .with_handler(shortcut_handler)
              .build()
      )?;

      let settings_val = settings::get_settings(app.handle().clone());
      let pool = window::init_pool(app.handle());
      app.manage(pool);
      let note_window_map: window::NoteWindowMap = std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
      app.manage(note_window_map);
      app.manage(notes::NotesLock(std::sync::Mutex::new(())));

      if let Err(e) = window::register_summon_hotkey(app.handle(), &settings_val.hotkeys.summon) {
          eprintln!("Failed to register summon hotkey: {}", e);
      }

      Ok(())
    })
.invoke_handler(tauri::generate_handler![
      settings::get_settings,
      settings::set_settings,
      window::show_pooled_window,
      window::close_window,
      window::list_windows,
      window::open_note_window,
      clipboard::copy_and_dismiss,
      clipboard::dismiss_without_copy,
      clipboard::can_enable_auto_paste,
      cursor_position::get_cursor_screen_position_command,
      templates::get_templates,
      templates::save_template,
      templates::delete_template,
      notes::get_notes,
      notes::save_note,
      notes::delete_note,
      ai::format_text
])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
