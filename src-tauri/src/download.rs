//! Generic download infrastructure using `hf-fetch-model` for HuggingFace repos
//! and custom HTTP downloads for non-HF files (GitHub raw, etc.).
//!
//! Any module can define its own [`DownloadSpec`] list and call [`download_files`].
//! The frontend listens to `download-start`, `download-progress`, and
//! `download-complete` events for progress bars.

use std::path::PathBuf;
use std::sync::Arc;
use futures::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tauri::Emitter;

use crate::constants;

const GITHUB_RAW_MIRROR: &str = "https://cdn.jsdelivr.net/gh";

/// Describes a single file to download.
pub struct DownloadSpec {
    pub file_name: String,
    /// HuggingFace repo ID (e.g. `"PaddlePaddle/PP-OCRv6_medium_det_onnx"`).
    /// If `Some`, uses `hf-fetch-model` for the download.
    pub hf_repo: Option<String>,
    /// File name within the HuggingFace repo (e.g. `"inference.onnx"`).
    /// When `None`, defaults to `file_name`.
    pub remote_file_name: Option<String>,
    /// Direct URL (for files not on HuggingFace, e.g. GitHub raw).
    /// Ignored when `hf_repo` is set.
    pub direct_url: Option<String>,
    /// Absolute destination directory (file is moved here after download succeeds).
    pub dest_dir: PathBuf,
}

fn build_direct_url(spec: &DownloadSpec, use_github_mirror: bool) -> Result<String, String> {
    let url = spec.direct_url.as_ref()
        .ok_or_else(|| format!("direct_url 未设置 (file: {})", spec.file_name))?;
    if use_github_mirror && url.starts_with("https://raw.githubusercontent.com/") {
        let path = url.strip_prefix("https://raw.githubusercontent.com/")
            .expect("already checked starts_with");
        if let Some(at) = path.match_indices('/').nth(1).map(|(i, _)| i) {
            Ok(format!("{}/{}{}", GITHUB_RAW_MIRROR, &path[..at], &path[at..].replacen('/', "@", 1)))
        } else {
            Ok(format!("{}/{}", GITHUB_RAW_MIRROR, path))
        }
    } else {
        Ok(url.clone())
    }
}

/// Check whether all files in the specs exist at the candidate directories.
pub fn check_files(
    specs: &[DownloadSpec],
    candidate_dirs: &[PathBuf],
) -> Vec<(String, bool)> {
    specs
        .iter()
        .map(|s| {
            let exists = candidate_dirs.iter().any(|d| d.join(&s.file_name).exists());
            (s.file_name.clone(), exists)
        })
        .collect()
}

#[derive(Clone, serde::Serialize)]
#[allow(unused)]
pub struct DownloadProgressPayload {
    pub file_name: String,
    pub total: u64,
    pub downloaded: u64,
    pub status: String,
}

fn emit_progress(
    app: &tauri::AppHandle,
    event: &str,
    file_name: &str,
    total: u64,
    downloaded: u64,
    status: &str,
) {
    let _ = app.emit(
        event,
        DownloadProgressPayload {
            file_name: file_name.to_string(),
            total,
            downloaded,
            status: status.to_string(),
        },
    );
}

