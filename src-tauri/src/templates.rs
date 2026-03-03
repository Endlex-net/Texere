use crate::types::Template;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

fn templates_path(app: &AppHandle) -> Result<PathBuf, String> {
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

fn read_templates_from_path(path: &Path) -> Vec<Template> {
    let content = match std::fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    serde_json::from_str::<Vec<Template>>(&content).unwrap_or_default()
}

fn load_templates(app: &AppHandle) -> Vec<Template> {
    let path = match templates_path(app) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("load_templates path error: {}", e);
            return Vec::new();
        }
    };

    read_templates_from_path(&path)
}

fn write_templates_to_path(path: &Path, templates: &[Template]) -> Result<(), String> {
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

fn save_templates(app: &AppHandle, templates: &[Template]) -> Result<(), String> {
    let path = templates_path(app)?;
    write_templates_to_path(&path, templates)
}

fn upsert_template(templates: &mut Vec<Template>, template: Template) {
    if let Some(pos) = templates.iter().position(|t| t.id == template.id) {
        templates[pos] = template;
    } else {
        templates.push(template);
    }
}

fn delete_template_in_memory(templates: &mut Vec<Template>, id: &str) -> Result<(), String> {
    let original_len = templates.len();
    templates.retain(|t| t.id != id);

    if templates.len() == original_len {
        return Err(format!("template not found: {}", id));
    }

    Ok(())
}

#[tauri::command]
pub fn get_templates(app: AppHandle) -> Vec<Template> {
    load_templates(&app)
}

#[tauri::command]
pub fn save_template(app: AppHandle, template: Template) -> Result<(), String> {
    let mut templates = load_templates(&app);
    upsert_template(&mut templates, template);

    save_templates(&app, &templates)
}

#[tauri::command]
pub fn delete_template(app: AppHandle, id: String) -> Result<(), String> {
    let mut templates = load_templates(&app);
    delete_template_in_memory(&mut templates, &id)?;

    save_templates(&app, &templates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn sample_template(id: &str, name: &str) -> Template {
        Template {
            id: id.to_string(),
            name: name.to_string(),
            content: format!("{} content", name),
            created_at: 1,
            updated_at: 1,
        }
    }

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("texere-templates-{}-{}", name, Uuid::new_v4()))
    }

    #[test]
    fn upsert_updates_existing_template() {
        let mut templates = vec![sample_template("1", "Old")];
        upsert_template(&mut templates, sample_template("1", "New"));

        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].name, "New");
    }

    #[test]
    fn upsert_inserts_new_template() {
        let mut templates = vec![sample_template("1", "First")];
        upsert_template(&mut templates, sample_template("2", "Second"));

        assert_eq!(templates.len(), 2);
    }

    #[test]
    fn delete_not_found_returns_error() {
        let mut templates = vec![sample_template("1", "First")];
        let result = delete_template_in_memory(&mut templates, "missing");

        assert!(result.is_err());
    }

    #[test]
    fn persistence_write_failure_returns_error() {
        let base = temp_path("missing-parent");
        let nested = base.join("subdir").join("templates.json");
        let templates = vec![sample_template("1", "First")];

        let result = write_templates_to_path(&nested, &templates);
        assert!(result.is_err());
    }
}
