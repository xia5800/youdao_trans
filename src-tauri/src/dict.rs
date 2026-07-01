use crate::constants;
use rusqlite::Connection;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tauri::Emitter;
use tokio::sync::{Mutex, Notify};

const DICT_ZIP_URL: &str = "https://github.com/skywind3000/ECDICT/releases/download/1.0.28/ecdict-sqlite-28.zip";
const DICT_ZIP_FILE: &str = "ecdict-sqlite-28.zip";
const DICT_MODEL_DIR: &str = "dict";
const DICT_GITHUB_MIRROR: &str = "https://gh-proxy.com";

pub struct DictDownloadState {
    pub cancel_flag: AtomicBool,
    pub paused_flag: AtomicBool,
    pub notify: Notify,
    pub temp_dir: Mutex<Option<PathBuf>>,
    pub downloaded: AtomicU64,
    pub total: AtomicU64,
}

impl DictDownloadState {
    pub fn new() -> Self {
        Self {
            cancel_flag: AtomicBool::new(false),
            paused_flag: AtomicBool::new(false),
            notify: Notify::new(),
            temp_dir: Mutex::new(None),
            downloaded: AtomicU64::new(0),
            total: AtomicU64::new(0),
        }
    }
}

fn db_path() -> PathBuf {
    crate::config::default_models_dir_inner().join(format!("{}/stardict.db", DICT_MODEL_DIR))
}

#[derive(Serialize)]
pub struct WordEntry {
    pub word: String,
    pub phonetic: String,
    pub definition: String,
    pub translation: String,
    pub pos: String,
    pub collins: i32,
    pub oxford: i32,
    pub tag: String,
    pub bnc: Option<i32>,
    pub frq: Option<i32>,
    pub exchange: String,
    pub detail: String,
}

#[derive(Serialize)]
pub struct ExchangeItem {
    pub label: String,
    pub value: String,
}

fn parse_exchange(raw: &str) -> Vec<ExchangeItem> {
    let order = ["0", "1", "s", "p", "d", "i", "3", "r", "t"];
    let labels = [
        ("0", "原型"), ("1", "复数形式"), ("s", "复数"),
        ("p", "过去式"), ("d", "过去分词"), ("i", "现在分词"),
        ("3", "第三人称单数"), ("r", "比较级"), ("t", "最高级"),
    ];
    let mut seen = std::collections::HashSet::new();
    let mut map = std::collections::BTreeMap::new();
    for part in raw.split('/') {
        let mut it = part.splitn(2, ':');
        let key = it.next().unwrap_or("");
        let val = it.next().unwrap_or("").trim_start_matches('\'');
        if val.is_empty() || !seen.insert(val) { continue; }
        let idx = order.iter().position(|&k| k == key).unwrap_or(99);
        let label = labels.iter().find(|(k, _)| *k == key).map(|(_, l)| *l).unwrap_or(key);
        map.insert(idx, ExchangeItem { label: label.to_string(), value: val.to_string() });
    }
    map.into_values().collect()
}

pub fn suggestions(prefix: &str, limit: u32) -> Result<Vec<String>, String> {
    let path = db_path();
    let conn = Connection::open(&path).map_err(|e| {
        log::error!("打开词典数据库失败: {}", e);
        format!("open dict: {}", e)
    })?;
    let pattern = format!("{}%", prefix);
    let mut stmt = conn
        .prepare(
            "SELECT word FROM stardict \
             WHERE sw LIKE ?1 AND word != '' \
             ORDER BY length(word), coalesce(frq, 99999) \
             LIMIT ?2",
        )
        .map_err(|e| format!("prepare: {}", e))?;
    let rows = stmt
        .query_map(rusqlite::params![pattern, limit], |row| row.get::<_, String>(0))
        .map_err(|e| format!("query: {}", e))?;
    let mut words = Vec::new();
    for row in rows {
        words.push(row.map_err(|e| format!("row: {}", e))?);
    }
    Ok(words)
}

