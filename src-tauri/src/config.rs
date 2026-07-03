#[cfg(not(debug_assertions))]
use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
#[cfg(not(debug_assertions))]
use aes_gcm::{Aes256Gcm, Nonce};
use serde::{Deserialize, Serialize};
#[cfg(not(debug_assertions))]
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;


#[cfg(not(debug_assertions))]
const APP_SALT: &[u8] = b"youdao-fanyi-v1";
#[cfg(not(debug_assertions))]
const TOKEN_CACHE_SALT: &[u8] = b"youdao-fanyi-ocr-v1";

// ---- Data model ----

#[derive(Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub theme: String,
    pub auto_start: bool,
    pub delay_time: u64,
    pub store_records: bool,
    pub replace_newlines: bool,
    pub programmer_mode: bool,
    pub auto_translate: bool,
    pub auto_translate_delay: u64,
    pub hotkeys: Hotkeys,
    pub active_translator: Option<String>,
    pub translator_keys: HashMap<String, String>,
    pub active_ocr: Option<String>,
    pub ocr_keys: HashMap<String, String>,
    pub close_behavior: String,
    pub show_screenshot_crosshair: bool,
    pub tts_engine: String,
    pub tts_voice: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hotkeys {
    #[serde(default = "default_hotkey_translate")]
    pub translate: String,
    #[serde(default = "default_hotkey_screenshot")]
    pub screenshot: String,
    #[serde(default = "default_hotkey_selection_translate")]
    pub selection_translate: String,
}

fn default_hotkey_translate() -> String {
    "Ctrl+Enter".into()
}
fn default_hotkey_screenshot() -> String {
    "Alt+W".into()
}
fn default_hotkey_selection_translate() -> String {
    "Alt+T".into()
}

impl Default for Hotkeys {
    fn default() -> Self {
        Self {
            translate: default_hotkey_translate(),
            screenshot: default_hotkey_screenshot(),
            selection_translate: default_hotkey_selection_translate(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "system".into(),
            auto_start: false,
            delay_time: crate::constants::DEFAULT_DELAY_TIME_MS,
            store_records: true,
            replace_newlines: false,
            programmer_mode: true,
            auto_translate: false,
            auto_translate_delay: crate::constants::DEFAULT_AUTO_TRANSLATE_DELAY_MS,
            hotkeys: Hotkeys::default(),
            active_translator: Some("microsoft_free".to_string()),
            translator_keys: HashMap::new(),
            active_ocr: Some("ollama_ocr".to_string()),
            ocr_keys: HashMap::new(),
            close_behavior: "ask".into(),
            show_screenshot_crosshair: true,
            tts_engine: "browser".into(),
            tts_voice: "zh-CN-XiaoxiaoNeural".into(),
        }
    }
}

// ---- File I/O ----

pub fn filename() -> &'static str {
    #[cfg(debug_assertions)]
    {
        "config-dev.json"
    }
    #[cfg(not(debug_assertions))]
    {
        "config.enc"
    }
}

#[cfg(debug_assertions)]
pub fn dev_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p
}

fn data_root() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        dev_root()
    }
    #[cfg(not(debug_assertions))]
    {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                return exe_dir.to_path_buf();
            }
        }
        PathBuf::from(".")
    }
}

pub fn default_config_dir_inner() -> PathBuf {
    data_root().join("config")
}

pub fn default_db_dir_inner() -> PathBuf {
    data_root().join("db")
}

pub fn default_models_dir_inner() -> PathBuf {
    data_root().join("models")
}

pub fn default_logs_dir_inner() -> PathBuf {
    data_root().join("logs")
}

fn default_path() -> PathBuf {
    default_config_dir_inner().join(filename())
}

#[cfg(not(debug_assertions))]
fn derive_key_with_salt(salt: &[u8]) -> Result<[u8; 32], String> {
    static MACHINE_ID: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    let uid = MACHINE_ID.get_or_init(|| {
        machine_uid::get().unwrap_or_else(|_| "fallback-id".to_string())
    });
    let mut hasher = Sha256::new();
    hasher.update(uid.as_bytes());
    hasher.update(salt);
    Ok(hasher.finalize().into())
}

#[cfg(not(debug_assertions))]
fn derive_key() -> Result<[u8; 32], String> {
    derive_key_with_salt(APP_SALT)
}

