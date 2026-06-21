use crate::constants;
use tauri::Manager;

pub fn show_main(app: &tauri::AppHandle) {
    if let Some(main) = app.get_webview_window(constants::WINDOW_MAIN) {
        let _ = main.show();
        let _ = main.unminimize();
        let _ = main.set_focus();
    }
}

#[tauri::command]
pub fn set_window_bg(app: tauri::AppHandle, r: u8, g: u8, b: u8) -> Result<(), String> {
    app.get_webview_window(constants::WINDOW_MAIN)
        .ok_or("main window not found")?
        .set_background_color(Some(tauri::webview::Color(r, g, b, 255)))
        .map_err(|e| e.to_string())
}

pub fn cursor_position() -> Option<(i32, i32)> {
    match mouse_position::mouse_position::Mouse::get_mouse_position() {
        mouse_position::mouse_position::Mouse::Position { x, y } => Some((x, y)),
        _ => None,
    }
}

#[tauri::command]
pub fn minimize_window(app: tauri::AppHandle) -> Result<(), String> {
    app.get_webview_window(constants::WINDOW_MAIN)
        .ok_or("main window not found")?
        .minimize()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn maximize_window(app: tauri::AppHandle) -> Result<bool, String> {
    let w = app
        .get_webview_window(constants::WINDOW_MAIN)
        .ok_or("main window not found")?;
    let maximized = w.is_maximized().unwrap_or(false);
    if maximized {
        w.unmaximize().map_err(|e| e.to_string())?;
    } else {
        w.maximize().map_err(|e| e.to_string())?;
    }
    Ok(!maximized)
}

#[tauri::command]
pub fn close_window(app: tauri::AppHandle) -> Result<(), String> {
    app.get_webview_window(constants::WINDOW_MAIN)
        .ok_or("main window not found")?
        .close()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pin_window(app: tauri::AppHandle) -> Result<bool, String> {
    let w = app
        .get_webview_window(constants::WINDOW_MAIN)
        .ok_or("main window not found")?;
    let is_pinned = w.is_always_on_top().unwrap_or(false);
    w.set_always_on_top(!is_pinned)
        .map_err(|e| e.to_string())?;
    Ok(!is_pinned)
}

#[tauri::command]
pub fn is_window_pinned(app: tauri::AppHandle) -> Result<bool, String> {
    app.get_webview_window(constants::WINDOW_MAIN)
        .ok_or("main window not found")?
        .is_always_on_top()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn is_window_maximized(app: tauri::AppHandle) -> Result<bool, String> {
    app.get_webview_window(constants::WINDOW_MAIN)
        .ok_or("main window not found")?
        .is_maximized()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn hide_window(app: tauri::AppHandle, label: String) -> Result<(), String> {
    app.get_webview_window(&label)
        .ok_or(format!("window '{}' not found", &label))?
        .hide()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn app_exit(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
pub fn show_window(app: tauri::AppHandle, label: String) -> Result<(), String> {
    let w = app
        .get_webview_window(&label)
        .ok_or(format!("window '{}' not found", &label))?;
    w.unminimize().map_err(|e| e.to_string())?;
    w.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}