pub fn lookup(word: &str) -> Result<Option<WordEntry>, String> {
    let path = db_path();
    let conn = Connection::open(&path).map_err(|e| format!("open dict: {}", e))?;
    let mut stmt = conn
        .prepare(
            "SELECT word, phonetic, definition, translation, pos, \
             collins, oxford, tag, bnc, frq, exchange, detail \
             FROM stardict WHERE word = ?1 COLLATE NOCASE LIMIT 1",
        )
        .map_err(|e| format!("prepare: {}", e))?;
    let mut rows = stmt
        .query_map(rusqlite::params![word], |row| {
            Ok(WordEntry {
                word: row.get(0)?,
                phonetic: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                definition: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                translation: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                pos: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                collins: row.get::<_, Option<i32>>(5)?.unwrap_or(0),
                oxford: row.get::<_, Option<i32>>(6)?.unwrap_or(0),
                tag: row.get::<_, Option<String>>(7)?.unwrap_or_default(),
                bnc: row.get(8)?,
                frq: row.get(9)?,
                exchange: row.get::<_, Option<String>>(10)?.unwrap_or_default(),
                detail: row.get::<_, Option<String>>(11)?.unwrap_or_default(),
            })
        })
        .map_err(|e| format!("query: {}", e))?;
    match rows.next() {
        Some(r) => Ok(Some(r.map_err(|e| format!("row: {}", e))?)),
        None => Ok(None),
    }
}

#[derive(Serialize)]
struct LookupResult {
    word: String,
    phonetic: String,
    meanings: Vec<serde_json::Value>,
    definitions: Vec<String>,
    tags: Vec<String>,
    collins: i32,
    oxford: i32,
    bnc: Option<i32>,
    frq: Option<i32>,
    exchange: Vec<ExchangeItem>,
    detail: String,
}

fn map_pos_label(prefix: &str) -> &str {
    match prefix.to_lowercase().trim_end_matches('.') {
        "n" => "n 名词",
        "v" => "v 动词",
        "vi" => "vi 不及物动词",
        "vt" => "vt 及物动词",
        "aux" => "aux 助动词",
        "adj" | "j" => "adj 形容词",
        "adv" | "r" => "adv 副词",
        "prep" => "prep 介词",
        "pron" => "pron 代词",
        "conj" => "conj 连词",
        "interj" | "int" | "u" => "int 感叹词",
        "num" => "num 数词",
        "art" | "a" => "art 冠词",
        "det" => "det 限定词",
        "pref" => "pref 前缀",
        "suff" => "suff 后缀",
        "pl" => "pl 复数",
        _ => prefix.trim_end_matches('.'),
    }
}

/// Maps a translation-line prefix to the code used in the `pos` ratio field
fn pos_ratio_code(prefix: &str) -> Option<&str> {
    Some(match prefix.to_lowercase().trim_end_matches('.') {
        "adj" | "j" => "j",
        "adv" | "r" => "r",
        "interj" | "int" | "u" => "u",
        "art" | "a" => "a",
        "vi" | "vt" => "v",       // pos field uses the umbrella "v"
        "v" | "n" | "prep" | "pron" | "conj" | "num"
            | "det" | "pref" | "suff" | "pl" => prefix.trim_end_matches('.'),
        _ => return None,
    })
}

fn parse_pos_ratios(raw: &str) -> std::collections::HashMap<String, i32> {
    let mut map = std::collections::HashMap::new();
    for part in raw.split('/') {
        let mut it = part.splitn(2, ':');
        if let (Some(code), Some(pct)) = (it.next(), it.next()) {
            if let Ok(n) = pct.trim().parse::<i32>() {
                map.insert(code.trim().to_string(), n);
            }
        }
    }
    map
}

fn extract_prefix(line: &str) -> Option<String> {
    let line = line.trim();
    if let Some(dot) = line.find(". ") {
        let candidate = &line[..dot];
        if !candidate.contains(' ') && candidate.len() <= 5 {
            return Some(candidate.to_string());
        }
    }
    None
}

fn parse_translation_line(line: &str) -> (String, String) {
    let line = line.trim();
    if let Some(dot) = line.find(". ") {
        let candidate = &line[..dot];
        if !candidate.contains(' ') && candidate.len() <= 5 {
            let pos = map_pos_label(candidate);
            let text = line[dot + 2..].trim().to_string();
            return (pos.to_string(), text);
        }
    }
    ("".to_string(), line.to_string())
}

