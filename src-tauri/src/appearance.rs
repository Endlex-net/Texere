use tauri::{Runtime, WebviewWindow};

/// Applies vibrancy to the given window if supported by the platform.
pub fn apply_vibrancy<R: Runtime>(window: &WebviewWindow<R>) {
    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
        
        if let Err(e) = apply_vibrancy(
            window, 
            NSVisualEffectMaterial::Sidebar, 
            None, 
            None
        ) {
            eprintln!("Failed to apply macOS vibrancy: {}", e);
        }
    }

    #[cfg(target_os = "windows")]
    {
        use window_vibrancy::apply_blur;
        
        if let Err(e) = apply_blur(window, Some((18, 18, 18, 125))) {
            eprintln!("Failed to apply Windows blur effect: {}", e);
        }
    }
}
