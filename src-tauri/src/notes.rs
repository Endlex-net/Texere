use crate::types::Note;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

/// Serializes all notes read-modify-write operations to prevent concurrent-save data loss (C5).
pub struct NotesLock(pub std::sync::Mutex<()>);

fn notes_path(app: &AppHandle) -> Result<PathBuf, String> {
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

    Ok(app_data_dir.join("notes.json"))
}

fn read_notes_from_path(path: &Path) -> Vec<Note> {
    let content = match std::fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => return Vec::new(), // File missing — not corrupt
    };
    match serde_json::from_str::<Vec<Note>>(&content) {
        Ok(notes) => notes,
        Err(e) => {
            eprintln!("Failed to parse notes.json: {}", e);
            // Preserve the corrupt file so the user can recover it (M6)
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let corrupt_path =
                path.with_file_name(format!("notes.json.corrupt-{}", timestamp));
            if let Err(rename_err) = std::fs::rename(path, &corrupt_path) {
                eprintln!("Failed to rename corrupt notes.json: {}", rename_err);
            } else {
                eprintln!(
                    "Corrupt notes.json preserved as {}",
                    corrupt_path.display()
                );
            }
            Vec::new()
        }
    }
}

/// Load notes from disk. Returns empty Vec if the file is missing.
/// If the file exists but is corrupt, renames it and returns empty Vec.
pub fn load_notes(app: &AppHandle) -> Vec<Note> {
    let path = match notes_path(app) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("load_notes path error: {}", e);
            return Vec::new();
        }
    };
    read_notes_from_path(&path)
}

fn write_notes_to_path(path: &Path, notes: &[Note]) -> Result<(), String> {
    let tmp_path = path.with_extension("json.tmp");
    let payload = serde_json::to_string_pretty(notes)
        .map_err(|e| format!("serialize notes failed: {}", e))?;

    std::fs::write(&tmp_path, &payload).map_err(|e| {
        format!(
            "write temp notes failed ({}): {}",
            tmp_path.display(),
            e
        )
    })?;

    // POSIX rename is atomic and replaces the destination file in one syscall.
    // Do NOT call remove_file first — that would create a window where the file
    // doesn't exist, breaking atomicity (C4).
    std::fs::rename(&tmp_path, path).map_err(|e| {
        format!(
            "rename temp notes failed ({} -> {}): {}",
            tmp_path.display(),
            path.display(),
            e
        )
    })
}