pub fn lookup_formatted(word: &str) -> Result<Option<serde_json::Value>, String> {
    let entry = lookup(word)?;
    match entry {
        None => Ok(None),
        Some(e) => {
            let tags: Vec<String> = e.tag.split_whitespace()
                .filter(|t| !t.is_empty()).map(|s| s.to_string()).collect();
            let exchange_vec = parse_exchange(&e.exchange);
            let pos_ratios = parse_pos_ratios(&e.pos);

            let definitions: Vec<String> = e.definition.split('\n')
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();

            let mut meanings = Vec::new();
            for line in e.translation.split('\n') {
                let l = line.trim();
                if l.is_empty() { continue; }
                let (pos, text) = parse_translation_line(l);

                // Look up percentage from pos field
                let pct = extract_prefix(l)
                    .as_deref()
                    .and_then(pos_ratio_code)
                    .and_then(|code| pos_ratios.get(code))
                    .copied();

                let mut obj = serde_json::Map::new();
                obj.insert("pos".into(), serde_json::Value::String(pos));
                obj.insert("translation".into(), serde_json::Value::String(text));
                if let Some(p) = pct {
                    obj.insert("pct".into(), serde_json::Value::Number(p.into()));
                }
                meanings.push(serde_json::Value::Object(obj));
            }

            let result = LookupResult {
                word: e.word,
                phonetic: e.phonetic,
                meanings,
                definitions,
                tags,
                collins: e.collins,
                oxford: e.oxford,
                bnc: e.bnc,
                frq: e.frq,
                exchange: exchange_vec,
                detail: e.detail,
            };
            serde_json::to_value(result).map(Some).map_err(|e| format!("serialize: {}", e))
        }
    }
}

// ---- Tauri command wrappers ----

#[tauri::command]
pub fn dict_suggestions(prefix: String, limit: Option<u32>) -> Result<Vec<String>, String> {
    suggestions(&prefix, limit.unwrap_or(constants::DICT_DEFAULT_SUGGESTION_LIMIT))
}

#[tauri::command]
pub fn dict_lookup(word: String) -> Result<Option<serde_json::Value>, String> {
    lookup_formatted(&word)
}

#[tauri::command]
pub fn check_dict_db() -> bool {
    db_path().exists()
}

async fn download_loop(
    app: &tauri::AppHandle,
    state: &DictDownloadState,
    url: &str,
    zip_path: &PathBuf,
    dict_dir: &PathBuf,
) -> Result<(), String> {
    let client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    let (response, offset) = loop {
        if state.cancel_flag.load(Ordering::SeqCst) {
            return Err("下载已取消".into());
        }

        while state.paused_flag.load(Ordering::SeqCst) {
            state.notify.notified().await;
            if state.cancel_flag.load(Ordering::SeqCst) {
                return Err("下载已取消".into());
            }
        }

        let offset = state.downloaded.load(Ordering::SeqCst);

        let mut req = client.get(url);
        if offset > 0 {
            req = req.header("Range", format!("bytes={}-", offset));
        }

        let response = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            req.send(),
        )
        .await
        .map_err(|_| "下载请求超时")?
        .map_err(|e| format!("下载请求失败: {}", e))?;

        if offset > 0 {
            if response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
                state.downloaded.store(0, Ordering::SeqCst);
                let _ = tokio::fs::remove_file(zip_path).await;
                continue;
            }
        } else if !response.status().is_success() {
            return Err(format!("下载失败: HTTP {}", response.status()));
        }

        break (response, offset);
    };

    let total = state.total.load(Ordering::SeqCst);
    if total == 0 {
        if let Some(cl) = response.content_length() {
            state.total.store(cl, Ordering::SeqCst);
        }
    }

    let mut file = if offset > 0 {
        tokio::fs::OpenOptions::new()
            .append(true)
            .open(zip_path)
            .await
            .map_err(|e| format!("打开文件失败: {}", e))?
    } else {
        tokio::fs::File::create(zip_path)
            .await
            .map_err(|e| format!("创建文件失败: {}", e))?
    };

    use futures::StreamExt;
    use tokio::io::AsyncWriteExt;

    let mut stream = response.bytes_stream();
    let mut downloaded = offset;

    while let Some(chunk) = stream.next().await {
        if state.cancel_flag.load(Ordering::SeqCst) {
            return Err("下载已取消".into());
        }

        while state.paused_flag.load(Ordering::SeqCst) {
            state.notify.notified().await;
            if state.cancel_flag.load(Ordering::SeqCst) {
                return Err("下载已取消".into());
            }
        }

        let data = chunk.map_err(|e| format!("下载中断: {}", e))?;
        file.write_all(&data).await.map_err(|e| format!("写入文件失败: {}", e))?;
        downloaded += data.len() as u64;
        state.downloaded.store(downloaded, Ordering::SeqCst);

        let total_val = state.total.load(Ordering::SeqCst);
        app.emit(constants::EVENT_DOWNLOAD_PROGRESS, crate::download::DownloadProgressPayload {
            file_name: DICT_ZIP_FILE.to_string(),
            total: total_val,
            downloaded,
            status: "downloading".into(),
        }).ok();
    }

    file.flush().await.map_err(|e| format!("刷新文件失败: {}", e))?;
    drop(file);

    let zip_path2 = zip_path.clone();
    let dict_dir2 = dict_dir.clone();

    tokio::task::spawn_blocking(move || {
        let zip_file = std::fs::File::open(&zip_path2)
            .map_err(|e| format!("打开zip文件失败: {}", e))?;
        let mut archive = zip::ZipArchive::new(zip_file)
            .map_err(|e| format!("读取zip文件失败: {}", e))?;

        std::fs::create_dir_all(&dict_dir2)
            .map_err(|e| format!("创建目标目录失败: {}", e))?;

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)
                .map_err(|e| format!("读取zip条目失败: {}", e))?;
            let out_path = dict_dir2.join(entry.name());

            if entry.is_dir() {
                std::fs::create_dir_all(&out_path)
                    .map_err(|e| format!("创建目录失败: {}", e))?;
                continue;
            }

            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("创建目录失败: {}", e))?;
            }

            let mut outfile = std::fs::File::create(&out_path)
                .map_err(|e| format!("创建文件失败: {}", e))?;
            std::io::copy(&mut entry, &mut outfile)
                .map_err(|e| format!("解压文件失败: {}", e))?;
        }

        std::fs::remove_file(&zip_path2)
            .map_err(|e| format!("删除zip文件失败: {}", e))?;

        Ok::<_, String>(())
    })
    .await
    .map_err(|e| format!("解压任务异常: {}", e))?
    .map_err(|e| e)?;

    if let Some(dir) = state.temp_dir.lock().await.as_ref() {
        let _ = tokio::fs::remove_dir(dir).await;
    }

    app.emit(constants::EVENT_DOWNLOAD_COMPLETE, crate::download::DownloadProgressPayload {
        file_name: DICT_ZIP_FILE.to_string(),
        total: 1,
        downloaded: 1,
        status: "completed".into(),
    }).ok();

    Ok(())
}

