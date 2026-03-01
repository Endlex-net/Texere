use tauri::AppHandle;
use tauri_plugin_store::Store;

/// Send a notification to the user
#[cfg(target_os = "macos")]
pub fn send_notification(app: &AppHandle, title: &str, body: &str) {
    use tauri_plugin_notification::NotificationExt;

    let _ = app.notification().builder().title(title).body(body).show();
}

#[cfg(not(target_os = "macos"))]
pub fn send_notification(_app: &AppHandle, title: &str, body: &str) {
    log::debug!(
        "Notifications are disabled on this platform - title='{}' body='{}'",
        title,
        body
    );
}

/// Check if a one-time notification has been shown
pub fn has_shown_notification(store: &Store<tauri::Wry>, key: &str) -> bool {
    store.get(key).and_then(|v| v.as_bool()).unwrap_or(false)
}

/// Mark a one-time notification as shown
pub fn mark_notification_shown(store: &Store<tauri::Wry>, key: &str) -> Result<(), String> {
    store.set(key, serde_json::json!(true));
    store.save().map_err(|e| e.to_string())
}

/// Send accessibility permission notification if not shown before
pub fn send_accessibility_notification_once(app: &AppHandle, store: &Store<tauri::Wry>) {
    const KEY: &str = "notification_accessibility_shown";

    if !has_shown_notification(store, KEY) {
        send_notification(
            app,
            "Texere: Accessibility Permission Required",
            "Go to System Settings > Privacy & Security > Accessibility and grant Texere access for cursor positioning."
        );
        let _ = mark_notification_shown(store, KEY);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_notification_key_format() {
        const KEY: &str = "notification_accessibility_shown";
        assert!(KEY.starts_with("notification_"));
    }

    #[test]
    fn test_notification_title_format() {
        let title = "Texere: Accessibility Permission Required";
        assert!(title.starts_with("Texere:"));
        assert!(title.len() > 10);
    }

    #[test]
    fn test_notification_body_contains_guidance() {
        let body = "Go to System Settings > Privacy & Security > Accessibility and grant Texere access for cursor positioning.";
        assert!(body.contains("System Settings"));
        assert!(body.contains("Accessibility"));
    }

    #[test]
    fn test_store_key_naming_convention() {
        let key = "notification_accessibility_shown";
        assert!(key.starts_with("notification_"));
        assert!(key.ends_with("_shown"));
    }
}
