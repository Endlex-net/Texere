#[cfg(target_os = "macos")]
mod macos {
    use accessibility_sys::{
        kAXBoundsForRangeParameterizedAttribute, kAXFocusedApplicationAttribute,
        kAXFocusedUIElementAttribute, kAXSelectedTextRangeAttribute, kAXValueTypeCGRect,
        AXUIElementCopyAttributeValue, AXUIElementCopyParameterizedAttributeValue,
        AXUIElementCreateSystemWide, AXValueGetValue,
    };
    use core_foundation::base::{CFRelease, CFTypeRef, TCFType};
    use core_foundation::string::CFString;
    use std::ptr;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGPoint {
        x: f64,
        y: f64,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGSize {
        width: f64,
        height: f64,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGRect {
        origin: CGPoint,
        size: CGSize,
    }

    #[link(name = "ApplicationServices", kind = "framework")]
    unsafe extern "C" {
        fn CGMainDisplayID() -> u32;
        fn CGDisplayBounds(display: u32) -> CGRect;
        fn CGEventCreate(source: *const std::ffi::c_void) -> *mut std::ffi::c_void;
        fn CGEventGetLocation(event: *mut std::ffi::c_void) -> CGPoint;
    }

    fn release_if_non_null(value: CFTypeRef) {
        if !value.is_null() {
            unsafe { CFRelease(value) };
        }
    }

    pub fn get_cursor_screen_position() -> Option<(f64, f64)> {
        unsafe {
            let system = AXUIElementCreateSystemWide();
            if system.is_null() {
                return None;
            }

            let focused_app_attr = CFString::from_static_string(kAXFocusedApplicationAttribute);
            let focused_ui_attr = CFString::from_static_string(kAXFocusedUIElementAttribute);
            let selected_range_attr = CFString::from_static_string(kAXSelectedTextRangeAttribute);
            let bounds_for_range_attr =
                CFString::from_static_string(kAXBoundsForRangeParameterizedAttribute);

            let mut focused_app: CFTypeRef = ptr::null_mut();
            if AXUIElementCopyAttributeValue(
                system,
                focused_app_attr.as_concrete_TypeRef(),
                &mut focused_app,
            ) != 0
                || focused_app.is_null()
            {
                release_if_non_null(system as CFTypeRef);
                return None;
            }

            let mut focused_ui: CFTypeRef = ptr::null_mut();
            if AXUIElementCopyAttributeValue(
                focused_app as _,
                focused_ui_attr.as_concrete_TypeRef(),
                &mut focused_ui,
            ) != 0
                || focused_ui.is_null()
            {
                release_if_non_null(focused_app);
                release_if_non_null(system as CFTypeRef);
                return None;
            }

            let mut selected_range: CFTypeRef = ptr::null_mut();
            if AXUIElementCopyAttributeValue(
                focused_ui as _,
                selected_range_attr.as_concrete_TypeRef(),
                &mut selected_range,
            ) != 0
                || selected_range.is_null()
            {
                release_if_non_null(focused_ui);
                release_if_non_null(focused_app);
                release_if_non_null(system as CFTypeRef);
                return None;
            }

            let mut bounds_value: CFTypeRef = ptr::null_mut();
            if AXUIElementCopyParameterizedAttributeValue(
                focused_ui as _,
                bounds_for_range_attr.as_concrete_TypeRef(),
                selected_range,
                &mut bounds_value,
            ) != 0
                || bounds_value.is_null()
            {
                release_if_non_null(selected_range);
                release_if_non_null(focused_ui);
                release_if_non_null(focused_app);
                release_if_non_null(system as CFTypeRef);
                return None;
            }

            let mut rect = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize {
                    width: 0.0,
                    height: 0.0,
                },
            };

            let ok = AXValueGetValue(
                bounds_value as _,
                kAXValueTypeCGRect,
                &mut rect as *mut CGRect as *mut _,
            );

            release_if_non_null(bounds_value);
            release_if_non_null(selected_range);
            release_if_non_null(focused_ui);
            release_if_non_null(focused_app);
            release_if_non_null(system as CFTypeRef);

            if !ok {
                return None;
            }

            let x = rect.origin.x;
            let y = rect.origin.y;
            Some((x, y))
        }
    }

    pub fn get_mouse_screen_position() -> Option<(f64, f64)> {
        unsafe {
            let event = CGEventCreate(ptr::null());
            if event.is_null() {
                return None;
            }

            let point = CGEventGetLocation(event);
            release_if_non_null(event as CFTypeRef);

            Some((point.x, point.y))
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_cursor_screen_position() -> Option<(f64, f64)> {
    macos::get_cursor_screen_position()
}

#[cfg(target_os = "macos")]
pub fn get_mouse_screen_position() -> Option<(f64, f64)> {
    macos::get_mouse_screen_position()
}

#[cfg(not(target_os = "macos"))]
pub fn get_cursor_screen_position() -> Option<(f64, f64)> {
    None
}

#[cfg(not(target_os = "macos"))]
pub fn get_mouse_screen_position() -> Option<(f64, f64)> {
    None
}

#[tauri::command]
pub fn get_cursor_screen_position_command() -> Option<(f64, f64)> {
    get_cursor_screen_position()
}
