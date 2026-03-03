use crate::types::TexereSettings;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager};

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("resolve app_data_dir failed: {}", e))?;

    std::fs::create_dir_all(&app_data_dir).map_err(|e| {
        format!(
            "create app_data_dir failed ({}): {}",
            app_data_dir.display(),
            e
        )
    })?;

    Ok(app_data_dir.join("settings.json"))
}

fn read_settings_from_path(path: &Path) -> TexereSettings {
    let content = match std::fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => return TexereSettings::default(),
    };

    serde_json::from_str::<TexereSettings>(&content).unwrap_or_default()
}

fn write_settings_to_path(path: &Path, settings: &TexereSettings) -> Result<(), String> {
    let tmp_path = path.with_extension("json.tmp");
    let payload = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("serialize settings failed: {}", e))?;

    std::fs::write(&tmp_path, payload)
        .map_err(|e| format!("write temp settings failed ({}): {}", tmp_path.display(), e))?;

    if path.exists() {
        let _ = std::fs::remove_file(path);
    }

    std::fs::rename(&tmp_path, path).map_err(|e| {
        format!(
            "rename temp settings failed ({} -> {}): {}",
            tmp_path.display(),
            path.display(),
            e
        )
    })
}

fn validate_auto_paste_transition(
    current_settings: &TexereSettings,
    new_settings: &TexereSettings,
    permission_granted: bool,
) -> Result<(), String> {
    let enabling_auto_paste = !current_settings.auto_paste && new_settings.auto_paste;

    if enabling_auto_paste && !permission_granted {
        return Err(
            "Auto-paste requires Accessibility permission on macOS. Open System Settings > Privacy & Security > Accessibility and grant Texere access."
                .to_string(),
        );
    }

    Ok(())
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> TexereSettings {
    let path = match settings_path(&app) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("get_settings path error: {}", e);
            return TexereSettings::default();
        }
    };

    read_settings_from_path(&path)
}

#[tauri::command]
pub fn set_settings(app: AppHandle, settings: TexereSettings) -> Result<(), String> {
    let current_settings = get_settings(app.clone());
    validate_auto_paste_transition(
        &current_settings,
        &settings,
        crate::clipboard::auto_paste_permission_granted(),
    )?;

    let path = settings_path(&app)?;
    write_settings_to_path(&path, &settings)?;

    crate::window::register_summon_hotkey(&app, &settings.hotkeys.summon)
        .map_err(|e| format!("settings saved but hotkey register failed: {}", e))?;

    for (_, window) in app.webview_windows() {
        let _ = window.emit("texere://refresh-settings", ());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("texere-settings-{}-{}", name, Uuid::new_v4()))
    }

    #[test]
    fn invalid_settings_file_returns_default() {
        let path = temp_path("invalid");
        std::fs::write(&path, "{not-json").expect("write invalid json");

        let settings = read_settings_from_path(&path);
        assert_eq!(settings, TexereSettings::default());

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn enabling_auto_paste_requires_permission() {
        let current = TexereSettings::default();
        let mut next = TexereSettings::default();
        next.auto_paste = true;

        let result = validate_auto_paste_transition(&current, &next, false);
        assert!(result.is_err());
    }

    #[test]
    fn write_failure_returns_error() {
        let base = temp_path("missing-parent");
        let nested_path = base.join("subdir").join("settings.json");

        let result = write_settings_to_path(&nested_path, &TexereSettings::default());
        assert!(result.is_err());
    }
}
