use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;
use base64::Engine;
use oar_ocr::prelude::*;
use oar_ocr::domain::tasks::TextDetectionConfig;
use crate::download::{DownloadSpec, check_files};

struct OcrEngine {
    engine: OAROCR,
}

/// The data directory where downloaded PaddleOCR models are stored.
fn models_data_dir() -> PathBuf {
    crate::config::default_models_dir_inner().join("ocr").join(crate::constants::OCR_MODEL_DIR)
}

/// All directories that may contain PaddleOCR models, in priority order.
pub fn candidate_dirs() -> Vec<PathBuf> {
    vec![models_data_dir()]
}

fn models_dir() -> PathBuf {
    candidate_dirs()
        .into_iter()
        .find(|d| {
            d.exists()
                && d.join(crate::constants::OCR_DET_MODEL).exists()
                && d.join(crate::constants::OCR_REC_MODEL).exists()
                && d.join(crate::constants::OCR_DICT_FILE).exists()
        })
        .unwrap_or_else(models_data_dir)
}

/// Where downloaded model files land.
fn download_dir() -> PathBuf {
    models_data_dir()
}

/// The three files PaddleOCR needs, as generic [`DownloadSpec`]s.
pub fn download_specs() -> Vec<DownloadSpec> {
    let dest = download_dir();
    vec![
        DownloadSpec {
            file_name: crate::constants::OCR_DET_MODEL.into(),
            hf_repo: Some("PaddlePaddle/PP-OCRv6_medium_det_onnx".into()),
            remote_file_name: Some("inference.onnx".into()),
            direct_url: None,
            dest_dir: dest.clone(),
        },
        DownloadSpec {
            file_name: crate::constants::OCR_REC_MODEL.into(),
            hf_repo: Some("PaddlePaddle/PP-OCRv6_medium_rec_onnx".into()),
            remote_file_name: Some("inference.onnx".into()),
            direct_url: None,
            dest_dir: dest.clone(),
        },
        DownloadSpec {
            file_name: crate::constants::OCR_DICT_FILE.into(),
            hf_repo: None,
            remote_file_name: None,
            direct_url: Some(
                "https://raw.githubusercontent.com/PaddlePaddle/PaddleOCR/main/ppocr/utils/dict/ppocrv6_dict.txt"
                    .into(),
            ),
            dest_dir: dest,
        },
    ]
}

/// Check which PaddleOCR model files exist in any candidate directory.
pub fn check_model_files() -> std::collections::HashMap<String, bool> {
    let dirs = candidate_dirs();
    let specs = download_specs();
    check_files(&specs, &dirs)
        .into_iter()
        .collect()
}

static ENGINE: OnceLock<Mutex<(Option<OcrEngine>, Instant)>> = OnceLock::new();
const ENGINE_IDLE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(300);

fn init_engine() -> Result<OcrEngine, String> {
    let dir = models_dir();
    let det_path = dir.join(crate::constants::OCR_DET_MODEL);
    let rec_path = dir.join(crate::constants::OCR_REC_MODEL);
    let dict_path = dir.join(crate::constants::OCR_DICT_FILE);

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

fn get_engine() -> Result<&'static Mutex<(Option<OcrEngine>, Instant)>, String> {
    let cell = ENGINE.get_or_init(|| Mutex::new((None, Instant::now())));

    let mut guard = cell.lock().map_err(|e| format!("引擎锁异常: {}", e))?;
    if guard.0.is_none() || guard.1.elapsed() > ENGINE_IDLE_TIMEOUT {
        if guard.0.is_some() {
            log::info!("PaddleOCR 引擎闲置超时，重新初始化");
        }
        guard.0 = Some(init_engine().map_err(|e| {
            log::error!("PaddleOCR 引擎初始化失败: {}", e);
            format!("PaddleOCR 引擎初始化失败: {}", e)
        })?);
    }
    guard.1 = Instant::now();
    drop(guard);

    Ok(cell)
}

pub fn unload_engine() {
    if let Some(cell) = ENGINE.get() {
        if let Ok(mut guard) = cell.lock() {
            log::info!("卸载 PaddleOCR 引擎，释放内存");
            guard.0 = None;
        }
    }
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