fn write_notes(app: &AppHandle, notes: &[Note]) -> Result<(), String> {
    let path = notes_path(app)?;
    write_notes_to_path(&path, notes)
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[tauri::command]
pub fn get_notes(app: AppHandle) -> Vec<Note> {
    load_notes(&app)
}

/// Upsert a note. If `note.id` is empty, generates a new UUID and creates the record.
/// Returns the saved note (with a generated ID if newly created).
///
/// `window_label`: when Some, atomically registers the note→window mapping in
/// `NoteWindowMap` **before** rebuilding the tray, preventing the race where a user
/// clicks the newly-visible tray entry before `bind_note_window` fires (C3).
#[tauri::command]
pub fn save_note(
    app: AppHandle,
    note: Note,
    window_label: Option<String>,
) -> Result<Note, String> {
    // Serialize all RMW operations to prevent concurrent saves from clobbering each other (C5)
    let notes_lock = app.state::<NotesLock>();
    let _guard = notes_lock
        .0
        .lock()
        .map_err(|_| "notes write lock poisoned")?;

    let mut notes = load_notes(&app);
    let now = now_ms();

    // Check for duplicate name (excluding the current note by id)
    let name_lower = note.name.to_lowercase();
    let duplicate = notes
        .iter()
        .any(|n| n.name.to_lowercase() == name_lower && n.id != note.id);
    if duplicate {
        // Structured prefix so the frontend can match reliably without substring heuristics (M1)
        return Err(format!(
            "DUPLICATE_NAME: A note named '{}' already exists",
            note.name
        ));
    }

    // Determine whether the tray needs rebuilding.
    // Skip rebuild for content-only autosaves to avoid tray flicker every 500 ms (M5).
    let should_rebuild_tray = if note.id.is_empty() {
        true // New note → always rebuild
    } else {
        notes
            .iter()
            .find(|n| n.id == note.id)
            .map(|n| n.name != note.name)
            .unwrap_or(true) // Unknown ID treated as new
    };

    let saved_note = if note.id.is_empty() {
        // Create new note
        let new_note = Note {
            id: Uuid::new_v4().to_string(),
            name: note.name,
            content: note.content,
            created_at: now,
            updated_at: now,
        };
        notes.push(new_note.clone());
        new_note
    } else if let Some(existing) = notes.iter_mut().find(|n| n.id == note.id) {
        // Update existing note
        existing.name = note.name;
        existing.content = note.content;
        existing.updated_at = now;
        existing.clone()
    } else {
        // Unknown ID — create with the given ID
        let new_note = Note {
            id: note.id,
            name: note.name,
            content: note.content,
            created_at: now,
            updated_at: now,
        };
        notes.push(new_note.clone());
        new_note
    };

    write_notes(&app, &notes)?;

    // Atomically bind window→note mapping BEFORE rebuilding the tray.
    // This closes the race where the tray entry becomes visible before bind_note_window
    // fires from the frontend (C3).
    if let Some(label) = window_label {
        if let Some(map) = app.try_state::<crate::window::NoteWindowMap>() {
            if let Ok(mut lock) = map.lock() {
                lock.insert(saved_note.id.clone(), label);
            }
        }
    }

    if should_rebuild_tray {
        let _ = crate::tray::rebuild_tray(&app, &notes);
    }

    Ok(saved_note)
}

#[tauri::command]
pub fn delete_note(app: AppHandle, id: String) -> Result<(), String> {
    // Serialize RMW (C5)
    let notes_lock = app.state::<NotesLock>();
    let _guard = notes_lock
        .0
        .lock()
        .map_err(|_| "notes write lock poisoned")?;

    let mut notes = load_notes(&app);
    let original_len = notes.len();
    notes.retain(|n| n.id != id);

    if notes.len() == original_len {
        return Err(format!("note not found: {}", id));
    }

    write_notes(&app, &notes)?;

    // Remove from NoteWindowMap (window stays open but is now "unnamed")
    if let Some(map) = app.try_state::<crate::window::NoteWindowMap>() {
        if let Ok(mut lock) = map.lock() {
            lock.remove(&id);
        }
    }

    // Rebuild tray menu
    let _ = crate::tray::rebuild_tray(&app, &notes);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_note(id: &str, name: &str) -> Note {
        Note {
            id: id.to_string(),
            name: name.to_string(),
            content: format!("{} content", name),
            created_at: 1,
            updated_at: 1,
        }
    }

    fn temp_path(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "texere-notes-{}-{}.json",
            label,
            Uuid::new_v4()
        ))
    }

    #[test]
    fn read_missing_file_returns_empty() {
        let p = temp_path("missing");
        assert_eq!(read_notes_from_path(&p), Vec::<Note>::new());
    }

    #[test]
    fn read_corrupt_file_returns_empty() {
        let p = temp_path("corrupt");
        std::fs::write(&p, b"not json").unwrap();
        assert_eq!(read_notes_from_path(&p), Vec::<Note>::new());
        // The corrupt file is renamed; original path no longer exists — remove_file is a no-op
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn write_and_read_roundtrip() {
        let p = temp_path("roundtrip");
        let notes = vec![sample_note("1", "Alpha"), sample_note("2", "Beta")];
        write_notes_to_path(&p, &notes).unwrap();
        let loaded = read_notes_from_path(&p);
        assert_eq!(loaded, notes);
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn write_to_missing_parent_returns_error() {
        let p = std::path::PathBuf::from("/nonexistent/deeply/nested/notes.json");
        let notes = vec![sample_note("1", "Test")];
        assert!(write_notes_to_path(&p, &notes).is_err());
    }
}
