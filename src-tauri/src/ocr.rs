pub mod baidu;
pub mod tencent;
pub mod xunfei;
pub mod ollama;
pub mod paddle;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::OnceLock;
use crate::constants;
use tauri::{Emitter, Manager};

type OcrFn = fn(
    String,
    HashMap<String, String>,
) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;

fn registry() -> &'static HashMap<&'static str, OcrFn> {
    static REG: OnceLock<HashMap<&'static str, OcrFn>> = OnceLock::new();
    REG.get_or_init(|| {
        let mut m: HashMap<&'static str, OcrFn> = HashMap::new();
        m.insert("baidu_ocr", |img, keys| Box::pin(async move { baidu::ocr(&img, &keys).await }));
        m.insert("tencent", |img, keys| Box::pin(async move { tencent::ocr(&img, &keys).await }));
        m.insert("xunfei", |img, keys| Box::pin(async move { xunfei::ocr(&img, &keys).await }));
        m.insert("ollama_ocr", |img, keys| Box::pin(async move { ollama::ocr(&img, &keys).await }));
        m.insert("paddle_ocr", |img, keys| Box::pin(async move { paddle::ocr(&img, &keys).await }));
        m
    })
}

pub async fn run(
    provider: &str,
    base64_img: &str,
    keys: &HashMap<String, String>,
) -> Result<String, String> {
    match registry().get(provider) {
        Some(f) => f(base64_img.to_string(), keys.clone()).await,
        None => Err(format!("OCR provider '{}' not supported", provider)),
    }
}

// ---- Command data ----

pub struct PendingOcrState(pub std::sync::Mutex<Option<String>>);

// ---- Commands ----

#[tauri::command]
pub async fn ocr_command(base64_img: String, app: tauri::AppHandle) -> Result<String, String> {
    let config_json = match crate::config::load_effective() {
        Ok(j) => j,
        Err(_) => return Ok(String::new()),
    };
    let config_val: serde_json::Value = match serde_json::from_str(&config_json) {
        Ok(v) => v,
        Err(_) => return Ok(String::new()),
    };

    let active_ocr = match config_val[constants::CFG_ACTIVE_OCR].as_str() {
        Some(p) => p,
        None => return Ok(String::new()),
    };

    let mut ocr_keys = HashMap::new();
    if let Some(keys) = config_val[constants::CFG_OCR_KEYS].as_object() {
        for (k, v) in keys {
            if let Some(s) = v.as_str() {
                ocr_keys.insert(k.clone(), s.to_string());
            }
        }
    }

    let text = run(active_ocr, &base64_img, &ocr_keys).await.map_err(|e| {
        log::error!("OCR 识别失败 [provider={}]: {}", active_ocr, e);
        let _ = app.emit(constants::EVENT_OCR_ERROR, &e);
        e
    })?;

    if !text.is_empty() {
        let _ = app.emit(constants::EVENT_OCR_RESULT, &text);
    }

    Ok(text)
}

#[tauri::command]
pub fn take_pending_ocr(
    state: tauri::State<'_, PendingOcrState>,
) -> Result<Option<String>, String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())
        .map(|mut g| g.take())
}

#[tauri::command]
pub async fn finish_ocr_screenshot(
    base64_img: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    if let Some(state) = app.try_state::<PendingOcrState>() {
        *state.0.lock().map_err(|e| e.to_string())? = Some(base64_img);
    }
    crate::window::show_main(&app);
    let _ = app.emit(constants::EVENT_NAVIGATE, constants::ROUTE_TRANSLATE);
    let _ = app.emit(constants::EVENT_CHECK_PENDING_OCR, "");
    Ok(())
}
