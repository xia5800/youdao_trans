use std::time::Duration;
use crate::constants;
use tauri::{Emitter, Manager, PhysicalPosition};

use crate::window::cursor_position;

// ---- Clipboard logic (original) ----

#[link(name = "user32")]
extern "system" {
    fn GetForegroundWindow() -> isize;
    fn SendMessageW(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize;
    fn keybd_event(bVk: u8, bScan: u8, dwFlags: u32, dwExtraInfo: usize);
}

const WM_COPY: u32 = 0x0301;
const KEYEVENTF_KEYUP: u32 = 0x0002;
const VK_CONTROL: u8 = 0x11;
const VK_C: u8 = 0x43;

fn send_wm_copy() {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd != 0 {
            SendMessageW(hwnd, WM_COPY, 0, 0);
        }
    }
}

fn send_ctrl_c() {
    unsafe {
        keybd_event(0x12, 0, KEYEVENTF_KEYUP, 0); // Alt up
        keybd_event(0x10, 0, KEYEVENTF_KEYUP, 0); // Shift up
        keybd_event(VK_CONTROL, 0, KEYEVENTF_KEYUP, 0);
        std::thread::sleep(Duration::from_millis(constants::KEY_SIM_DELAY_MS));
        keybd_event(VK_CONTROL, 0, 0, 0);
        std::thread::sleep(Duration::from_millis(constants::KEY_SIM_DELAY_MS));
        keybd_event(VK_C, 0, 0, 0);
        std::thread::sleep(Duration::from_millis(constants::KEY_SIM_DELAY_MS));
        keybd_event(VK_C, 0, KEYEVENTF_KEYUP, 0);
        std::thread::sleep(Duration::from_millis(constants::KEY_SIM_DELAY_MS));
        keybd_event(VK_CONTROL, 0, KEYEVENTF_KEYUP, 0);
    }
}

pub fn get_selected_text(delay_ms: u64) -> Result<String, String> {
    for _attempt in 0..constants::SELECTION_RETRY_COUNT {
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| format!("clipboard init: {}", e))?;

        let original = clipboard.get_text().ok();
        clipboard.clear().ok();
        drop(clipboard);

        send_wm_copy();
        std::thread::sleep(Duration::from_millis(delay_ms));

        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| format!("clipboard init: {}", e))?;

        if let Ok(t) = clipboard.get_text() {
            let trimmed = t.trim().to_string();
            if !trimmed.is_empty() {
                if let Some(orig) = original {
                    let _ = clipboard.set_text(orig);
                }
                return Ok(trimmed);
            }
        }

        clipboard.clear().ok();
        drop(clipboard);

        send_ctrl_c();
        std::thread::sleep(Duration::from_millis(delay_ms));

        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| format!("clipboard init: {}", e))?;

        let text = clipboard.get_text().ok();
        if let Some(orig) = original {
            let _ = clipboard.set_text(orig);
        }

        if let Some(t) = text {
            let trimmed = t.trim().to_string();
            if !trimmed.is_empty() {
                return Ok(trimmed);
            }
        }
    }

    Err("取词失败: 无法读取选中文本，请确认已选中文本后重试".to_string())
}

// ---- Selection translate command data ----

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct SelectionPayload {
    pub source_text: String,
    pub translated_text: String,
    pub target_lang: String,
    pub x: i32,
    pub y: i32,
    pub active_translator: Option<String>,
    pub translator_keys: std::collections::HashMap<String, String>,
    pub is_translating: bool,
    pub store_records: bool,
}

pub struct SelectionState(pub std::sync::Mutex<Option<SelectionPayload>>);

// ---- Helpers ----

fn contains_chinese(text: &str) -> bool {
    text.chars().any(|c| c >= '\u{4E00}')
}

fn read_selection_config(
    config_json: &str,
) -> (
    Option<String>,
    std::collections::HashMap<String, String>,
    bool,
    u64,
) {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SelectionConfigRaw {
        #[serde(default)]
        active_translator: Option<String>,
        #[serde(default)]
        translator_keys: std::collections::HashMap<String, String>,
        #[serde(default = "default_true")]
        store_records: bool,
        #[serde(default = "default_delay")]
        delay_time: u64,
    }
    fn default_true() -> bool { true }
    fn default_delay() -> u64 { constants::DEFAULT_DELAY_TIME_MS }

    if let Ok(c) = serde_json::from_str::<SelectionConfigRaw>(config_json) {
        (c.active_translator, c.translator_keys, c.store_records, c.delay_time)
    } else {
        (None, std::collections::HashMap::new(), true, constants::DEFAULT_DELAY_TIME_MS)
    }
}

fn load_config_any() -> Option<String> {
    crate::config::load_effective().ok()
}

fn payload_for_window(app: &tauri::AppHandle, payload: &SelectionPayload) {
    if let Some(state) = app.try_state::<SelectionState>() {
        *state.0.lock().unwrap() = Some(payload.clone());
    }
    if let Some(popup) = app.get_webview_window(constants::WINDOW_SELECTION_POPUP) {
        let _ = popup.emit(constants::EVENT_SELECTION_UPDATE, payload);
    }
}

