use crate::types::Template;
use tauri::{AppHandle, Manager};

fn templates_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
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

    Ok(app_data_dir.join("templates.json"))
}

fn load_templates(app: &AppHandle) -> Vec<Template> {
    let path = match templates_path(app) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("load_templates path error: {}", e);
            return Vec::new();
        }
    };

    let content = match std::fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    serde_json::from_str::<Vec<Template>>(&content).unwrap_or_default()
}

fn save_templates(app: &AppHandle, templates: &[Template]) -> Result<(), String> {
    let path = templates_path(app)?;
    let tmp_path = path.with_extension("json.tmp");
    let payload = serde_json::to_string_pretty(templates)
        .map_err(|e| format!("serialize templates failed: {}", e))?;

    std::fs::write(&tmp_path, payload).map_err(|e| {
        format!(
            "write temp templates failed ({}): {}",
            tmp_path.display(),
            e
        )
    })?;

    if path.exists() {
        let _ = std::fs::remove_file(&path);
    }

    std::fs::rename(&tmp_path, &path).map_err(|e| {
        format!(
            "rename temp templates failed ({} -> {}): {}",
            tmp_path.display(),
            path.display(),
            e
        )
    })
}

#[tauri::command]
pub fn get_templates(app: AppHandle) -> Vec<Template> {
    load_templates(&app)
}

#[tauri::command]
pub fn save_template(app: AppHandle, template: Template) -> Result<(), String> {
    let mut templates = load_templates(&app);

    // Upsert: update existing or add new
    if let Some(pos) = templates.iter().position(|t| t.id == template.id) {
        templates[pos] = template;
    } else {
        templates.push(template);
    }

    save_templates(&app, &templates)
}

#[tauri::command]
pub fn delete_template(app: AppHandle, id: String) -> Result<(), String> {
    let mut templates = load_templates(&app);
    let original_len = templates.len();

    templates.retain(|t| t.id != id);

    if templates.len() == original_len {
        return Err(format!("template not found: {}", id));
    }

    save_templates(&app, &templates)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_template_commands_signature() {
        // Ensure the commands are correctly defined
        // Actual testing requires a full AppHandle with store
    }
}