#[cfg(not(debug_assertions))]
pub fn encrypt_data(plaintext: &[u8]) -> Result<Vec<u8>, String> {
    use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
    use aes_gcm::Aes256Gcm;
    let key = derive_key_with_salt(TOKEN_CACHE_SALT)?;
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| format!("key init: {}", e))?;
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| format!("encrypt: {}", e))?;
    let mut out = nonce.to_vec();
    out.extend(ciphertext);
    Ok(out)
}

#[cfg(not(debug_assertions))]
pub fn decrypt_data(raw: &[u8]) -> Result<Vec<u8>, String> {
    use aes_gcm::aead::{Aead, KeyInit};
    use aes_gcm::{Aes256Gcm, Nonce};
    if raw.len() < 12 {
        return Err("data too short".into());
    }
    let key = derive_key_with_salt(TOKEN_CACHE_SALT)?;
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| format!("key init: {}", e))?;
    let (nonce_bytes, ciphertext) = raw.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("decrypt: {}", e))
}

fn serialize_settings(settings: &Settings) -> Result<String, String> {
    serde_json::to_string(settings).map_err(|e| format!("serialize: {}", e))
}

fn parse_settings(json: &str) -> Result<Settings, String> {
    serde_json::from_str::<Settings>(json).map_err(|e| format!("parse: {}", e))
}

/// Read raw bytes, decode to JSON string, then parse into Settings.
/// If the file is empty / corrupted / tampered, it is removed and defaults are returned.
fn decode_settings(p: &std::path::Path, raw: Vec<u8>) -> Result<String, String> {
    if raw.is_empty() {
        let _ = std::fs::remove_file(p);
        return serialize_settings(&Settings::default());
    }

    let json_str = {
        #[cfg(debug_assertions)]
        {
            String::from_utf8(raw).map_err(|e| format!("utf8: {}", e))?
        }

        #[cfg(not(debug_assertions))]
        {
            let key = derive_key()?;
            let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| format!("key init: {}", e))?;
            if raw.len() < 12 {
                let _ = std::fs::remove_file(p);
                return Err("config file too short, removed".into());
            }
            let (nonce_bytes, ciphertext) = raw.split_at(12);
            let nonce = Nonce::from_slice(nonce_bytes);
            let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| {
                let _ = std::fs::remove_file(p);
                format!("decrypt failed, removed bad config: {}", e)
            })?;
            String::from_utf8(plaintext).map_err(|e| format!("utf8: {}", e))?
        }
    };

    // Parse → merge with defaults via #[serde(default)], then re-serialize
    match parse_settings(&json_str) {
        Ok(s) => serialize_settings(&s),
        Err(_) => {
            let _ = std::fs::remove_file(p);
            serialize_settings(&Settings::default())
        }
    }
}

pub fn load() -> Result<String, String> {
    let p = default_path();
    if !p.exists() {
        return serialize_settings(&Settings::default());
    }
    let raw = std::fs::read(&p).map_err(|e| {
        log::error!("读取配置文件失败: {}", e);
        format!("read config: {}", e)
    })?;
    decode_settings(&p, raw)
}

pub fn save(json: &str) -> Result<(), String> {
    let p = default_path();

    // Validate before writing — reject malformed JSON
    if let Err(e) = parse_settings(json) {
        log::error!("保存配置时JSON校验失败: {}", e);
        return Err(e);
    }

    let data: Vec<u8> = {
        #[cfg(debug_assertions)]
        {
            json.as_bytes().to_vec()
        }

        #[cfg(not(debug_assertions))]
        {
            let key = derive_key()?;
            let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| format!("key init: {}", e))?;
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
            let ciphertext = cipher
                .encrypt(&nonce, json.as_bytes())
                .map_err(|e| format!("encrypt: {}", e))?;
            let mut d = nonce.to_vec();
            d.extend(ciphertext);
            d
        }
    };

    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            log::error!("创建配置目录失败: {}", e);
            format!("create dir: {}", e)
        })?;
    }
    std::fs::write(&p, data).map_err(|e| {
        log::error!("写入配置文件失败: {}", e);
        format!("write config: {}", e)
    })?;
    Ok(())
}

// ---- Tauri command wrappers ----

use tauri::command;

#[command]
pub fn load_config() -> Result<String, String> {
    load()
}

#[command]
pub fn save_config(json: String) -> Result<(), String> {
    save(&json)
}

#[command]
pub fn config_filename() -> String {
    filename().to_string()
}