#[tauri::command]
pub async fn download_dict_db(app: tauri::AppHandle, use_github_mirror: bool, state: tauri::State<'_, DictDownloadState>) -> Result<(), String> {
    state.cancel_flag.store(false, Ordering::SeqCst);
    state.paused_flag.store(false, Ordering::SeqCst);

    let url = if use_github_mirror {
        format!("{}/{}", DICT_GITHUB_MIRROR, DICT_ZIP_URL)
    } else {
        DICT_ZIP_URL.to_string()
    };

    let temp_dir = std::env::temp_dir().join("youdao-fanyi-dict");
    tokio::fs::create_dir_all(&temp_dir).await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    *state.temp_dir.lock().await = Some(temp_dir.clone());

    let zip_path = temp_dir.join(DICT_ZIP_FILE);
    let dict_dir = crate::config::default_models_dir_inner().join(DICT_MODEL_DIR);

    // Check for partial file to resume
    if zip_path.exists() {
        if let Ok(meta) = tokio::fs::metadata(&zip_path).await {
            state.downloaded.store(meta.len(), Ordering::SeqCst);
        }
    }

    let result = download_loop(&app, &state, &url, &zip_path, &dict_dir).await;

    if let Err(ref e) = result {
        if e == "下载已取消" {
            let _ = tokio::fs::remove_file(&zip_path).await;
            let _ = tokio::fs::remove_dir(&temp_dir).await;
            Ok(())
        } else {
            app.emit(constants::EVENT_DOWNLOAD_ERROR, crate::download::DownloadProgressPayload {
                file_name: DICT_ZIP_FILE.to_string(),
                total: state.total.load(Ordering::SeqCst),
                downloaded: state.downloaded.load(Ordering::SeqCst),
                status: e.clone(),
            }).ok();
            result
        }
    } else {
        result
    }
}

#[tauri::command]
pub fn pause_dict_download(state: tauri::State<'_, DictDownloadState>) {
    state.paused_flag.store(true, Ordering::SeqCst);
}

#[tauri::command]
pub fn resume_dict_download(state: tauri::State<'_, DictDownloadState>) {
    state.paused_flag.store(false, Ordering::SeqCst);
    state.notify.notify_one();
}

#[tauri::command]
pub fn cancel_dict_download(state: tauri::State<'_, DictDownloadState>) {
    state.cancel_flag.store(true, Ordering::SeqCst);
    state.notify.notify_one();
}