fn show_popup_with_payload(app: &tauri::AppHandle, payload: &SelectionPayload, cx: i32, cy: i32) {
    if let Some(state) = app.try_state::<SelectionState>() {
        *state.0.lock().unwrap() = Some(payload.clone());
    }

    if let Some(existing) = app.get_webview_window(constants::WINDOW_SELECTION_POPUP) {
        let _ = existing.set_position(PhysicalPosition::new(cx.max(0) as i32, cy.max(0) as i32));
        let _ = existing.emit(constants::EVENT_SELECTION_UPDATE, payload);
    } else if let Ok(window) = tauri::WebviewWindowBuilder::new(
        app,
        constants::WINDOW_SELECTION_POPUP,
        tauri::WebviewUrl::App(constants::POPUP_HTML.into()),
    )
    .always_on_top(true)
    .decorations(false)
    .transparent(true)
    .visible(false)
    .skip_taskbar(true)
    .inner_size(400.0, 240.0)
    .resizable(false)
    .focused(false)
    .build()
    {
        let _ = window.set_position(PhysicalPosition::new(cx.max(0) as i32, cy.max(0) as i32));
    }
}

// ---- Commands ----

#[tauri::command]
pub fn get_selection_payload(
    state: tauri::State<'_, SelectionState>,
) -> Result<Option<SelectionPayload>, String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())
        .map(|mut g| g.take())
}

// ---- Main handler ----

pub async fn handle_selection_translate(app: tauri::AppHandle) {
    fn emit_err(app: &tauri::AppHandle, msg: &str) {
        let _ = app.emit(constants::EVENT_SELECTION_TRANSLATE_ERROR, msg);
    }

    let pos = cursor_position();
    let (cx, cy) = match pos {
        Some(p) => p,
        None => {
            emit_err(&app, "获取鼠标位置失败");
            return;
        }
    };

    let loading_payload = SelectionPayload {
        source_text: String::new(),
        translated_text: String::new(),
        target_lang: "zh".to_string(),
        x: cx,
        y: cy,
        active_translator: None,
        translator_keys: std::collections::HashMap::new(),
        is_translating: true,
        store_records: false,
    };
    show_popup_with_payload(&app, &loading_payload, cx, cy);

    let config_json = match load_config_any() {
        Some(j) => j,
        None => {
            if let Some(popup) = app.get_webview_window(constants::WINDOW_SELECTION_POPUP) {
                let _ = popup.close();
            }
            return;
        }
    };

    let (active_translator, translator_keys, store_records, delay_time) =
        read_selection_config(&config_json);

    let source_text = match crate::selection::get_selected_text(delay_time) {
        Ok(t) => t,
        Err(e) => {
            emit_err(&app, &format!("取词失败: {}", e));
            let error_payload = SelectionPayload {
                source_text: String::new(),
                translated_text: format!("取词失败: {}", e),
                target_lang: "zh".to_string(),
                x: cx,
                y: cy,
                active_translator: None,
                translator_keys: std::collections::HashMap::new(),
                is_translating: false,
                store_records: false,
            };
            payload_for_window(&app, &error_payload);
            if let Some(popup) = app.get_webview_window(constants::WINDOW_SELECTION_POPUP) {
                let _ = popup.set_focus();
            }
            return;
        }
    };

    let target_lang = if contains_chinese(&source_text) {
        "en".to_string()
    } else {
        "zh".to_string()
    };

    let translating_payload = SelectionPayload {
        source_text: source_text.clone(),
        translated_text: String::new(),
        target_lang: target_lang.clone(),
        x: cx,
        y: cy,
        active_translator: active_translator.clone(),
        translator_keys: translator_keys.clone(),
        is_translating: true,
        store_records,
    };
    payload_for_window(&app, &translating_payload);
    if let Some(popup) = app.get_webview_window(constants::WINDOW_SELECTION_POPUP) {
        let _ = popup.set_focus();
    }

    let translated_text = match crate::translate::run(crate::translate::TranslateArgs {
        text: source_text.clone(),
        source_lang: None,
        target_lang: target_lang.clone(),
        active_translator: active_translator.clone(),
        translator_keys: translator_keys.clone(),
    })
    .await
    {
        Ok(t) => t,
        Err(_) => {
            let failed = SelectionPayload {
                source_text,
                translated_text: "翻译失败".to_string(),
                target_lang,
                x: cx,
                y: cy,
                active_translator,
                translator_keys,
                is_translating: false,
                store_records,
            };
            payload_for_window(&app, &failed);
            return;
        }
    };

    let final_payload = SelectionPayload {
        source_text,
        translated_text,
        target_lang,
        x: cx,
        y: cy,
        active_translator,
        translator_keys,
        is_translating: false,
        store_records,
    };
    payload_for_window(&app, &final_payload);
}
