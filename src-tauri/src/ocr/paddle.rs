use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use base64::Engine;
use oar_ocr::prelude::*;
use oar_ocr::domain::tasks::TextDetectionConfig;
use crate::download::{DownloadSpec, check_files};

const CARGO_DIR: &str = env!("CARGO_MANIFEST_DIR");

struct OcrEngine {
    engine: OAROCR,
}

const PADDLE_DIR: &str = "PaddleOCR";

/// The data directory where downloaded PaddleOCR models are stored.
fn models_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(crate::constants::APP_DATA_DIR)
        .join("models")
        .join("ocr")
        .join(PADDLE_DIR)
}

/// All directories that may contain PaddleOCR models, in priority order.
pub fn candidate_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    // 1) Portable mode: relative to executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            dirs.push(exe_dir.join("models").join("ocr").join(PADDLE_DIR));
        }
    }

    // 2) User data directory (downloaded models)
    dirs.push(models_data_dir());

    // 3) Development: relative to CARGO_MANIFEST_DIR
    let mut from_manifest = PathBuf::from(CARGO_DIR);
    from_manifest.pop();
    from_manifest = from_manifest.join("models").join("ocr").join(PADDLE_DIR);
    dirs.push(from_manifest);

    // 4) Current working directory
    dirs.push(
        std::env::current_dir()
            .unwrap_or_default()
            .join("models")
            .join("ocr")
            .join(PADDLE_DIR),
    );

    dirs
}

fn models_dir() -> PathBuf {
    candidate_dirs()
        .into_iter()
        .find(|d| {
            d.exists()
                && d.join("PP-OCRv6_medium_det.onnx").exists()
                && d.join("PP-OCRv6_medium_rec.onnx").exists()
                && d.join("ppocrv6_dict.txt").exists()
        })
        .unwrap_or_else(|| {
            let mut p = PathBuf::from(CARGO_DIR);
            p.pop();
            p.join("models").join("ocr").join(PADDLE_DIR)
        })
}

/// Where downloaded model files land.
fn download_dir() -> PathBuf {
    // Development: download into the project tree
    let dev_path = {
        let mut p = PathBuf::from(CARGO_DIR);
        p.pop();
        p.join("models").join("ocr").join(PADDLE_DIR)
    };
    if PathBuf::from(CARGO_DIR).exists() {
        return dev_path;
    }

    // Production: exe-relative (portable), then user data dir
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            return exe_dir.join("models").join("ocr").join(PADDLE_DIR);
        }
    }

    models_data_dir()
}

/// The three files PaddleOCR needs, as generic [`DownloadSpec`]s.
pub fn download_specs() -> Vec<DownloadSpec> {
    let dest = download_dir();
    vec![
        DownloadSpec {
            file_name: "PP-OCRv6_medium_det.onnx".into(),
            hf_repo: Some("PaddlePaddle/PP-OCRv6_medium_det_onnx".into()),
            remote_file_name: Some("inference.onnx".into()),
            direct_url: None,
            dest_dir: dest.clone(),
        },
        DownloadSpec {
            file_name: "PP-OCRv6_medium_rec.onnx".into(),
            hf_repo: Some("PaddlePaddle/PP-OCRv6_medium_rec_onnx".into()),
            remote_file_name: Some("inference.onnx".into()),
            direct_url: None,
            dest_dir: dest.clone(),
        },
        DownloadSpec {
            file_name: "ppocrv6_dict.txt".into(),
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
