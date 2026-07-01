use std::time::Duration;
use crate::constants;
use tauri::{Emitter, Manager, PhysicalPosition};

use crate::window::cursor_position;

fn init_clipboard() -> Result<arboard::Clipboard, String> {
    match arboard::Clipboard::new() {
        Ok(c) => Ok(c),
        Err(e) => {
            log::error!("初始化剪贴板失败: {}", e);
            Err(format!("clipboard init: {}", e))
        }
    }
}

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
        let mut clipboard = init_clipboard()?;

        let original = clipboard.get_text().ok();
        if let Err(e) = clipboard.clear() {
            log::error!("清空剪贴板失败: {}", e);
        }
        drop(clipboard);

        send_wm_copy();
        std::thread::sleep(Duration::from_millis(delay_ms));

        let mut clipboard = init_clipboard()?;

        if let Ok(t) = clipboard.get_text() {
            let trimmed = t.trim().to_string();
            if !trimmed.is_empty() {
                if let Some(orig) = original {
                    if let Err(e) = clipboard.set_text(orig) {
                        log::error!("恢复原始剪贴板内容失败: {}", e);
                    }
                }
                return Ok(trimmed);
            }
        }

        if let Err(e) = clipboard.clear() {
            log::error!("清空剪贴板失败: {}", e);
        }
        drop(clipboard);

        send_ctrl_c();
        std::thread::sleep(Duration::from_millis(delay_ms));

        let mut clipboard = init_clipboard()?;

        let text = clipboard.get_text().ok();
        if let Some(orig) = original {
            if let Err(e) = clipboard.set_text(orig) {
                log::error!("恢复原始剪贴板内容失败: {}", e);
            }
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
    pub programmer_mode: bool,
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
    bool,
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
        #[serde(default)]
        programmer_mode: bool,
    }
    fn default_true() -> bool { true }
    fn default_delay() -> u64 { constants::DEFAULT_DELAY_TIME_MS }

    if let Ok(c) = serde_json::from_str::<SelectionConfigRaw>(config_json) {
        (c.active_translator, c.translator_keys, c.store_records, c.delay_time, c.programmer_mode)
    } else {
        (None, std::collections::HashMap::new(), true, constants::DEFAULT_DELAY_TIME_MS, true)
    }
}

fn split_programmer_text(text: &str) -> String {
    let mut result = text.replace(|c: char| c == '_' || c == '-', " ");
    // Insert space at camelCase boundaries: aB → a B
    result = regex_for_camel(&result);
    // Insert space between Chinese and English
    result = regex_for_cjk_boundary(&result);
    // Collapse whitespace
    let mut out = String::with_capacity(result.len());
    let mut prev_space = false;
    for ch in result.chars() {
        if ch.is_whitespace() {
            if !prev_space {
                out.push(' ');
                prev_space = true;
            }
        } else {
            out.push(ch);
            prev_space = false;
        }
    }
    out.trim().to_string()
}

fn regex_for_camel(text: &str) -> String {
    let mut out = String::with_capacity(text.len() * 2);
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        out.push(chars[i]);
        if i + 1 < chars.len() {
            let cur = chars[i];
            let next = chars[i + 1];
            // aB or ABc → insert space
            if (cur.is_ascii_lowercase() && next.is_ascii_uppercase())
                || (i + 2 < chars.len()
                    && cur.is_ascii_uppercase()
                    && next.is_ascii_uppercase()
                    && chars[i + 2].is_ascii_lowercase())
            {
                out.push(' ');
            }
        }
        i += 1;
    }
    out
}

