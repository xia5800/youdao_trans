use crate::util;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    expires_in: Option<u64>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Deserialize)]
struct WordsResult {
    words: String,
}

#[derive(Deserialize)]
struct OcrResponse {
    words_result: Option<Vec<WordsResult>>,
    error_code: Option<String>,
    error_msg: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct TokenCacheFile {
    token: String,
    expires_at_unix: u64,
    key_hash: u64,
}

struct TokenEntry {
    token: String,
    expires_at: Instant,
    key_hash: u64,
}

static TOKEN_CACHE: Mutex<Option<TokenEntry>> = Mutex::new(None);

fn cache_filename() -> &'static str {
    #[cfg(debug_assertions)]
    {
        "ocr_token_cache.json"
    }
    #[cfg(not(debug_assertions))]
    {
        "ocr_token_cache.enc"
    }
}

fn cache_path() -> PathBuf {
    crate::config::default_config_dir_inner().join(cache_filename())
}

#[cfg(debug_assertions)]
fn read_cache_raw() -> Option<Vec<u8>> {
    std::fs::read(cache_path()).ok()
}

#[cfg(not(debug_assertions))]
fn read_cache_raw() -> Option<Vec<u8>> {
    let raw = std::fs::read(cache_path()).ok()?;
    crate::config::decrypt_data(&raw).ok()
}

fn load_cache_from_file() -> Option<TokenEntry> {
    let raw = read_cache_raw()?;
    let content = String::from_utf8(raw).ok()?;
    let file: TokenCacheFile = serde_json::from_str(&content).ok()?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    if now >= file.expires_at_unix {
        return None;
    }
    let remaining = file.expires_at_unix - now;
    Some(TokenEntry {
        token: file.token,
        expires_at: Instant::now() + Duration::from_secs(remaining),
        key_hash: file.key_hash,
    })
}

#[cfg(debug_assertions)]
fn write_cache_raw(data: &[u8]) {
    let path = cache_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, data);
}

#[cfg(not(debug_assertions))]
fn write_cache_raw(data: &[u8]) {
    let path = cache_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(encrypted) = crate::config::encrypt_data(data) {
        let _ = std::fs::write(path, encrypted);
    }
}

fn save_cache_to_file(token: &str, expires_at_unix: u64, key_hash: u64) {
    if let Ok(content) = serde_json::to_string(&TokenCacheFile {
        token: token.to_string(),
        expires_at_unix,
        key_hash,
    }) {
        write_cache_raw(content.as_bytes());
    }
}

fn compute_key_hash(api_key: &str, secret_key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    api_key.hash(&mut hasher);
    secret_key.hash(&mut hasher);
    hasher.finish()
}

fn get_cached_token(api_key: &str, secret_key: &str) -> Option<String> {
    let mut cache = TOKEN_CACHE.lock().ok()?;
    if cache.is_none() {
        *cache = load_cache_from_file();
    }
    let entry = cache.as_ref()?;
    let kh = compute_key_hash(api_key, secret_key);
    if entry.key_hash != kh || Instant::now() >= entry.expires_at {
        *cache = None;
        return None;
    }
    Some(entry.token.clone())
}

fn set_cached_token(api_key: &str, secret_key: &str, token: String, expires_in: u64) {
    let safe_ttl = if expires_in > 86400 {
        expires_in - 86400
    } else {
        expires_in
    };
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let expires_at_unix = now_secs + safe_ttl;
    let key_hash = compute_key_hash(api_key, secret_key);

    if let Ok(mut cache) = TOKEN_CACHE.lock() {
        *cache = Some(TokenEntry {
            token: token.clone(),
            expires_at: Instant::now() + Duration::from_secs(safe_ttl),
            key_hash,
        });
    }

    save_cache_to_file(&token, expires_at_unix, key_hash);
}

fn clear_token_cache() {
    if let Ok(mut cache) = TOKEN_CACHE.lock() {
        *cache = None;
    }
    let _ = std::fs::remove_file(cache_path());
}

async fn obtain_token(
    client: &reqwest::Client,
    api_key: &str,
    secret_key: &str,
) -> Result<(String, u64), String> {
    let token_url = format!(
        "https://aip.baidubce.com/oauth/2.0/token?grant_type=client_credentials&client_id={}&client_secret={}",
        api_key, secret_key
    );

    let token_resp = client
        .post(&token_url)
        .send()
        .await
        .map_err(|e| format!("获取百度OCR token失败: {}", e))?;

    let token_body: TokenResponse = token_resp
        .json()
        .await
        .map_err(|e| format!("解析百度OCR token响应失败: {}", e))?;

    let access_token = token_body.access_token.ok_or_else(|| {
        let err = token_body.error.unwrap_or_default();
        let desc = token_body.error_description.unwrap_or_default();
        format!("百度OCR获取token失败: {} - {}", err, desc)
    })?;

    let expires_in = token_body.expires_in.unwrap_or(2592000);
    Ok((access_token, expires_in))
}

async fn call_ocr(
    client: &reqwest::Client,
    access_token: &str,
    base64_img: &str,
) -> Result<String, String> {
    let ocr_url = format!(
        "https://aip.baidubce.com/rest/2.0/ocr/v1/general_basic?access_token={}",
        access_token
    );

    let ocr_resp = client
        .post(&ocr_url)
        .form(&[("image", base64_img)])
        .send()
        .await
        .map_err(|e| format!("请求百度OCR失败: {}", e))?;

    let ocr_body: OcrResponse = ocr_resp
        .json()
        .await
        .map_err(|e| format!("解析百度OCR响应失败: {}", e))?;

    if let Some(code) = ocr_body.error_code {
        let msg = ocr_body.error_msg.unwrap_or_default();
        return Err(format!("百度OCR错误 ({}): {}", code, msg));
    }

    let words = ocr_body
        .words_result
        .ok_or_else(|| "百度OCR返回结果为空".to_string())?;

    let text: Vec<String> = words.into_iter().map(|w| w.words).collect();
    Ok(text.join("\n"))
}

pub async fn ocr(base64_img: &str, keys: &HashMap<String, String>) -> Result<String, String> {
    let api_key = util::require_key(keys, "baidu_ocr-apikey", "百度云OCR API Key 未配置")?;
    let secret_key = util::require_key(keys, "baidu_ocr-apisecret", "百度云OCR Secret Key 未配置")?;

    let client = reqwest::Client::new();

    let access_token = match get_cached_token(api_key, secret_key) {
        Some(t) => t,
        None => {
            let (token, expires_in) = obtain_token(&client, api_key, secret_key).await?;
            set_cached_token(api_key, secret_key, token.clone(), expires_in);
            token
        }
    };

    let result = call_ocr(&client, &access_token, base64_img).await;

    match result {
        Ok(text) => Ok(text),
        Err(e) => {
            if e.contains("110") || e.contains("token") || e.contains("access_token") {
                clear_token_cache();
                let (new_token, _) = obtain_token(&client, api_key, secret_key).await?;
                call_ocr(&client, &new_token, base64_img).await
            } else {
                Err(e)
            }
        }
    }
}