/// Max time to wait for TCP connection.
const CONNECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
/// Max time waiting for initial response headers or idle between data chunks.
const CHUNK_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// Download a single file from a direct URL (non-HF) with progress.
async fn download_direct_url(
    app: &tauri::AppHandle,
    client: &reqwest::Client,
    spec: &DownloadSpec,
    temp_dir: &std::path::Path,
    use_github_mirror: bool,
) -> Result<(), String> {
    let url = build_direct_url(spec, use_github_mirror)?;
    let temp_path = temp_dir.join(&spec.file_name);
    let final_path = spec.dest_dir.join(&spec.file_name);

    log::info!("下载开始: {} → {}", spec.file_name, url);

    let response = tokio::time::timeout(
        CHUNK_TIMEOUT,
        client.get(&url).send(),
    )
    .await
    .map_err(|_| format!("请求超时 ({}): 服务器 {} 秒无响应", spec.file_name, CHUNK_TIMEOUT.as_secs()))?
    .map_err(|e| format!("请求失败 ({}): {}", spec.file_name, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "下载失败 ({}): HTTP {}",
            spec.file_name,
            response.status()
        ));
    }

    let total = response.content_length().unwrap_or(0);
    emit_progress(
        app,
        constants::EVENT_DOWNLOAD_START,
        &spec.file_name,
        total,
        0,
        "downloading",
    );

    let mut file = tokio::fs::File::create(&temp_path)
        .await
        .map_err(|e| format!("创建临时文件失败 ({}): {}", spec.file_name, e))?;

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;

    loop {
        let chunk = tokio::time::timeout(CHUNK_TIMEOUT, stream.next())
            .await
            .map_err(|_| format!("下载超时 ({}): 超过 {} 秒无数据", spec.file_name, CHUNK_TIMEOUT.as_secs()))?;

        match chunk {
            Some(Ok(data)) => {
                file.write_all(&data)
                    .await
                    .map_err(|e| format!("写入文件失败 ({}): {}", spec.file_name, e))?;
                downloaded += data.len() as u64;
                emit_progress(
                    app,
                    constants::EVENT_DOWNLOAD_PROGRESS,
                    &spec.file_name,
                    total,
                    downloaded,
                    "downloading",
                );
            }
            Some(Err(e)) => {
                return Err(format!("下载中断 ({}): {}", spec.file_name, e));
            }
            None => break,
        }
    }

    file.flush()
        .await
        .map_err(|e| format!("刷新文件失败 ({}): {}", spec.file_name, e))?;
    drop(file);

    tokio::fs::create_dir_all(&spec.dest_dir)
        .await
        .map_err(|e| format!("创建目录失败: {}", e))?;

    log::info!("移动文件: {} → {}", temp_path.display(), final_path.display());
    if let Err(e) = tokio::fs::rename(&temp_path, &final_path).await {
        log::warn!("rename 失败 (可能跨驱动器), 回退到 copy+删除: {}", e);
        tokio::fs::copy(&temp_path, &final_path)
            .await
            .map_err(|_| format!("移动文件失败 ({}): 跨驱动器复制失败", spec.file_name))?;
        tokio::fs::remove_file(&temp_path)
            .await
            .ok();
    }

    emit_progress(
        app,
        constants::EVENT_DOWNLOAD_COMPLETE,
        &spec.file_name,
        total,
        downloaded,
        "completed",
    );

    Ok(())
}

/// Download a single file from a HuggingFace repo via `hf-fetch-model`.
async fn download_hf_file(
    app: &tauri::AppHandle,
    spec: &DownloadSpec,
) -> Result<(), String> {
    let repo_id = spec.hf_repo.as_ref().expect("hf_repo required");
    let final_path = spec.dest_dir.join(&spec.file_name);

    let app_clone = app.clone();
    let fname = spec.file_name.clone();

    // FetchConfig with progress callback
    let config = hf_fetch_model::FetchConfig::builder()
        .on_progress(move |event| {
            emit_progress(
                &app_clone,
                constants::EVENT_DOWNLOAD_PROGRESS,
                &fname,
                event.bytes_total,
                event.bytes_downloaded,
                "downloading",
            );
        })
        .build()
        .map_err(|e| format!("配置下载失败: {}", e))?;

    emit_progress(
        app,
        constants::EVENT_DOWNLOAD_START,
        &spec.file_name,
        0,
        0,
        "downloading",
    );

    // The file name inside the HF repo (may differ from the local file_name)
    let remote_name = spec.remote_file_name.as_deref().unwrap_or(&spec.file_name);

    log::info!("HF 下载开始: {} (仓库: {}, 远端文件: {})", spec.file_name, repo_id, remote_name);

    // Download via hf-fetch-model (uses HF cache at ~/.cache/huggingface/hub/)
    let outcome = hf_fetch_model::download_file(
        repo_id.clone(),
        remote_name,
        &config,
    )
    .await
    .map_err(|e| format!("下载失败 ({}): {}", spec.file_name, e))?;

    let cached = outcome.inner();

    // Copy from HF cache to our target directory
    tokio::fs::create_dir_all(&spec.dest_dir)
        .await
        .map_err(|e| format!("创建目录失败: {}", e))?;

    log::info!("复制文件: {} → {}", cached.display(), final_path.display());
    tokio::fs::copy(&cached, &final_path)
        .await
        .map_err(|e| format!("复制文件失败 ({}): {}", spec.file_name, e))?;

    let metadata = std::fs::metadata(&cached)
        .map_err(|e| format!("获取文件信息失败: {}", e))?;
    let total = metadata.len();

    emit_progress(
        app,
        constants::EVENT_DOWNLOAD_COMPLETE,
        &spec.file_name,
        total,
        total,
        "completed",
    );

    Ok(())
}

