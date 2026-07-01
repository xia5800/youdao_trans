mod baidu;
mod ollama;
mod paddle;
mod tencent;
mod xunfei;

use crate::make_registry;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use crate::constants;
use tauri::{Emitter, Manager};

type OcrFn = fn(
    String,
    HashMap<String, String>,
) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;

make_registry!(OcrFn, registry, [
    ("baidu_ocr", |img: String, keys: HashMap<String, String>| Box::pin(async move { baidu::ocr(&img, &keys).await })),
    ("tencent", |img: String, keys: HashMap<String, String>| Box::pin(async move { tencent::ocr(&img, &keys).await })),
    ("xunfei", |img: String, keys: HashMap<String, String>| Box::pin(async move { xunfei::ocr(&img, &keys).await })),
    ("ollama_ocr", |img: String, keys: HashMap<String, String>| Box::pin(async move { ollama::ocr(&img, &keys).await })),
    ("paddle_ocr", |img: String, keys: HashMap<String, String>| Box::pin(async move { paddle::ocr(&img, &keys).await })),
]);

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
    let config_json: String = match crate::config::load() {
        Ok(j) => j,
        Err(e) => {
            log::error!("加载OCR配置失败: {}", e);
            return Ok(String::new());
        }
    };
    let config_val: serde_json::Value = match serde_json::from_str(&config_json) {
        Ok(v) => v,
        Err(e) => {
            log::error!("解析OCR配置JSON失败: {}", e);
            return Ok(String::new());
        }
    };

    let active_ocr = match config_val[constants::CFG_ACTIVE_OCR].as_str() {
        Some(p) => p,
        None => {
            log::warn!("未配置活动OCR引擎");
            return Ok(String::new());
        }
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
        if let Err(e) = app.emit(constants::EVENT_OCR_RESULT, &text) {
            log::error!("发送OCR结果到前端失败: {}", e);
        }
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
        .map_err(|e| {
            log::error!("PendingOcrState 锁异常: {}", e);
            e.to_string()
        })
        .map(|mut g| g.take())
}

#[tauri::command]
pub async fn finish_ocr_screenshot(
    base64_img: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    if let Some(state) = app.try_state::<PendingOcrState>() {
        match state.0.lock() {
            Ok(mut s) => *s = Some(base64_img),
            Err(e) => log::error!("PendingOcrState 锁异常: {}", e),
        }
    }
    crate::window::show_main(&app);
    if let Err(e) = app.emit(constants::EVENT_NAVIGATE, constants::ROUTE_TRANSLATE) {
        log::error!("通知前端导航至翻译页面失败: {}", e);
    }
    if let Err(e) = app.emit(constants::EVENT_CHECK_PENDING_OCR, "") {
        log::error!("通知前端检查待处理OCR失败: {}", e);
    }
    Ok(())
}

#[tauri::command]
pub fn unload_ocr_engine() {
    paddle::unload_engine();
}

#[tauri::command]
pub fn check_ocr_models_state() -> std::collections::HashMap<String, bool> {
    paddle::check_model_files()
}

#[tauri::command]
pub fn ocr_models_data_dir() -> String {
    let dirs = paddle::candidate_dirs();
    let found = dirs.into_iter().find(|d| {
        d.join(crate::constants::OCR_DET_MODEL).exists()
            && d.join(crate::constants::OCR_REC_MODEL).exists()
            && d.join(crate::constants::OCR_DICT_FILE).exists()
    });
    if let Some(d) = found {
        return d.to_string_lossy().to_string();
    }
    crate::config::default_models_dir_inner().join("ocr").join(crate::constants::OCR_MODEL_DIR).to_string_lossy().to_string()
}

#[tauri::command]
pub async fn download_ocr_models(
    app: tauri::AppHandle,
    use_mirror: bool,
    use_github_mirror: bool,
) -> Result<(), String> {
    let specs = paddle::download_specs();
    crate::download::download_files(&app, &specs, use_mirror, use_github_mirror).await
}

#[tauri::command]
pub async fn retry_download_ocr_file(
    app: tauri::AppHandle,
    file_name: String,
    use_mirror: bool,
    use_github_mirror: bool,
) -> Result<(), String> {
    let specs = paddle::download_specs();
    let spec = specs
        .into_iter()
        .find(|s| s.file_name == file_name)
        .ok_or_else(|| format!("未知文件: {}", file_name))?;
    crate::download::retry_download_file(&app, spec, use_mirror, use_github_mirror).await
}
