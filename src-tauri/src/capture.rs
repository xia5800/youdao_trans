use crate::constants;
use screenshots::Screen;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Emitter, Listener, Manager};

pub static FRONTEND_READY: AtomicBool = AtomicBool::new(false);

fn screenshot_file_path() -> std::path::PathBuf {
    std::env::temp_dir().join("youdao_screenshot.dat")
}

pub fn cleanup_screenshot_file() {
    let path = screenshot_file_path();
    if path.exists() {
        if let Err(e) = std::fs::remove_file(&path) {
            log::error!("清理截图临时文件失败: {}", e);
        }
    }
}

fn capture_screen_to_file() -> Result<(String, u32, u32), String> {
    let screen = crate::window::cursor_position()
        .and_then(|(cx, cy)| Screen::from_point(cx, cy).ok())
        .or_else(|| Screen::all().ok()?.into_iter().next())
        .ok_or_else(|| "无可用屏幕".to_string())?;
    let (mw, mh) = (screen.display_info.width as u32, screen.display_info.height as u32);

    let captured = screen.capture_area(0, 0, mw, mh)
        .map_err(|e| format!("截图失败: {}", e))?;

    let rgba = captured.into_raw();

    let path = screenshot_file_path();
    let path_str = path.to_string_lossy().to_string();

    let mut buf = Vec::with_capacity(8 + rgba.len());
    buf.extend_from_slice(&mw.to_le_bytes());
    buf.extend_from_slice(&mh.to_le_bytes());
    buf.extend_from_slice(&rgba);

    std::fs::write(&path, &buf)
        .map_err(|e| format!("写入临时文件失败: {}", e))?;

    Ok((path_str, mw, mh))
}

fn build_overlay_window(app: &tauri::AppHandle) -> Result<(), String> {
    tauri::WebviewWindowBuilder::new(
        app,
        constants::WINDOW_SCREENSHOT_OVERLAY,
        tauri::WebviewUrl::App(constants::SCREENSHOT_HTML.into()),
    )
    .always_on_top(true)
    .decorations(false)
    .transparent(true)
    .visible(false)
    .skip_taskbar(true)
    .build()
    .map_err(|e| format!("创建截图窗口失败: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn prepare_screenshot(app: tauri::AppHandle) -> Result<(), String> {
    if app.get_webview_window(constants::WINDOW_SCREENSHOT_OVERLAY).is_none() {
        build_overlay_window(&app)?;
        FRONTEND_READY.store(false, Ordering::SeqCst);
    }

    if !FRONTEND_READY.load(Ordering::SeqCst) {
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let tx = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));
        let tx_clone = tx.clone();
        let listener_id = app.listen(constants::EVENT_SCREENSHOT_READY, move |_event| {
            FRONTEND_READY.store(true, Ordering::SeqCst);
            if let Some(tx) = tx_clone.lock().unwrap().take() {
                let _ = tx.send(());
            }
        });

        let _ = tokio::time::timeout(std::time::Duration::from_secs(3), rx).await;
        app.unlisten(listener_id);
    }

    let overlay = app.get_webview_window(constants::WINDOW_SCREENSHOT_OVERLAY)
        .ok_or_else(|| "截图窗口不存在".to_string())?;

    let _ = overlay.hide();

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    match crate::window::cursor_position()
        .and_then(|(cx, cy)| Screen::from_point(cx, cy).ok())
    {
        Some(screen) => {
            let sx = screen.display_info.x as i32;
            let sy = screen.display_info.y as i32;
            let sw = screen.display_info.width as u32;
            let sh = screen.display_info.height as u32;
            if let Err(e) = overlay.set_position(tauri::PhysicalPosition::new(sx, sy)) {
                log::error!("设置截图覆盖层位置失败: {}", e);
            }
            if let Err(e) = overlay.set_size(tauri::PhysicalSize::new(sw, sh)) {
                log::error!("设置截图覆盖层大小失败: {}", e);
            }
            if let Err(e) = overlay.set_fullscreen(true) {
                log::error!("设置截图覆盖层全屏失败: {}", e);
            }
        }
        None => log::warn!("无法获取鼠标所在屏幕，截图覆盖层将留在默认位置"),
    }

    match capture_screen_to_file() {
        Ok((path, mw, mh)) => {
            let payload = serde_json::json!({ "path": path, "width": mw, "height": mh });
            let _ = overlay.emit(constants::EVENT_CAPTURE_SCREEN, payload);
        }
        Err(e) => {
            log::error!("[截图] 截图失败: {}", e);
            let _ = overlay.emit(constants::EVENT_CAPTURE_SCREEN, serde_json::json!({ "error": e }));
        }
    }

    Ok(())
}

#[tauri::command]
pub fn cancel_overlay(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(overlay) = app.get_webview_window(constants::WINDOW_SCREENSHOT_OVERLAY) {
        let _ = overlay.close();
    }
    Ok(())
}

#[tauri::command]
pub fn cleanup_screenshot() -> Result<(), String> {
    cleanup_screenshot_file();
    Ok(())
}