fn regex_for_cjk_boundary(text: &str) -> String {
    let mut out = String::with_capacity(text.len() * 2);
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        out.push(chars[i]);
        if i + 1 < chars.len() {
            let cur = chars[i];
            let next = chars[i + 1];
            let cur_is_cjk = is_cjk(cur);
            let next_is_ascii_letter = next.is_ascii_alphabetic();
            let cur_is_ascii_letter = cur.is_ascii_alphabetic();
            let next_is_cjk = is_cjk(next);
            if (cur_is_cjk && next_is_ascii_letter) || (cur_is_ascii_letter && next_is_cjk) {
                out.push(' ');
            }
        }
        i += 1;
    }
    out
}

fn is_cjk(c: char) -> bool {
    matches!(c, '\u{4E00}'..='\u{9FFF}' | '\u{3400}'..='\u{4DBF}')
}

fn load_config_any() -> Option<String> {
    crate::config::load().ok()
}

fn payload_for_window(app: &tauri::AppHandle, payload: &SelectionPayload) {
    if let Some(state) = app.try_state::<SelectionState>() {
        match state.0.lock() {
            Ok(mut s) => *s = Some(payload.clone()),
            Err(e) => log::error!("SelectionState 锁异常: {}", e),
        }
    }
    if let Some(popup) = app.get_webview_window(constants::WINDOW_SELECTION_POPUP) {
        if let Err(e) = popup.emit(constants::EVENT_SELECTION_UPDATE, payload) {
            log::error!("发送划词翻译数据到弹窗失败: {}", e);
        }
    }
}

fn show_popup_with_payload(app: &tauri::AppHandle, payload: &SelectionPayload, cx: i32, cy: i32) {
    if let Some(state) = app.try_state::<SelectionState>() {
        match state.0.lock() {
            Ok(mut s) => *s = Some(payload.clone()),
            Err(e) => log::error!("SelectionState 锁异常: {}", e),
        }
    }

    if let Some(existing) = app.get_webview_window(constants::WINDOW_SELECTION_POPUP) {
        if let Err(e) = existing.set_position(PhysicalPosition::new(cx.max(0) as i32, cy.max(0) as i32)) {
            log::error!("设置划词弹窗位置失败: {}", e);
        }
        if let Err(e) = existing.emit(constants::EVENT_SELECTION_UPDATE, payload) {
            log::error!("发送划词数据到弹窗失败: {}", e);
        }
    } else {
        match tauri::WebviewWindowBuilder::new(
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
            Ok(window) => {
                if let Err(e) = window.set_position(PhysicalPosition::new(cx.max(0) as i32, cy.max(0) as i32)) {
                    log::error!("设置划词弹窗位置失败: {}", e);
                }
            }
            Err(e) => log::error!("创建划词弹窗失败: {}", e),
        }
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
        log::error!("划词翻译错误: {}", msg);
        if let Err(e) = app.emit(constants::EVENT_SELECTION_TRANSLATE_ERROR, msg) {
            log::error!("发送划词翻译错误事件到前端失败: {}", e);
        }
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
        programmer_mode: false,
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

    let (active_translator, translator_keys, store_records, delay_time, programmer_mode) =
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
                programmer_mode: false,
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
        programmer_mode,
    };
    payload_for_window(&app, &translating_payload);
    if let Some(popup) = app.get_webview_window(constants::WINDOW_SELECTION_POPUP) {
        let _ = popup.set_focus();
    }

    let translate_text = if programmer_mode {
        split_programmer_text(&source_text)
    } else {
        source_text.clone()
    };

    let translated_text = match crate::translate::run(crate::translate::TranslateArgs {
        text: translate_text,
        source_lang: None,
        target_lang: target_lang.clone(),
        active_translator: active_translator.clone(),
        translator_keys: translator_keys.clone(),
    })
    .await
    {
        Ok(t) => t,
        Err(e) => {
            let failed = SelectionPayload {
                source_text,
                translated_text: format!("翻译失败: {}", e),
                target_lang,
                x: cx,
                y: cy,
                active_translator,
                translator_keys,
                is_translating: false,
                store_records,
                programmer_mode,
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
        programmer_mode,
    };
    payload_for_window(&app, &final_payload);
}
