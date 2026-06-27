use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use base64::Engine;
use oar_ocr::prelude::*;
use oar_ocr::domain::tasks::TextDetectionConfig;

const CARGO_DIR: &str = env!("CARGO_MANIFEST_DIR");

struct OcrEngine {
    engine: OAROCR,
}

fn models_dir() -> PathBuf {
    // 1) Portable mode: relative to the executable's directory
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let p = exe_dir.join("models").join("ocr");
            if p.exists() {
                return p;
            }
        }
    }

    // 2) Development: relative to CARGO_MANIFEST_DIR (project/src-tauri)
    let from_manifest = {
        let mut p = PathBuf::from(CARGO_DIR);
        p.pop();
        p.join("models").join("ocr")
    };
    if from_manifest.exists() {
        return from_manifest;
    }

    // 3) Fallback: current working directory
    let cwd = std::env::current_dir()
        .unwrap_or_default()
        .join("models").join("ocr");
    if cwd.exists() {
        return cwd;
    }

    from_manifest
}

static ENGINE: OnceLock<Mutex<Option<OcrEngine>>> = OnceLock::new();

fn init_engine() -> Result<OcrEngine, String> {
    let dir = models_dir();
    let det_path = dir.join("PP-OCRv6_medium_det.onnx");
    let rec_path = dir.join("PP-OCRv6_medium_rec.onnx");
    let dict_path = dir.join("ppocrv6_dict.txt");

    if !det_path.exists() {
        return Err(format!("检测模型不存在: {}", det_path.display()));
    }
    if !rec_path.exists() {
        return Err(format!("识别模型不存在: {}", rec_path.display()));
    }
    if !dict_path.exists() {
        return Err(format!("字典文件不存在: {}", dict_path.display()));
    }

    log::info!("初始化 PaddleOCR 引擎");
    log::info!("  检测模型: {}", det_path.display());
    log::info!("  识别模型: {}", rec_path.display());
    log::info!("  字典文件: {}", dict_path.display());

    let engine = OAROCRBuilder::new(
        det_path.to_str().ok_or("检测模型路径无效")?,
        rec_path.to_str().ok_or("识别模型路径无效")?,
        dict_path.to_str().ok_or("字典文件路径无效")?,
    )
    .text_detection_config(TextDetectionConfig {
        score_threshold: 0.3,
        box_threshold: 0.5,
        unclip_ratio: 1.5,
        ..Default::default()
    })
    .build()
    .map_err(|e| format!("PaddleOCR 引擎初始化失败: {}", e))?;

    log::info!("PaddleOCR 引擎初始化成功");

    Ok(OcrEngine { engine })
}

fn get_engine() -> Result<&'static Mutex<Option<OcrEngine>>, String> {
    let cell = ENGINE.get_or_init(|| {
        Mutex::new(Some(match init_engine() {
            Ok(e) => e,
            Err(e) => {
                log::error!("PaddleOCR 引擎初始化失败: {}", e);
                return Mutex::new(None);
            }
        }))
    });

    let guard = cell.lock().map_err(|e| format!("引擎锁异常: {}", e))?;
    if guard.is_none() {
        return Err("PaddleOCR 引擎未初始化".to_string());
    }
    drop(guard);

    Ok(cell)
}

pub async fn ocr(base64_img: &str, _keys: &HashMap<String, String>) -> Result<String, String> {
    let cell = get_engine()?;

    let img_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_img)
        .map_err(|e| format!("Base64 解码失败: {}", e))?;

    let img = image::load_from_memory(&img_bytes)
        .map_err(|e| format!("图片解码失败: {}", e))?;

    let rgb_img = img.to_rgb8();

    let guard = cell.lock().map_err(|e| format!("引擎锁异常: {}", e))?;
    let eng = guard.as_ref().ok_or("PaddleOCR 引擎未初始化")?;

    let results = eng.engine
        .predict(vec![rgb_img])
        .map_err(|e| format!("PaddleOCR 识别失败: {}", e))?;

    drop(guard);

    let mut all_texts = Vec::new();

    for result in &results {
        for region in &result.text_regions {
            if let Some((text, _confidence)) = region.text_with_confidence() {
                if !text.is_empty() {
                    all_texts.push(text);
                }
            }
        }
    }

    if all_texts.is_empty() {
        return Err("未识别到文字".to_string());
    }

    Ok(all_texts.join("\n"))
}
