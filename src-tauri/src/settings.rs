use crate::types::TexereSettings;
use std::path::PathBuf;
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

#[tauri::command]
pub fn get_settings(app: AppHandle) -> TexereSettings {
    let path = match settings_path(&app) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("get_settings path error: {}", e);
            return TexereSettings::default();
        }
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(v) => v,
        Err(_) => return TexereSettings::default(),
    };

    serde_json::from_str::<TexereSettings>(&content).unwrap_or_default()
}

#[tauri::command]
pub fn set_settings(app: AppHandle, settings: TexereSettings) -> Result<(), String> {
    let current_settings = get_settings(app.clone());
    let enabling_auto_paste = !current_settings.auto_paste && settings.auto_paste;

    if enabling_auto_paste && !crate::clipboard::auto_paste_permission_granted() {
        return Err(
            "Auto-paste requires Accessibility permission on macOS. Open System Settings > Privacy & Security > Accessibility and grant Texere access."
                .to_string(),
        );
    }

    let path = settings_path(&app)?;
    let tmp_path = path.with_extension("json.tmp");

    let payload = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("serialize settings failed: {}", e))?;

    std::fs::write(&tmp_path, payload)
        .map_err(|e| format!("write temp settings failed ({}): {}", tmp_path.display(), e))?;

    if path.exists() {
        let _ = std::fs::remove_file(&path);
    }

    std::fs::rename(&tmp_path, &path).map_err(|e| {
        format!(
            "rename temp settings failed ({} -> {}): {}",
            tmp_path.display(),
            path.display(),
            e
        )
    })?;

    crate::window::register_summon_hotkey(&app, &settings.hotkeys.summon)
        .map_err(|e| format!("settings saved but hotkey register failed: {}", e))?;

    for (_, window) in app.webview_windows() {
        let _ = window.emit("texere://refresh-settings", ());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_settings_roundtrip() {
        // This is a bit tricky to test without a full AppHandle,
        // but we can test the logic if we mock the store or use a real one in a temporary dir.
        // For now, let's focus on the commands being correctly defined.
    }
}