/// Download a batch of files concurrently.
///
/// * For files with `hf_repo`: uses `hf-fetch-model` (multi-connection, resume, HF cache).
/// * For files with `direct_url`: uses custom HTTP download with temp → move.
fn hf_env(mirror: bool) {
    if mirror {
        std::env::set_var("HF_ENDPOINT", "https://hf-mirror.com");
    } else {
        std::env::remove_var("HF_ENDPOINT");
    }
}

pub async fn download_files(
    app: &tauri::AppHandle,
    specs: &[DownloadSpec],
    use_hf_mirror: bool,
    use_github_mirror: bool,
) -> Result<(), String> {
    let client = reqwest::Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;
    let temp_root = std::env::temp_dir().join("youdao-fanyi-download");

    tokio::fs::create_dir_all(&temp_root)
        .await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    hf_env(use_hf_mirror);

    if let Some(first) = specs.first() {
        log::info!("模型文件下载目录: {}", first.dest_dir.display());
    }

    let results: Arc<Mutex<Vec<(String, Result<(), String>)>>> =
        Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for spec in specs {
        // Skip files that already exist at destination
        if spec.dest_dir.join(&spec.file_name).exists() {
            results
                .lock()
                .await
                .push((spec.file_name.clone(), Ok(())));
            continue;
        }

        let app = app.clone();
        let client = client.clone();
        let temp_root = temp_root.clone();
        let spec: Arc<DownloadSpec> = Arc::new(DownloadSpec {
            file_name: spec.file_name.clone(),
            hf_repo: spec.hf_repo.clone(),
            remote_file_name: spec.remote_file_name.clone(),
            direct_url: spec.direct_url.clone(),
            dest_dir: spec.dest_dir.clone(),
        });
        let results = results.clone();

        handles.push(tokio::spawn(async move {
            let r = if spec.hf_repo.is_some() {
                download_hf_file(&app, &spec).await
            } else {
                download_direct_url(&app, &client, &spec, &temp_root, use_github_mirror).await
            };
            if let Err(ref e) = r {
                emit_progress(&app, constants::EVENT_DOWNLOAD_ERROR, &spec.file_name, 0, 0, &format!("{}", e));
            }
            results.lock().await.push((spec.file_name.clone(), r));
        }));
    }

    for handle in handles {
        handle
            .await
            .map_err(|e| format!("下载任务异常: {}", e))?;
    }

    let _ = tokio::fs::remove_dir_all(&temp_root).await;

    let results = results.lock().await;
    let errors: Vec<String> = results
        .iter()
        .filter_map(|(name, r)| r.as_ref().err().map(|e| format!("{}: {}", name, e)))
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

/// Download a single file (retry).
pub async fn retry_download_file(
    app: &tauri::AppHandle,
    spec: DownloadSpec,
    use_hf_mirror: bool,
    use_github_mirror: bool,
) -> Result<(), String> {
    hf_env(use_hf_mirror);
    let r = if spec.hf_repo.is_some() {
        download_hf_file(app, &spec).await
    } else {
        let client = reqwest::Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .build()
            .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;
        let temp_root = std::env::temp_dir().join("youdao-fanyi-download");
        tokio::fs::create_dir_all(&temp_root)
            .await
            .map_err(|e| format!("创建临时目录失败: {}", e))?;
        let result =
            download_direct_url(app, &client, &spec, &temp_root, use_github_mirror).await;
        let _ = tokio::fs::remove_dir_all(&temp_root).await;
        result
    };
    if let Err(ref e) = r {
        emit_progress(app, constants::EVENT_DOWNLOAD_ERROR, &spec.file_name, 0, 0, &format!("{}", e));
    }
    r
}
